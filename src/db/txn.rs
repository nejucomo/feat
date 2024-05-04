use anyhow::{bail, Result};
use rusqlite::{Params, Transaction};

use crate::model::{Action, Updatable};

use super::{OrmEntity, SqlKey};

#[derive(Debug)]
pub struct FeatTransaction<'conn>(Transaction<'conn>);

impl<'conn> FeatTransaction<'conn> {
    pub(super) fn new(inner: Transaction<'conn>) -> Self {
        FeatTransaction(inner)
    }

    pub fn execute_one<S, P>(&self, sql: S, params: P) -> Result<SqlKey>
    where
        S: AsRef<str>,
        P: Params,
    {
        let updated = self.0.execute(sql.as_ref(), params)?;
        if updated == 1 {
            Ok(self.0.last_insert_rowid())
        } else {
            bail!(
                "SQL postcondition error: expected a single update, found {updated} rows updated"
            );
        }
    }

    pub fn commit(self) -> Result<()> {
        self.0.commit()?;
        Ok(())
    }
}

impl<'conn> Updatable<Action> for FeatTransaction<'conn> {
    fn apply(&mut self, action: Action) -> Result<()> {
        action.insert(self).map(|_| ())
    }
}
