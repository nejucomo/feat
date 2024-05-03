use anyhow::Result;

pub trait Updatable<Action> {
    fn apply(&mut self, action: Action) -> Result<()>;
}
