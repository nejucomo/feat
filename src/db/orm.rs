use anyhow::Result;
use indoc::indoc;

use super::{FeatTransaction, SqlKey, SqlType};

pub trait Orm {
    // consumer methods:
    fn create_tables(txn: &FeatTransaction) -> Result<()> {
        Self::create_dependency_tables(txn)?;
        txn.execute_one(Self::create_table_statement(), [])?;
        Ok(())
    }

    // implementor methods:
    fn column_schema() -> Vec<(&'static str, SqlType)>;

    fn create_dependency_tables(_: &FeatTransaction) -> Result<()> {
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
                  id INTEGER PRIMARY KEY,
                  {}
                )
            "# },
            Self::table_name(),
            Self::column_schema()
                .into_iter()
                .fold(String::default(), |s, (name, sqltype)| format!(
                    "{s}\n  {name} {sqltype}"
                ))
        )
    }
}

pub trait OrmEntity: Orm {
    fn insert(&self, txn: &FeatTransaction) -> Result<SqlKey>;
}

pub trait OrmLinked: Orm {
    fn insert_linked(&self, txn: &FeatTransaction, linked: SqlKey) -> Result<SqlKey>;
}
