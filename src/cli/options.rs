mod dbpath;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use crate::cli::RunWithDb;
use crate::db::FeatDb;

pub use self::dbpath::DbPath;

/// Simple program to greet a person
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Options {
    /// The database path
    #[clap(short, long, default_value_t)]
    pub dbpath: DbPath,

    #[clap(subcommand)]
    pub command: Command,
}

impl Options {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    pub fn run(self) -> Result<()> {
        let db = FeatDb::open_or_init(&self.dbpath)?;
        self.command.run_with_db(db)
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(subcommand)]
    Task(CommandTask),
}

#[derive(Debug, Subcommand)]
pub enum CommandTask {
    New(CommandTaskNew),
}

#[derive(Debug, Args)]
pub struct CommandTaskNew {
    pub title: String,
}
