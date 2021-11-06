//! Module representing [Executable] objects
//!
//! To run an executable with privileges, we need to know what executable to run
//! and with what arguments. This module serves to collect and package that
//! information.
//!
//! At its core, this module has an [Executable] structure, which contains the
//! path to the file to execute, as well as a [Vec] of command line arguments to
//! supply.
//!
//! Additionally, the module has methods for getting the [Executable] from the
//! user. It has various functions to get it from command line arguments or from
//! iterables.

pub mod factory;
pub mod run;

use std::ffi::CString;
use std::path::PathBuf;

/// Structure representing an executable program
///
/// It holds the path of the program to be executed, as well as the comamnd line
/// arguments to pass it.
#[derive(Debug, Clone)]
pub struct Executable {
    /// The path to the executable
    path: PathBuf,
    /// The command line arguments to pass to the executable
    args: Vec<CString>,
}
