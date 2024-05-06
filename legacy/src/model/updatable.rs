use anyhow::Result;

pub trait Updatable<Action> {
    type Output;

    fn apply(&mut self, action: Action) -> Result<Self::Output>;
}
