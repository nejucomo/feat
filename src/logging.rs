use anyhow::Result;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

pub(crate) fn init() -> Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    Ok(())
}
