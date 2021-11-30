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
use std::fmt;
use std::fmt::{Display, Formatter};

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

impl Display for Permission {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Handle the Secondary GIDs
        // Frist, collect them and convert all of them to integers
        // Then, sort the list
        // Finally, convert all of them to strings
        let sgid_vec: Vec<_> = {
            // Step 1
            let mut sgid_int_vec: Vec<_> = self.secondary_gids.iter().map(|g| g.as_raw()).collect();
            // Step 2
            // Must be done separately because no way to sort in place
            sgid_int_vec.sort_unstable();
            // Step 3
            // Return
            sgid_int_vec.into_iter().map(|g| g.to_string()).collect()
        };
        // Write everything
        write!(
            f,
            "uid={} gid={} groups={}",
            self.uid.as_raw(),
            self.primary_gid.as_raw(),
            sgid_vec.join(","),
        )?;
        // Return
        Ok(())
    }
}
