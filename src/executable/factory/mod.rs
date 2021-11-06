//! Module containing methods of creating [Executable]s
//!
//! There are many ways to create an [Executable]. For instance, we might supply
//! a generic iterator an initialize it that way, or we might take arguments
//! from the command line. This module provides methods for all the ways to
//! create an [Executable].
//!
//! Additionally, this module defines auxilary types relating to the creation of
//! [Executable]s, including the result to be returned, and any errors.

pub mod commandline;
pub mod iterator;
pub use commandline::from_commandline;
pub use iterator::from_iterator;

use super::Executable;

/// Type for (automatic) [Executable] factories
///
/// We need to be able to generate [Executable]s in different ways. We might
/// want to create one from command line arguments, or we might create one from
/// an iterable, or we might use it for stubbing.
///
/// As such, we have various functions that create [Executable]s. We term these
/// "Executable Factories." We define one of these to be "Automatic" if it takes
/// no arguments.
///
/// The [main](crate::main) function can use [AutoExecutableFactory]s during
/// runtime to create [Executable]s.
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
///
/// Notice that no arguments being found isn't an error - it just means the
/// process will be created with no arguments. As such, that possibiliy isn't
/// present here. It is possible, however, for the path to be malformed. It has
/// to be parsed to a CString, and that may fail.
#[derive(Debug)]
pub enum ExecutableFactoryError {
    /// Path could not be located
    PathNotFound,
    /// Path is not a valid C String and has a null byte in the middle
    PathMalformed { content: String },

    /// Comamnd line argument couldn't be parsed. The `position` is
    /// the zero-indexed number of the command line argument that failed, and
    /// `content` is the content of the string.
    ArgMalformed { position: usize, content: String },
}
