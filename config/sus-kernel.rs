//! Configuration variables for the SUS Kernel
//!
//! This file contains some configuration variables for the SUS kernel. It
//! defines constants that are to be compiled into the final binary.
//!
//! Make sure to edit this file in `config/`. This file is copied to the `src/`
//! directory as part of the build process. Any changes made there will be
//! ignored by `cargo build`.

#![allow(dead_code)]

use crate::executable;
use crate::executable::factory::AutoExecutableFactory;
use crate::executable::run::Runner;
use crate::log;
use crate::log::Logger;
use crate::permission;
use crate::permission::factory::AutoPermissionFactory;
use crate::permission::verify::Verifier;

/// The method to use to find the [Executable][eb] to run
///
/// [eb]: executable::Executable
pub const EXECUTABLE_FACTORY: AutoExecutableFactory = executable::factory::from_commandline;

/// The method to use to find the current [Permission][p]s the user has
///
/// Note that this is the current [Permission][p]s this binary is executing
/// under. For this reason, the user should not be trusted to provide this. It
/// should be obtained by other means, like looking at the Real UID, Real
/// Primary GID, and Secondary Groups.
///
/// [p]: permission::Permission
pub const CURRENT_PERMISSION_FACTORY: AutoPermissionFactory = permission::factory::from_environment;
/// The method to use to find the [Permission][p]s to run the executable as
///
/// This is the target [Permission][p]s of the [Executable][eb]. As such, it is
/// perfectly safe to take this in from the user. Indeed, the user must be able
/// to execute as a requested privilege.
///
/// [p]: permission::Permission
/// [eb]: executable::Executable
pub const REQUESTED_PERMISSION_FACTORY: AutoPermissionFactory =
    permission::factory::from_commandline;

/// An array of all the [Verifier]s to invoke
///
/// We might want multiple checks to pass before running [Executable][eb]. This
/// is a list of all the checks that have to pass.
///
/// Note that *all* the checks have to pass for the [Executable][eb] to be run.
/// Effectively, these checks are `AND`ed together. As a corollary, if this list
/// is empty, the [Executable][eb] will be run unconditionally.
///
/// [eb]: executable::Executable
pub const VERIFIERS: &[Verifier] = &[];

/// The method to run the [Executable][eb] created
///
/// [eb]: executable::Executable
pub const RUNNER: Runner = executable::run::exec;

/// How to log incoming [Request][rq]s
///
/// For administrative purposes, it might be useful to log what [Request][rq]s
/// people make to this binary. This is the function that is called for logging.
///
/// [rq]: crate::request::Request
pub const LOGGER: Logger = log::to_file;

/// The path to log to
///
/// The [log::to_file] logger uses this path to determine where to log *all* the
/// incoming [Request][rq]s, both successful and failed. As such, this log file
/// can grow very quickly and should be rotated regularly, say with `logrotate`.
/// This path is hard-coded into the binary and cannot be changed at runtime.
///
/// [rq]: crate::request::Request
pub const LOG_FILE_PATH: &str = "/dev/stdout";

/// The format of the log message on success
///
/// The logging functionality of this crate allows us to configure the messages
/// that are written on success and failure. This configuration parameter
/// configures the success message.
///
/// Note that this is a macro instead of a hard string literal. This is so that
/// formatting string still works. The format string literal has to be in the
/// code literally or as a macro. Thus, this solution.
#[macro_export]
macro_rules! CONFIG_LOG_SUCCESS_MSG {
    () => {
        "{tstamp_sign}{tstamp_secs}.{tstamp_nanos}\n"
    };
}
/// The format of the log message on failure
///
/// The logging functionality of this crate allows us to configure the messages
/// that are written on success and failure. This configuration parameter
/// configures the failure message.
///
/// Note that this is a macro instead of a hard string literal. This is so that
/// formatting string still works. The format string literal has to be in the
/// code literally or as a macro. Thus, this solution.
#[macro_export]
macro_rules! CONFIG_LOG_FAILURE_MSG {
    () => {
        "{tstamp_sign}{tstamp_secs}.{tstamp_nanos}\n"
    };
}

/// What command line argument number to look for for the path of the binary to
/// execute
///
/// Used by [executable::factory::from_commandline]
pub const EXECUTABLE_COMMANDLINE_PATH_IDX: usize = 4;
/// What command line argument number to use as the first parameter to the
/// program, with subsequent arguments being used in order
///
/// Used by [executable::factory::from_commandline]
pub const EXECUTABLE_COMMANDLINE_ARG_START_IDX: usize = 5;

/// What command line argument number to look at for the UID
///
/// Used by [permission::factory::from_commandline]
pub const PERMISSION_COMMANDLINE_UID_IDX: usize = 1;
/// What command line argument number to look at for the Primary GID
///
/// Used by [permission::factory::from_commandline]
pub const PERMISSION_COMMANDLINE_PRIMARY_GID_IDX: usize = 2;
/// What command line argument number to look at for a comma separated list of
/// the Secondary GIDs.
///
/// Used by [permission::factory::from_commandline]
pub const PERMISSION_COMMANDLINE_SECONDARY_GID_IDX: usize = 3;
