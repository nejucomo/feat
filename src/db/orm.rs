mod ormtype;

use anyhow::Result;
use indoc::indoc;
use rusqlite::ToSql;

pub use self::ormtype::OrmType;

use super::{FeatTransaction, SqlKey};

pub trait Orm {
    // consumer methods:
    fn create_tables(txn: &FeatTransaction) -> Result<()> {
        Self::create_dependency_tables(txn)?;
        txn.execute_zero(Self::create_table_statement(), [])?;
        Ok(())
    }

    // implementor methods:
    fn column_schema() -> Vec<(&'static str, OrmType)>;

    fn create_dependency_tables(txn: &FeatTransaction) -> Result<()> {
        let _ = txn;
        Ok(())
    }

    fn insert_dependents(&self, txn: &FeatTransaction, self_id: SqlKey) -> Result<()> {
        let _ = (txn, self_id);
        Ok(())
    }

    // do not override:
    fn table_name() -> &'static str {
        std::any::type_name::<Self>()
    }

    fn create_table_statement() -> String {
        // TODO: foreign key constraints
        format!(
            indoc! { r#"
                CREATE TABLE IF NOT EXISTS {:?} (
                  id INTEGER PRIMARY KEY{}
                )
            "# },
            Self::table_name(),
            Self::column_schema()
                .into_iter()
                .fold(String::default(), |s, (name, sqltype)| format!(
                    "{s},\n  {name} {sqltype}"
                ))
        )
    }

    fn insert_statement(&self) -> String {
        let (param_names, placeholders): (Vec<_>, Vec<_>) = Self::column_schema()
            .into_iter()
            .enumerate()
            .map(|(ix, (name, _))| (name, format!("?{}", ix + 1)))
            .unzip();

        format!(
            "INSERT INTO {:?} (\n  {}) VALUES ({})",
            Self::table_name(),
            param_names.join(",\n  "),
            placeholders.join(", ")
        )
    }
}

pub trait OrmEntity: Orm {
    // consumer methods
    fn insert(&self, txn: &FeatTransaction) -> Result<SqlKey> {
        let self_id =
            self.with_entity_params(|params| txn.execute_one(self.insert_statement(), params))?;

        self.insert_dependents(txn, self_id)?;
        Ok(self_id)
    }

    // implementor methods:
    fn with_entity_params<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&[&dyn ToSql]) -> R;
}

pub trait OrmLinked: Orm {
    // consumer methods
    fn insert_linked(&self, txn: &FeatTransaction, linked: SqlKey) -> Result<SqlKey> {
        let self_id = self.with_entity_linked_params(linked, |params| {
            txn.execute_one(self.insert_statement(), params)
        })?;

        self.insert_dependents(txn, self_id)?;
        Ok(self_id)
    }

    // implementor methods:
    fn with_entity_linked_params<F, R>(&self, linked: SqlKey, f: F) -> R
    where
        F: FnOnce(&[&dyn ToSql]) -> R;
}
