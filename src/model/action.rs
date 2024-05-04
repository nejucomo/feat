use anyhow::Result;
use clap::{Args, Subcommand};
use name_variant::NamedVariant;
use rusqlite::params;

use crate::db::{
    FeatTransaction, Orm, OrmEntity, OrmLinked, SqlKey,
    SqlType::{self, ForeignKeyId, Text},
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
    fn column_schema() -> Vec<(&'static str, SqlType)> {
        vec![("discriminant", Text)]
    }

    fn create_dependency_tables(txn: &FeatTransaction) -> anyhow::Result<()> {
        ActionTask::create_tables(txn)
    }
}

impl OrmEntity for Action {
    fn insert(&self, txn: &FeatTransaction) -> Result<SqlKey> {
        use Action::*;

        let (param_names, placeholders): (Vec<_>, Vec<_>) = Self::column_schema()
            .into_iter()
            .enumerate()
            .map(|(ix, (name, _))| (name, format!("&{ix}")))
            .unzip();

        let stmt = format!(
            "INSERT INTO {:?} (\n  {}) VALUES ({})",
            Self::table_name(),
            param_names.join(",\n  "),
            placeholders.join(", ")
        );

        let action_id = txn.execute_one(stmt, params![self.discriminant_name()])?;

        match self {
            Task(x) => x.insert_linked(txn, action_id)?,
        };

        Ok(action_id)
    }
}

impl Orm for ActionTask {
    fn column_schema() -> Vec<(&'static str, SqlType)> {
        vec![
            ("action_id", ForeignKeyId(Action::table_name())),
            ("discriminant", Text),
        ]
    }

    fn create_dependency_tables(txn: &FeatTransaction) -> anyhow::Result<()> {
        ActionTaskSetTitle::create_tables(txn)
    }
}

impl OrmLinked for ActionTask {
    fn insert_linked(&self, txn: &FeatTransaction, action: SqlKey) -> Result<SqlKey> {
        use ActionTask::*;

        let (param_names, placeholders): (Vec<_>, Vec<_>) = Self::column_schema()
            .into_iter()
            .enumerate()
            .map(|(ix, (name, _))| (name, format!("&{ix}")))
            .unzip();

        let stmt = format!(
            "INSERT INTO {:?} (\n  {}) VALUES ({})",
            Self::table_name(),
            param_names.join(",\n  "),
            placeholders.join(", ")
        );

        let actiontask_id = txn.execute_one(stmt, params![action, self.discriminant_name()])?;

        match self {
            Create => {}
            SetTitle(x) => x.insert_linked(txn, actiontask_id).map(|_| ())?,
        };

        Ok(actiontask_id)
    }
}

impl Orm for ActionTaskSetTitle {
    fn column_schema() -> Vec<(&'static str, SqlType)> {
        vec![
            ("actiontask_id", ForeignKeyId(ActionTask::table_name())),
            ("title", Text),
        ]
    }
}

impl OrmLinked for ActionTaskSetTitle {
    fn insert_linked(&self, txn: &FeatTransaction, actiontask: SqlKey) -> Result<SqlKey> {
        let (param_names, placeholders): (Vec<_>, Vec<_>) = Self::column_schema()
            .into_iter()
            .enumerate()
            .map(|(ix, (name, _))| (name, format!("&{ix}")))
            .unzip();

        let stmt = format!(
            "INSERT INTO {:?} (\n  {}) VALUES ({})",
            Self::table_name(),
            param_names.join(",\n  "),
            placeholders.join(", ")
        );

        txn.execute_one(stmt, params![actiontask, self.title])
    }
}
