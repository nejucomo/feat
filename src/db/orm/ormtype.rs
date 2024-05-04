use std::fmt::Display;

use rusqlite::types::Type;

#[derive(Debug)]
pub enum OrmType {
    Integer,
    Text,
    /// Only supports foreign key to an integer columdn `id` on the given table:
    ForeignKeyId(&'static str),
}

impl Display for OrmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OrmType::*;

        match self {
            Integer => Type::Integer.fmt(f),
            Text => Type::Text.fmt(f),
            ForeignKeyId(foreign_table) => write!(
                f,
                "{} REFERENCES {:?} (id) ON DELETE CASCADE",
                Type::Integer,
                foreign_table
            ),
        }
    }
}
