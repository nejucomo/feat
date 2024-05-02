use anyhow::Result;

use crate::cli::options::{Command, CommandTask, CommandTaskNew};
use crate::db::FeatDb;

pub trait RunWithDb {
    fn run_with_db(self, db: FeatDb) -> Result<()>;
}

impl RunWithDb for Command {
    fn run_with_db(self, db: FeatDb) -> Result<()> {
        use Command::*;

        match self {
            Task(x) => x.run_with_db(db),
        }
    }
}

impl RunWithDb for CommandTask {
    fn run_with_db(self, db: FeatDb) -> Result<()> {
        use CommandTask::*;

        match self {
            New(x) => x.run_with_db(db),
        }
    }
}

impl RunWithDb for CommandTaskNew {
    fn run_with_db(self, mut db: FeatDb) -> Result<()> {
        db.task_new(&self.title)
    }
}
