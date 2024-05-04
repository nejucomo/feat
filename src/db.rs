mod coldef;
mod orm;
mod txn;

use std::path::Path;

use anyhow::Result;
use anyhow_std::PathAnyhow;
use rusqlite::Connection;

use crate::model::{Action, Updatable};

pub use self::coldef::SqlType;
pub use self::orm::{Orm, OrmEntity, OrmLinked};
pub use self::txn::FeatTransaction;

pub type SqlKey = i64;

#[derive(Debug)]
pub struct FeatDb {
    conn: Connection,
}

impl FeatDb {
    pub fn open_or_init<P>(dbpath: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let dbpath = dbpath.as_ref();
        log::debug!("opening db {:?}", dbpath.display());
        dbpath.parent_anyhow()?.create_dir_all_anyhow()?;
        let conn = Connection::open(dbpath)?;
        let mut myself = FeatDb { conn };

        let txn = myself.transaction()?;
        Action::create_tables(&txn)?;
        txn.commit()?;

        Ok(myself)
    }

    pub fn transaction(&mut self) -> Result<FeatTransaction> {
        let inner = self.conn.transaction()?;
        Ok(FeatTransaction::new(inner))
    }
}

impl Updatable<Action> for FeatDb {
    fn apply(&mut self, action: Action) -> Result<()> {
        let mut txn = self.transaction()?;
        txn.apply(action)?;
        txn.commit()?;
        Ok(())
    }
}
