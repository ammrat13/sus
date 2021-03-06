//! Module containing methods to execute [Executable]s
//!
//! [Executable]s have to be run somehow. This module defines methods for
//! invoking them, as well as auxilary types related to those functions.

pub mod exec;
pub use exec::exec;

use super::Executable;
use crate::permission::Permission;

use nix::errno::Errno;
use std::convert::Infallible;

/// Type for functions that run [Executable]s
///
/// These functions take in the [Permission]s the user wishes to execute as, and
/// runs the [Executable] with those permissions. Ideally, this function never
/// returns. If it returns, it always returns a [Result::Err].
pub type Runner = fn(&Permission, &Executable) -> RunResult;
/// Abstract supertype of [Runner]
///
/// For testing purposes, we might want to have [Runner]s signal other parts of
/// the code. This trait allows for that. Since it's a `dyn` type, we can't
/// create variables with it. However, it will work for automatically generated
/// closures.
pub type AbstractRunner = dyn FnMut(&Permission, &Executable) -> RunResult;

/// Convinience type for the result of a [Runner]
///
/// For convinience, this type aliases to the expected return type. Ideally,
/// [Runner]s never return. If they return, they always return in error. As
/// such, the [Ok](Result::Ok) branch of this type is [Infallible] and cannot be
/// explicitly constructed.
pub type RunResult = Result<Infallible, RunError>;

/// Error for [Runner]s
///
/// Ideally, [Runner]s never return. If they return, they always return in
/// error. This type enumerates the types of errors that can occur. Essentially,
/// any one of the system calls required can fail.
///
/// If any one of these errors are returned, the application should be taken to
/// be in an indeterminate state. There is no easy way to roll back a system
/// call. As such, the appropriate course of action is to terminate the
/// application as soon as possible.
#[derive(Debug)]
pub enum RunError {
    /// An error occurred when setting the UID of the process
    SetUID { errno: Errno },
    /// An error occurred when setting the Primary GID of the process
    SetPrimaryGID { errno: Errno },
    /// An error occurred when seting the Secondary GIDs of the process
    SetSecondaryGID { errno: Errno },

    /// An error occured when attempting to change to the target binary
    Execute { errno: Errno },
}
