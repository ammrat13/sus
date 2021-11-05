//! Configuration variables for SUS
//!
//! This file contains some configuration variables for the SUS application. It
//! defines constants that are to be compiled into the final binary.
//!
//! Make sure to edit this file in `config/build.rs`. This file is copied to the
//! `src/` directory as part of the build process. Any changes made there will
//! be ignored by `cargo build`.

/// What command line argument number to look for for the path of the binary to
/// execute
pub const EXECUTABLE_COMMANDLINE_PATH_IDX: usize = 0;
/// What command line argument number to use as the first parameter to the
/// program, with subsequent arguments being used in order
pub const EXECUTABLE_COMMANDLINE_ARG_START_IDX: usize = 1;
