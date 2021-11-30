//! Module for handling the options the user passes
//!
//! The user can pass various [CommandLineOptions] to the `sus` binary to tell
//! it what command to execute. These [CommandLineOptions] need to further be
//! parsed to map it to the arguments the `sus-kernel` needs to take. This
//! module houses all of that functionality.

pub mod commandline;
pub use commandline::CommandLineOptions;
