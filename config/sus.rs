//! Configuration variables for the user-interface of SUS
//!
//! This file defines the configuration constants for the user-space `sus`
//! program. These will be compiled into the final binary. Many of them must
//! also align with other configuration files, otherwise the user-space binary
//! will not work properly.
//!
//! Make sure to edit this file in `config/`. This file is copied to the `src/`
//! directory as part of the build process. Any changes made there will be
//! ignored by `cargo build`.

#![allow(dead_code)]

/// The path to the kernel
pub const KERNEL_PATH: &str = "/usr/local/bin/sus-kernel";

/// What command line argument number to put the path of the binary to execute
/// at
pub const KERNEL_COMMANDLINE_BINARY_IDX: usize = 3;
/// What command line argument number to use for the first parameter to the
/// program, with subsequent arguments being used in order
pub const KERNEL_COMMANDLINE_ARG_START_IDX: usize = 4;

/// What command line argument number to put the UID at
pub const KERNEL_COMMANDLINE_UID_IDX: usize = 0;
/// What command line argument number to put the Primary GID at
pub const KERNEL_COMMANDLINE_PRIMARY_GID_IDX: usize = 1;
/// What command line argument number to put a comma separated list of the
/// Secondary GIDs at
pub const KERNEL_COMMANDLINE_SECONDARY_GID_IDX: usize = 2;
