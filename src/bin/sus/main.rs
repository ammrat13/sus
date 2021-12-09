//! User binary for SUS
//!
//! Ideally, the user would not invoke the kernel directly. Instead, they would
//! do so through this intermediate binary. It provides a `sudo`-like interface
//! for ease of use.

mod config;
mod option;

use std::error::Error;
use structopt::StructOpt;

use option::CommandLineOptions;
use option::Options;

/// The entrypoint of the binary
///
/// As is standard practice in Rust, most of the work is done by internal
/// libraries, and this function simply calls into those libraries.
/// Specifically, it parses the command line arguments and calls the kernel with
/// them, printing out errors if there were any.
fn main() -> Result<(), Box<dyn Error>> {
    Options::parse_options_like(CommandLineOptions::from_args())?.execute()?;
    Ok(())
}
