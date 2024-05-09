mod mem;
mod task;

use std::fmt::Debug;

use crate::{action, Unique, Updatable};

pub use self::mem::MemState;
pub use self::task::Task;

pub trait State:
    Debug
    + Updatable<action::task::Create, Output = Unique>
    + Updatable<action::task::SetTitle, Output = ()>
{
    /// Get a specific task
    fn get_task<'s>(&'s self, id: &Unique) -> Option<&'s Task>;

    /// Iterate over all tasks
    fn all_tasks(&self) -> Self::AllTasks<'_>;

    /// The iterator for [Self::all_tasks]
    type AllTasks<'s>: Iterator<Item = (&'s Unique, &'s Task)>
    where
        Self: 's;
}
