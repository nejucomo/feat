use error_report::{Ref, Report};

use crate::{cli::options::Options, db::FeatDb};

pub fn run() -> Result<(), Report<Ref<anyhow::Error>>> {
    run_inner().map_err(Report::from)
}

fn run_inner() -> anyhow::Result<()> {
    let options = Options::parse();
    let db = FeatDb::open_or_init(options.dbpath)?;
    dbg!(db);
    Ok(())
}
