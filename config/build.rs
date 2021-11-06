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
use crate::permission;
use crate::executable::factory::AutoExecutableFactory;
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
pub const SOURCE_PERMISSION_FACTORY: AutoPermissionFactory = permission::factory::from_environment;
/// The method to use to find the [Permission][p]s to run the executable as
///
/// This is the target [Permission][p]s of the [Executable][eb]. As such, it is
/// perfectly safe to take this in from the user. Indeed, the user must be able
/// to execute as a requested privilege.
///
/// [p]: permission::Permission
/// [eb]: executable::Executable
pub const TARGET_PERMISSION_FACTORY: AutoPermissionFactory = permission::factory::from_commandline;

/// An array of all the [Verifier]s to invoke
///
/// We might want multiple checks to pass before running [Executable][eb]. This
/// is a list of all the checks that have to pass.
///
/// [eb]: executable::Executable
pub const VERIFICATIONS: &[Verifier] = &[
    permission::verify::succeed,
];

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
