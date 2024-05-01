use std::{fmt::Display, path::PathBuf, str::FromStr};

#[derive(Clone, Debug)]
pub struct DbPath(PathBuf);

impl Default for DbPath {
    fn default() -> Self {
        DbPath(
            dirs::data_local_dir()
                .unwrap()
                .join(env!("CARGO_PKG_NAME"))
                .join("db.sqlite"),
        )
    }
}

impl FromStr for DbPath {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(DbPath)
    }
}

impl Display for DbPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.display().fmt(f)
    }
}
