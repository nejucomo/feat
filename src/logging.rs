use anyhow::Result;
use log::LevelFilter;

#[cfg(not(test))]
pub(crate) fn init() -> Result<()> {
    init_inner(false)
}

#[cfg(test)]
pub(crate) fn test_init() -> Result<()> {
    init_inner(true)
}

fn init_inner(is_test: bool) -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .is_test(is_test)
        .try_init()?;
    Ok(())
}
