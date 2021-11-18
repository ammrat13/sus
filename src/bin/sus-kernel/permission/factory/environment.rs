//! Retrieve [Permission][p]s from the current execution environment
//!
//! This module has a function to pull the [Permission][p]s the executable was
//! run as. It pulls them from the environment surrounding the executable. Since
//! the kernel executes as Set-UID, it will pull the permissions from the Real
//! UID and Primary GID.
//!
//! [p]: super::Permission

use super::Permission;
use super::PermissionFactoryError;
use super::PermissionFactoryResult;

use nix::unistd;
use std::collections::HashSet;

/// Function to make a [Permission] by looking at the environment
///
/// It will use the functions imported by this file to obtain the Real UID, the
/// Real Primary GID, and the Secondary GIDs for the calling process.
///
/// The only error this function can raise has to do with finding the Secondary
/// GIDs. If any of the calls to [getgroups][gg] fail, this function will return
/// a [SecondaryGIDNotFound][sgnf] error. It succeeds otherwise.
///
/// [gg]: unistd::getgroups
/// [sgnf]: PermissionFactoryError::SecondaryGIDNotFound
#[allow(dead_code)]
pub fn from_environment() -> PermissionFactoryResult {
    // Get the vector of Gids
    let secondary_gids_vec: Vec<_> =
        unistd::getgroups().map_err(|_| PermissionFactoryError::SecondaryGIDNotFound)?;
    // Return the result
    // Convert the vector to a hashset
    Ok(Permission {
        uid: unistd::getuid(),
        primary_gid: unistd::getgid(),
        secondary_gids: HashSet::from_iter(secondary_gids_vec),
    })
}
