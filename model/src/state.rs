mod task;

use std::collections::BTreeMap;

use anyhow::{anyhow, Result};

use crate::{action, Unique, Updatable};

pub use self::task::Task;

#[derive(Debug, Default)]
pub struct State {
    tasks: BTreeMap<Unique, Task>,
}

impl State {
    pub fn tasks(&self) -> &BTreeMap<Unique, Task> {
        &self.tasks
    }

    fn task_create(&mut self) -> Result<Unique> {
        let id = Unique::generate();
        assert!(self.tasks.insert(id.clone(), Task::default()).is_none());
        Ok(id)
    }

    fn task_set_title(&mut self, id: Unique, title: String) -> Result<()> {
        let idref = &id;
        let task = self
            .tasks
            .get_mut(idref)
            .ok_or_else(|| anyhow!("missing task {idref:?}"))?;
        task.set_title(title);
        Ok(())
    }
}

impl Updatable<action::task::Create> for State {
    type Output = Unique;

    fn update(&mut self, _: action::task::Create) -> Result<Unique> {
        self.task_create()
    }
}

impl Updatable<action::task::SetTitle> for State {
    type Output = ();

    fn update(&mut self, action::task::SetTitle(id, title): action::task::SetTitle) -> Result<()> {
        self.task_set_title(id, title)
    }
}
