//! Module containing a method that `execve`s an [Executable]
//!
//! Most commonly, the user will want to run the binary. This module provides a
//! way to do that. It will either change to the given [Executable], or it will
//! fail to do so and return to this application.

use super::Executable;
use super::RunError;
use super::RunResult;
use crate::permission::Permission;

use nix::unistd;
use std::ffi::CString;

/// Function that calls `execve` to run the [Executable] given
///
/// It will set the permissions to those given in the first parameter, then
/// execute the new binary. It only returns if any of those steps failed.
pub fn exec(perm: &Permission, execable: &Executable) -> RunResult {
    // Set the secondary groups
    // First, ensure that the primary group is part of the list of secondary
    //  groups. It is not guaranteed to be.
    // Second, convert it to a vector. We need to do this to call setgroups
    {
        // Add the Primary GID to the Secondary GIDs
        // Note that this clones the set
        let mut new_sgid_set = perm.secondary_gids.clone();
        new_sgid_set.insert(perm.primary_gid);

        // Convert to a vector and sort it
        let mut new_sgid_vec = Vec::from_iter(new_sgid_set.into_iter());
        new_sgid_vec.sort_by_key(|g| g.as_raw());

        // Do the call
        // Fail out on error
        unistd::setgroups(&new_sgid_vec).map_err(|en| RunError::SetSecondaryGID { errno: en })?;
    };

    // Set the primary group
    // Fail out on error
    unistd::setgid(perm.primary_gid).map_err(|en| RunError::SetPrimaryGID { errno: en })?;

    // Set the user
    // Fail out on error
    unistd::setuid(perm.uid).map_err(|en| RunError::SetUID { errno: en })?;
    
    // Execute
    unistd::execve::<CString, CString>(&execable.path, &execable.args, &[])
        .map_err(|en| RunError::Execute { errno: en })
}
