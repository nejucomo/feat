use anyhow::Result;
use log::LevelFilter;

pub(crate) fn init() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .try_init()?;
    Ok(())
}
