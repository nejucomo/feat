use clap::{Args, Parser, Subcommand};

/// Simple program to greet a person
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Options {
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
