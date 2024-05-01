use std::error::Error;

use error_report::Report;

use crate::cli::options::Options;

pub fn run() -> Result<(), Report<impl Error>> {
    let options = Options::parse();
    dbg!(options);
    Ok::<_, Report<std::io::Error>>(())
}
