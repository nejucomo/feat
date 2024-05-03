mod txn;

use std::path::Path;

use anyhow::Result;
use anyhow_std::PathAnyhow;
use rusqlite::Connection;

pub use self::txn::FeatTransaction;

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
        let mut conn = Connection::open(dbpath)?;
        migrate(&mut conn)?;
        Ok(FeatDb { conn })
    }

    pub fn transaction(&mut self) -> Result<FeatTransaction> {
        let inner = self.conn.transaction()?;
        Ok(FeatTransaction::new(inner))
    }
}

fn migrate(conn: &mut Connection) -> Result<()> {
    use include_dir::{include_dir, Dir};
    use rusqlite_migration::Migrations;

    static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/db/migrations");

    let ms = Migrations::from_directory(&MIGRATIONS_DIR).unwrap();

    log::debug!(
        "Applying db migrations to version {}...",
        ms.current_version(conn)?
    );
    ms.to_latest(conn)?;

    Ok(())
}
