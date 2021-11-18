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
pub mod verify;

use nix::unistd::{Gid, Uid};
use std::collections::HashSet;

/// Structure representing a permission set
///
/// This structure can be used to record the permissions the user currently has,
/// or express the set of permissions they desire. It's tied intimately with the
/// Unix permission model.
#[derive(Debug, Clone)]
pub struct Permission {
    /// The user id
    pub uid: Uid,
    /// The primary group id
    pub primary_gid: Gid,
    /// A set of secondary group ids, which may or may not contain the primary
    /// group id itself
    pub secondary_gids: HashSet<Gid>,
}
