mod config;
mod option;

use std::error::Error;
use structopt::StructOpt;

use option::CommandLineOptions;
use option::Options;

fn main() -> Result<(), Box<dyn Error>> {
    Options::parse_options_like(CommandLineOptions::from_args())?.execute()?;
    Ok(())
}
