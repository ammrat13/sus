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

use libc::{gid_t, uid_t};
use std::os::raw::c_int;
use std::ptr;

use std::collections::HashSet;

extern "C" {
    /// See `man 2 getuid` or [Man7]
    ///
    /// [Man7]: https://man7.org/linux/man-pages/man2/getuid.2.html
    fn getuid() -> uid_t;

    /// See `man 2 getgid` or [Man7]
    ///
    /// [Man7]: https://man7.org/linux/man-pages/man2/getgid.2.html
    fn getgid() -> gid_t;

    /// See `man 2 getgroups` or [Man7]
    ///
    /// [Man7]: https://man7.org/linux/man-pages/man2/getgroups.2.html
    fn getgroups(size: c_int, list: *mut gid_t) -> c_int;
}

/// Function to make a [Permission] by looking at the environment
///
/// It will use the functions imported by this file to obtain the Real UID, the
/// Real Primary GID, and the Secondary GIDs for the calling process.
///
/// Most of this functions work has to do with the [getgroups] function. It can
/// fail, and a lot of work needs to be done to deal with the failures. It also
/// returns a list into a parameter, and that takes some work. The [getuid] and
/// [getgid] functions are much easier to work with - just a single call.
///
/// The only error this function can raise has to do with finding the Secondary
/// GIDs. If any of the calls to [getgroups] fail, this function will return a
/// [SecondaryGIDNotFound][sgnf] error. It succeeds otherwise.
///
/// [sgnf]: PermissionFactoryError::SecondaryGIDNotFound
pub fn from_environment() -> PermissionFactoryResult {
    // Get the number of secondary groups
    // Check for and fail on error
    let res_number = unsafe { getgroups(0, ptr::null_mut()) };
    if res_number < 0 {
        return Err(PermissionFactoryError::SecondaryGIDNotFound);
    }
    // Convert the result to a usize, and fail out if that doesn't work
    let num_secondary_gids: usize = res_number
        .try_into()
        .map_err(|_| PermissionFactoryError::SecondaryGIDNotFound)?;

    // Create a vector for the secondary groups
    let mut secondary_gids: Vec<gid_t> = Vec::with_capacity(num_secondary_gids);
    // Populate the secondary groups
    // Check for and fail on error
    let res_groups = unsafe { getgroups(res_number, secondary_gids.as_mut_ptr()) };
    if res_groups != res_number {
        return Err(PermissionFactoryError::SecondaryGIDNotFound);
    }
    // Set the length of the vector
    // It isn't set automatically
    unsafe { secondary_gids.set_len(num_secondary_gids) };

    Ok(Permission {
        uid: unsafe { getuid() },
        primary_gid: unsafe { getgid() },
        secondary_gids: HashSet::from_iter(secondary_gids.into_iter()),
    })
}
