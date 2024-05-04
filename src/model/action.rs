use anyhow::Result;
use clap::{Args, Subcommand};
use name_variant::NamedVariant;
use rusqlite::{params, ToSql};

use crate::db::{
    FeatTransaction, Orm, OrmEntity, OrmLinked,
    OrmType::{self, ForeignKeyId, Text},
    SqlKey,
};
use crate::DiscriminantName;

#[derive(Debug, Subcommand, NamedVariant)]
pub enum Action {
    #[clap(subcommand)]
    Task(ActionTask),
}
disc_from_variant_name!(Action);

#[derive(Debug, Subcommand, NamedVariant)]
pub enum ActionTask {
    Create,
    SetTitle(ActionTaskSetTitle),
}
disc_from_variant_name!(ActionTask);

#[derive(Debug, Args)]
pub struct ActionTaskSetTitle {
    pub title: String,
}

impl Orm for Action {
    fn column_schema() -> Vec<(&'static str, OrmType)> {
        vec![("discriminant", Text)]
    }

    fn create_dependency_tables(txn: &FeatTransaction) -> anyhow::Result<()> {
        ActionTask::create_tables(txn)
    }

    fn insert_dependents(&self, txn: &FeatTransaction, self_id: SqlKey) -> Result<()> {
        use Action::*;

        match self {
            Task(x) => x.insert_linked(txn, self_id).map(|_| ()),
        }
    }
}

impl OrmEntity for Action {
    fn with_entity_params<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&[&dyn ToSql]) -> R,
    {
        f(params![self.discriminant_name()])
    }
}

impl Orm for ActionTask {
    fn column_schema() -> Vec<(&'static str, OrmType)> {
        vec![
            ("action_id", ForeignKeyId(Action::table_name())),
            ("discriminant", Text),
        ]
    }

    fn create_dependency_tables(txn: &FeatTransaction) -> anyhow::Result<()> {
        ActionTaskSetTitle::create_tables(txn)
    }

    fn insert_dependents(&self, txn: &FeatTransaction, self_id: SqlKey) -> Result<()> {
        use ActionTask::*;

        match self {
            Create => {}
            SetTitle(x) => x.insert_linked(txn, self_id).map(|_| ())?,
        };

        Ok(())
    }
}

impl OrmLinked for ActionTask {
    fn with_entity_linked_params<F, R>(&self, action_id: SqlKey, f: F) -> R
    where
        F: FnOnce(&[&dyn ToSql]) -> R,
    {
        f(params![action_id, self.discriminant_name()])
    }
}

impl Orm for ActionTaskSetTitle {
    fn column_schema() -> Vec<(&'static str, OrmType)> {
        vec![
            ("actiontask_id", ForeignKeyId(ActionTask::table_name())),
            ("title", Text),
        ]
    }
}

impl OrmLinked for ActionTaskSetTitle {
    fn with_entity_linked_params<F, R>(&self, actiontask_id: SqlKey, f: F) -> R
    where
        F: FnOnce(&[&dyn ToSql]) -> R,
    {
        f(params![actiontask_id, self.title])
    }
}
