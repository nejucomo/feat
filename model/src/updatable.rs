use anyhow::Result;

pub trait Updatable<T> {
    type Output;

    fn update(&mut self, msg: T) -> Result<Self::Output>;
}
