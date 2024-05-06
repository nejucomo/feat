use anyhow::Result;

use crate::{
    db::FeatDb,
    model::{
        Action::Task,
        ActionTask::{Create, SetTitle},
        ActionTaskSetTitle, Updatable,
    },
};

#[test]
fn new_task_with_title() -> Result<()> {
    crate::logging::test_init()?;

    let testdir = tempfile::Builder::new()
        .prefix(&format!("{}.", env!("CARGO_PKG_NAME")))
        .suffix(".testdata")
        .tempdir()?;

    let dbpath = testdir.as_ref().join("db.sqlite");

    let mut db = FeatDb::open_or_init(&dbpath)?;

    let k1 = db.apply(Task(Create))?;
    assert_eq!(k1, 1);
    let k2 = db.apply(Task(SetTitle(ActionTaskSetTitle {
        title: "hello world".to_string(),
    })))?;
    assert_eq!(k2, 2);

    Ok(())
}
