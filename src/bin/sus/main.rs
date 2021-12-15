//! User binary for SUS
//!
//! Ideally, the user would not invoke the kernel directly. Instead, they would
//! do so through this intermediate binary. It provides a `sudo`-like interface
//! for ease of use.

mod config;
mod option;

use std::process::exit;
use structopt::StructOpt;

use option::CommandLineOptions;
use option::Options;

/// The entrypoint of the binary
///
/// As is standard practice in Rust, most of the work is done by internal
/// libraries, and this function simply calls into those libraries.
/// Specifically, it parses the command line arguments and calls the kernel with
/// them, printing out errors if there were any.
///
/// In this function, we need to manually pretty-print errors.
fn main() {
    // Create the options and check for errors
    let opts = match Options::parse_options_like(CommandLineOptions::from_args()) {
        Err(e) => {
            println!("Error: {}", e);
            exit(101);
        }
        Ok(o) => o,
    };
    // Execute and print any errors
    let res = opts.execute();
    if let Err(e) = res {
        println!("Error: {}", e);
        exit(101);
    }

    // We should never be able to reach here
    // The execution should happen before
    println!("Successfully failed");
}
