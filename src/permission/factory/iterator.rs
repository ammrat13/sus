//! Parse [Permission]s from an [Iterator]
//!
//! This module implements a method to parse a [Permission] from an [Iterator].
//! This functionality can be useful when parsing command line arguments, since
//! those can be turned into an [Iterator] easily. It also works on generic
//! [Vec]s for the same reason, which is useful for testing.

use super::Permission;
use super::PermissionFactoryError;
use super::PermissionFactoryResult;

use nix::unistd::{Gid, Uid};
use std::collections::HashSet;

/// Function to make a [Permission] from an [Iterator]
///
/// This function will consume the [Iterator] it's given and parse particular
/// indicies to determine the UID, Primary GID, and Secondary GIDs. The indicies
/// it looks at are passed as the parameters `uid_idx`, `gid1_idx`, and
/// `gid2_idx` respectively.
///
/// For parsing, it expects the UID and Primary GID to be base-10 strings. It
/// expects the list of Secondary GIDs to be a comma separated list of base-10
/// strings, with no additional spaces inserted.
///
/// The function will return the created [Permission], or a
/// [PermissionFactoryError] if it was not able to find and parse the
/// components.
pub fn from_iterator<I, S>(
    it: I,
    uid_idx: usize,
    gid1_idx: usize,
    gid2_idx: usize,
) -> PermissionFactoryResult
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    // Collect the iterator into a vector
    let args: Vec<S> = it.collect();

    // Get the UID
    // Note the question mark at the end to unwrap
    let uid: Uid = match args.get(uid_idx) {
        None => Err(PermissionFactoryError::UIDNotFound),
        Some(s) => {
            // Convert to a &str for ease of use
            let s_ref = s.as_ref();

            // Get the error to return
            // For readability - we may not always return this
            let err_ret = PermissionFactoryError::UIDMalformed {
                content: s_ref.to_string(),
            };

            // Try to parse
            // If it succeeds, convert to a Uid object
            // If it fails, handle that
            str::parse(s_ref)
                .map(|u| Uid::from_raw(u))
                .map_err(|_| err_ret)
        }
    }?;

    // Get the Primary GID with the same strategy
    let primary_gid: Gid = match args.get(gid1_idx) {
        None => Err(PermissionFactoryError::PrimaryGIDNotFound),
        Some(s) => {
            let s_ref = s.as_ref();
            let err_ret = PermissionFactoryError::PrimaryGIDMalformed {
                content: s_ref.to_string(),
            };
            str::parse(s_ref)
                .map(|g| Gid::from_raw(g))
                .map_err(|_| err_ret)
        }
    }?;

    // Parse the list of secondary GIDs
    // Split on commas, and parse everything else as integers
    let secondary_gids: HashSet<Gid> = match args.get(gid2_idx) {
        None => Err(PermissionFactoryError::SecondaryGIDNotFound),
        Some(s) => {
            // Convert to a &str for ease of use
            let s_ref = s.as_ref();

            // Split the string and collect it into a vector
            let s_spl: Vec<_> = s_ref.split(',').collect();

            // Try to convert each of them to a gid_t
            let gs_r: Vec<_> = s_spl.iter().map(|c| str::parse(c)).collect();

            // If any one failed, return an error
            match gs_r.iter().position(|g_r| g_r.is_err()) {
                None => (),
                Some(i) => {
                    return Err(PermissionFactoryError::SecondaryGIDMalformed {
                        content: s_spl[i].to_string(),
                    })
                }
            };

            // Otherwise, everything succeeded
            // Unwrap everything
            // Convert them into Gids
            Ok(gs_r
                .into_iter()
                .map(|g_r| Gid::from_raw(g_r.unwrap()))
                .collect())
        }
    }?;

    Ok(Permission {
        uid,
        primary_gid,
        secondary_gids,
    })
}
