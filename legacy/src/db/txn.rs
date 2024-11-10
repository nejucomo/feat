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

    pub fn execute_zero<S, P>(&self, sql: S, params: P) -> Result<SqlKey>
    where
        S: AsRef<str>,
        P: Params,
    {
        self.execute_n(0, sql, params)
    }

    pub fn execute_one<S, P>(&self, sql: S, params: P) -> Result<SqlKey>
    where
        S: AsRef<str>,
        P: Params,
    {
        self.execute_n(1, sql, params)
    }

    fn execute_n<S, P>(&self, expected_rows: usize, sql: S, params: P) -> Result<SqlKey>
    where
        S: AsRef<str>,
        P: Params,
    {
        let sql = sql.as_ref();
        log::debug!("Executing SQL:\n  {sql}");
        let updated = self.0.execute(sql, params)?;
        if updated == expected_rows {
            Ok(self.0.last_insert_rowid())
        } else {
            bail!(
                "SQL postcondition error: expected to update {expected_rows} rows, but actually updated {updated} rows"
            );
        }
    }

    pub fn commit(self) -> Result<()> {
        self.0.commit()?;
        Ok(())
    }
}

impl Updatable<Action> for FeatTransaction<'_> {
    type Output = SqlKey;

    fn apply(&mut self, action: Action) -> Result<SqlKey> {
        action.insert(self)
    }
}
