use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use rand::rngs::StdRng;

use crate::action::task::{Create as TaskCreate, SetTitle as TaskSetTitle};
use crate::{Unique, Updatable};

use super::{State, Task};

#[derive(Debug)]
pub struct MemState {
    rng: StdRng,
    tasks: BTreeMap<Unique, Task>,
}

impl MemState {
    pub fn from_seed(seed: u64) -> Self {
        use rand::SeedableRng;

        MemState {
            rng: StdRng::seed_from_u64(seed),
            tasks: BTreeMap::default(),
        }
    }
}

impl State for MemState {
    type AllTasks<'s> = std::collections::btree_map::Iter<'s, Unique, Task>;

    fn all_tasks(&self) -> Self::AllTasks<'_> {
        self.tasks.iter()
    }

    fn get_task<'s>(&'s self, id: &Unique) -> Option<&'s Task> {
        self.tasks.get(id)
    }
}

impl Updatable<TaskCreate> for MemState {
    type Output = Unique;

    fn update(&mut self, _: TaskCreate) -> Result<Unique> {
        let id = Unique::generate(&mut self.rng);
        assert!(self.tasks.insert(id.clone(), Task::default()).is_none());
        Ok(id)
    }
}

impl Updatable<TaskSetTitle> for MemState {
    type Output = ();

    fn update(&mut self, TaskSetTitle(id, title): TaskSetTitle) -> Result<()> {
        let idref = &id;
        let task = self
            .tasks
            .get_mut(idref)
            .ok_or_else(|| anyhow!("missing task {idref:?}"))?;
        task.set_title(title);
        Ok(())
    }
}
