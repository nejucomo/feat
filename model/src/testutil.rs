//! Testing utilities for any [State] impl

use crate::{state::State, Updatable};
use anyhow::Result;

pub use test_case::test_case;

/// Create a [State] via `mkstate`, transform it with `msg`, then check for correct outcomes via `pred`
///
/// Users typically use the [def_state_impl_tests] macro which defines a standard set of test cases which delegate to this work function.
pub fn test_state_impl<S, M, T, P>(mkstate: M, msg: T, pred: P) -> Result<()>
where
    S: State + Updatable<T>,
    M: FnOnce() -> Result<S>,
    P: FnOnce(S, <S as Updatable<T>>::Output) -> Result<()>,
{
    let mut s = mkstate()?;
    let output = s.update(msg)?;
    pred(s, output)?;
    Ok(())
}

#[rustfmt::skip]
#[macro_export]
macro_rules! def_state_impl_tests {
    ( $basename:ident, $mk_state:expr ) => {
        use $crate::{action, State, Updatable};

        #[test_case::test_case(
                $mk_state,
                action::task::Create,
                |s, _id| {
                     assert_eq!(1, s.all_tasks().count());
                     Ok(())
                }
                ; "task_create"
            )]
        fn $basename<S, M, T, P>(mkstate: M, msg: T, pred: P) -> anyhow::Result<()>
        where
            S: State + Updatable<T>,
            M: FnOnce() -> anyhow::Result<S>,
            P: FnOnce(S, <S as $crate::Updatable<T>>::Output) -> anyhow::Result<()>,
        {
            $crate::testutil::test_state_impl(mkstate, msg, pred)
        }
    };
}
