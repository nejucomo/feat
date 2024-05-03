use anyhow::Result;

use crate::{
    db::{FeatDb, FeatTransaction},
    model::Updatable,
};

use super::{Command, CommandTask};

impl Updatable<Command> for FeatDb {
    fn apply(&mut self, action: Command) -> Result<()> {
        let mut txn = self.transaction()?;
        txn.apply(action)?;
        txn.commit()?;
        Ok(())
    }
}

impl<'conn> Updatable<Command> for FeatTransaction<'conn> {
    fn apply(&mut self, action: Command) -> Result<()> {
        use Command::*;

        match action {
            Task(x) => self.apply(x),
        }
    }
}

impl<'conn> Updatable<CommandTask> for FeatTransaction<'conn> {
    fn apply(&mut self, action: CommandTask) -> Result<()> {
        todo!("FIXME: let's rip out the cli and wire it directly into action.")
    }
}
