use anyhow::Result;

pub trait Updatable<T> {
    type Output;

    fn update(&mut self, msg: T) -> Result<Self::Output>;
}

impl<S, A, B> Updatable<(A, B)> for S
where
    S: Updatable<A> + Updatable<B>,
{
    type Output = (<S as Updatable<A>>::Output, <S as Updatable<B>>::Output);

    fn update(&mut self, (a, b): (A, B)) -> Result<Self::Output> {
        let a_out = self.update(a)?;
        let b_out = self.update(b)?;
        Ok((a_out, b_out))
    }
}

impl<S, A, B, C> Updatable<(A, B, C)> for S
where
    S: Updatable<A> + Updatable<B> + Updatable<C>,
{
    type Output = (
        <S as Updatable<A>>::Output,
        <S as Updatable<B>>::Output,
        <S as Updatable<C>>::Output,
    );

    fn update(&mut self, (a, b, c): (A, B, C)) -> Result<Self::Output> {
        let (a_out, (b_out, c_out)) = self.update((a, (b, c)))?;
        Ok((a_out, b_out, c_out))
    }
}
