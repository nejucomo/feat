use std::path::Path;

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
    crate::logging::init()?;

    let dbpath = tempfile::Builder::new()
        .prefix("testdata.")
        .tempdir_in(Path::new(env!("CARGO_MANIFEST_DIR")).join("target"))?
        .into_path()
        .join("db.sqlite");

    let mut db = FeatDb::open_or_init(&dbpath)?;

    let k1 = db.apply(Task(Create))?;
    assert_eq!(k1, 1);
    let k2 = db.apply(Task(SetTitle(ActionTaskSetTitle {
        title: "hello world".to_string(),
    })))?;
    assert_eq!(k2, 2);

    Ok(())
}
