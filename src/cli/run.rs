use anyhow::{Error, Result};
use error_report::{Ref, Report};

use crate::cli::options::Options;

pub fn run() -> Result<(), Report<Ref<Error>>> {
    run_inner().map_err(Report::from)
}

fn run_inner() -> Result<()> {
    let options = Options::parse();
    #[cfg(not(test))]
    crate::logging::init()?;
    log::debug!("parsed options {:?}", &options);
    options.run()
}
