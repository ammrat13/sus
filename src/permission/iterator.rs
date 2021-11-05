//! Parse [Permission]s from an [Iterator]
//!
//! This module implements a method to parse a [Permission] from an [Iterator].
//! This functionality can be useful when parsing command line arguments, since
//! those can be turned into an [Iterator] easily. It also works on generic
//! [Vec]s for the same reason, which is useful for testing.

use super::Permission;
use super::PermissionFactoryError;
use super::PermissionFactoryResult;

use libc::{gid_t, uid_t};
use std::collections::HashSet;

/// Function to make a [Permission] from an [Iterator]
///
/// This function will consume the [Iterator] it's given and parse particular
/// indicies to determine the UID, Primary GID, and Secondary GIDs. The indicies
/// it looks at are passed as parameters.
///
/// For parsing, it expects the UID and Primary GID to be base-10 strings. It
/// expects the list of Secondary GIDs to be a comma separated list of base-10
/// strings, with no additional spaces inserted.
///
/// The function will return the created [Permission], or a
/// [PermissionFactoryError] if it was not able to find and parse the
/// components.
#[allow(dead_code)]
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
    let uid: uid_t = match args.get(uid_idx) {
        Some(s) => {
            // Convert to a &str for ease of use
            let s_ref = s.as_ref();
            // Try to parse
            let parse_result = str::parse(s_ref);
            // Handle errors if they arise
            parse_result.map_err(|_| PermissionFactoryError::UIDMalformed {
                content: s.as_ref().to_string(),
            })
        }
        None => Err(PermissionFactoryError::UIDNotFound),
    }?;

    // Get the Primary GID with the same strategy
    let primary_gid: gid_t = match args.get(gid1_idx) {
        Some(s) => {
            let s_ref = s.as_ref();
            let parse_result = str::parse(s_ref);
            parse_result.map_err(|_| PermissionFactoryError::PrimaryGIDMalformed {
                content: s.as_ref().to_string(),
            })
        }
        None => Err(PermissionFactoryError::PrimaryGIDNotFound),
    }?;

    // Parse the list of secondary GIDs
    // Split on commas, and parse everything else as integers
    let secondary_gids: HashSet<gid_t> = match args.get(gid2_idx) {
        Some(s) => {
            // Convert to a &str for ease of use
            // Also split it and collect it into a vector
            let s_ref = s.as_ref();
            let s_spl: Vec<_> = s_ref.split(',').collect();
            // Try to convert each of them to a GID
            let gs: Vec<_> = s_spl.iter().map(|c| str::parse::<gid_t>(c)).collect();
            // If any one failed, return an error
            match gs.iter().position(|g| g.is_err()) {
                Some(i) => Err(PermissionFactoryError::SecondaryGIDMalformed {
                    content: s_spl[i].to_string(),
                }),
                None => Ok(HashSet::from_iter(
                    gs.into_iter().collect::<Result<Vec<gid_t>, _>>().unwrap(),
                )),
            }
        }
        None => Err(PermissionFactoryError::SecondaryGIDNotFound),
    }?;

    Ok(Permission {
        uid,
        primary_gid,
        secondary_gids,
    })
}
