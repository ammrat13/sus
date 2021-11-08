//! Module containing methods for creating [Permission]s
//!
//! There are multiple ways to create a [Permission]. Indeed there must be - we
//! don't want to take input from the user to get the current permissions, but
//! we need to ask the user what permissions to run as. As such, this module
//! contains all the ways to create a [Permission].
//!
//! Additionally, this module defines auxilary types relating to the creation of
//! [Permission]s, including the result to be returned, and any errors.

pub mod commandline;
pub mod environment;
pub mod iterator;
pub use commandline::from_commandline;
pub use environment::from_environment;
pub use iterator::from_iterator;

use super::Permission;

/// Type for (automatic) [Permission] factories
///
/// We need to be able to generate [Permission]s in different ways. We might
/// want to create one from command line arguments, or we might create one from
/// the environment the process is running in.
///
/// As such, we have various functions that create [Permission]s. We term these
/// "Permission Factories." We define one of these to be "Automatic" if it takes
/// no arguments.
///
/// The [main](crate::main) function can use [AutoPermissionFactory]s during
/// runtime to create [Permission]s.
pub type AutoPermissionFactory = fn() -> PermissionFactoryResult;

/// Convinience type for the result of a [Permission] factory
///
/// Creating a [Permission] may succeed or may fail. A [Result] is thus returned
/// with the status. For convinience, this type aliases to the result.
pub type PermissionFactoryResult = Result<Permission, PermissionFactoryError>;

/// Error for [Permission] factories
///
/// When creating [Permission]s, functions might run into errors with finding
/// the parameters needed. This `enum` supplies error codes for the different
/// possibilities. Any one of the components might not be found, or might not be
/// parseable.
#[derive(Debug)]
pub enum PermissionFactoryError {
    /// UID could not be located
    UIDNotFound,
    /// Primary GID could not be located
    PrimaryGIDNotFound,
    /// Secondary GID list could not be found
    SecondaryGIDNotFound,

    /// Parse error for UID, where `content` is the string we tried to parse
    UIDMalformed { content: String },
    /// Parse error for Primary GID, where `content` is the string we tried to
    /// parse
    PrimaryGIDMalformed { content: String },
    /// Parse error for a Secondary GID, where `content` is the failing string
    SecondaryGIDMalformed { content: String },
}
