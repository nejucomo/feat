use anyhow::{Error, Result};
use error_report::{Ref, Report};

use crate::cli::options::Options;

pub fn run() -> Result<(), Report<Ref<Error>>> {
    run_inner().map_err(Report::from)
}

fn run_inner() -> Result<()> {
    let options = Options::parse();
    init_logging()?;
    log::debug!("parsed options {:?}", &options);
    options.run()
}

fn init_logging() -> Result<()> {
    use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    Ok(())
}
