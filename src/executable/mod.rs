//! Module representing executable objects
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
//! user. It has various functions to get it from command line arguments, from
//! iterables, or from a presupplied structure.

pub mod iterator;
pub use iterator::from_iterator;

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

/// Type for (automatic) [Executable] factories
///
/// We need to be able to generate [Executable]s in different ways. We might
/// want to create one from command line arguments, or we might create one from
/// an iterable, or we might use it for stubbing.
///
/// As such, we have various functions that create executables. We term these
/// "Executable Factories." We define one of these to be "Automatic" if it takes
/// no arguments.
///
/// The [main](crate::main) function can use [AutoExecutableFactory]s
/// during runtime to create [Executable]s.
#[allow(dead_code)]
pub type AutoExecutableFactory = fn() -> ExecutableFactoryResult;

/// Convinience type for the result of an [Executable] factory
///
/// Creating an [Executable] may succeed or may fail. A [Result] is thus
/// returned with the status. For convinience, this type aliases to the result.
pub type ExecutableFactoryResult = Result<Executable, ExecutableFactoryError>;

/// Error for [Executable] factories
///
/// When creating an [Executable], functions might run into errors with finding
/// the parameters needed. This `enum` supplies error codes for the different
/// possibilities.
#[derive(Debug)]
pub enum ExecutableFactoryError {
    /// Path could not be located
    PathNotFound,

    /// Required command line argument could not be located. The `position` is
    /// the zero-indexed number of the command line argument.
    ArgNotFound { position: usize },
    // Comamnd line argument couldn't be parsed. The `position` is
    /// the zero-indexed number of the command line argument.
    ArgMalformed { position: usize },
}
