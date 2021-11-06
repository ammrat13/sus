//! Module for permission storage
//!
//! We will need to deal with [Permission]s. For instance, we'll need to find
//! the permissions the user currently has, and the permissions that they wish
//! to have.
//!
//! This module serves to aggregate permissions and have methods to construct
//! them from various sources. It's tied to the Unix permission model, where
//! each user has a user id, a group id, and a list of secondary group ids.

pub mod factory;

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
