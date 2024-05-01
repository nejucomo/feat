mod dbpath;

use clap::{Args, Parser, Subcommand};

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
