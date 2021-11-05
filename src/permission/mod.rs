//! Module for permission storage
//!
//! We will need to deal with [Permission]s. For instance, we'll need to find
//! the permissions the user currently has, and the permissions that they wish
//! to have.
//!
//! This module serves to aggregate permissions and have methods to construct
//! them from various sources. It's tied to the Unix permission model, where
//! each user has a user id, a group id, and a list of secondary group ids.

pub mod commandline;
pub mod iterator;
pub use commandline::from_commandline;
pub use iterator::from_iterator;

use libc::{gid_t, uid_t};
use std::collections::HashSet;

/// Structure representing a permission set
///
/// This structure can be used to record the permissions the user currently has,
/// or express the set of permissions they desire. It's tied intimately with the
/// Unix permission model.
#[derive(Debug, Clone)]
pub struct Permission {
    /// The user id
    uid: uid_t,
    /// The primary group id
    primary_gid: gid_t,
    /// A set of secondary group ids, which may or may not contain the primary
    /// group id itself
    secondary_gids: HashSet<gid_t>,
}

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
#[allow(dead_code)]
pub type AutoPermissionFactory = fn() -> PermissionFactoryResult;

/// Convinience type for the result of a [Permission] factory
///
/// Creating a [Permission] may succeed or may fail. A [Result] is thus returned
/// with the status. For convinience, this type aliases to the result.
pub type PermissionFactoryResult = Result<Permission, PermissionFactoryError>;

#[derive(Debug)]
#[allow(dead_code)]
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
