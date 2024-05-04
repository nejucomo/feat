mod dbpath;

use anyhow::Result;
use clap::Parser;

use crate::db::FeatDb;
use crate::model::{Action, Updatable};

pub use self::dbpath::DbPath;

/// Simple program to greet a person
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Options {
    /// The database path
    #[clap(short, long, default_value_t)]
    pub dbpath: DbPath,

    #[clap(subcommand)]
    pub command: Action,
}

impl Options {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    pub fn run(self) -> Result<()> {
        let mut db = FeatDb::open_or_init(&self.dbpath)?;
        let key = db.apply(self.command)?;
        println!("{key}");
        Ok(())
    }
}
