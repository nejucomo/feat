use std::path::Path;

use anyhow::Result;
use anyhow_std::PathAnyhow;
use rusqlite::Connection;

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
        Ok(FeatDb { conn })
    }

    pub fn task_new(&mut self, title: &str) -> Result<()> {
        let txn = self.conn.transaction()?;
        txn.execute("INSERT INTO action_task_new(title) VALUES (?1)", [title])?;
        txn.commit()?;
        Ok(())
    }
}
