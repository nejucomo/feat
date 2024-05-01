use std::path::Path;

use anyhow_std::PathAnyhow;
use rusqlite::Connection;

#[derive(Debug)]
pub struct FeatDb {
    conn: Connection,
}

impl FeatDb {
    pub fn open_or_init<P>(dbpath: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let dbpath = dbpath.as_ref();
        dbpath.parent_anyhow()?.create_dir_all_anyhow()?;
        let conn = Connection::open(dbpath)?;
        let feat = FeatDb { conn };
        let _ = &feat.conn;
        Ok(feat)
    }
}
