use anyhow::Result;
use rusqlite::{Params, Transaction};

use crate::model::{action, Updatable};

#[derive(Debug)]
pub struct FeatTransaction<'conn>(Transaction<'conn>);

impl<'conn> FeatTransaction<'conn> {
    pub(super) fn new(inner: Transaction<'conn>) -> Self {
        FeatTransaction(inner)
    }

    pub fn commit(self) -> Result<()> {
        self.0.commit()?;
        Ok(())
    }

    pub fn insert_into_orm_table<T, P>(&mut self, names: &[&str], params: P) -> Result<()>
    where
        P: Params,
    {
        let updated = self.0.execute(
            &format!(
                "INSERT INTO {:?} ({}) VALUES ({})",
                std::any::type_name::<T>(),
                names.join(", "),
                (0..names.len())
                    .map(|ix| format!("?{}", ix + 1))
                    .intersperse(", ".to_string())
                    .collect::<String>(),
            ),
            params,
        )?;
        Ok(())
    }
}

impl<'conn> Updatable<action::task::Create> for FeatTransaction<'conn> {
    fn apply(&mut self, action: action::task::Create) -> Result<()> {
        self.insert_into_orm_table::<action::task::Create, _>(&[], [])?;
        Ok(())
    }
}

//     pub fn task_new(&mut self, title: &str) -> Result<()> {
//         let txn = self.conn.transaction()?;
//         txn.commit()?;
//         Ok(())
//     }
// }
