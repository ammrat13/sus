//! Configuration variables for SUS
//!
//! This file contains some configuration variables for the SUS application. It
//! defines constants that are to be compiled into the final binary.
//!
//! Make sure to edit this file in `config/build.rs`. This file is copied to the
//! `src/` directory as part of the build process. Any changes made there will
//! be ignored by `cargo build`.

#![allow(dead_code)]

use crate::executable;
use crate::executable::AutoExecutableFactory;

/// The method to use to find the [Executable][eb] to run
///
/// [eb]: executable::Executable
pub const EXECUTABLE_FACTORY: AutoExecutableFactory = executable::from_commandline;

/// What command line argument number to look for for the path of the binary to
/// execute
///
/// Used by [executable::from_commandline].
pub const EXECUTABLE_COMMANDLINE_PATH_IDX: usize = 0;
/// What command line argument number to use as the first parameter to the
/// program, with subsequent arguments being used in order
///
/// Used by [executable::from_commandline].
pub const EXECUTABLE_COMMANDLINE_ARG_START_IDX: usize = 1;
