use std::error::Error;

use error_report::Report;

pub fn run() -> Result<(), Report<impl Error>> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "whoop!"))?;
    Ok(())
}
