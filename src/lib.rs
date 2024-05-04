#![feature(iter_intersperse)]

#[macro_use]
mod discriminantname;

pub mod cli;
pub mod db;
pub mod model;

pub use self::discriminantname::DiscriminantName;
