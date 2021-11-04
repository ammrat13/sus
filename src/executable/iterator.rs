//! Parse [Executable]s from an [Iterator]
//!
//! This module implements a method to parse an [Executable] using an [Iterator]
//! object. This functionality can be useful when parsing command line
//! arguments, as those can be turned into an [Iterator] easily. It also works
//! on generic [Vec]s for the same reason.

use super::Executable;
use super::ExecutableFactoryError;
use super::ExecutableFactoryResult;

use std::ffi::{CString, NulError};
use std::path::PathBuf;

/// Function to make an [Executable] from an [Iterator]
///
/// This function will consume the iterator it's given, and look at particular
/// indices of it to determine the path and the arguments to pass to the
/// executable. In particular, it looks at:
///   * `path_idx` to find the index of the iterator to go to for the path
///   * `args_start_idx` to find the first argument, with everything after being
///     subsequent arguments
///
/// This function will return the created executable, or an error on failure. It
/// will return a [PathNotFound](ExecutableFactoryError::PathNotFound) if the
/// index for the path could not be found. It will also produce a
/// [ArgNotFound](ExecutableFactoryError::ArgNotFound) error if the first
/// argument couldn't be found, and a
/// [ArgMalformed](ExecutableFactoryError::ArgMalformed) error if it can't be
/// converted to a [CString].
#[allow(dead_code)]
pub fn from_iterator<'a, I, S>(
    it: I,
    path_idx: usize,
    args_start_idx: usize,
) -> ExecutableFactoryResult
where
    I: Iterator<Item = &'a S>,
    S: 'a + AsRef<str>,
{
    // Collect the iterator into a vector
    let args: Vec<&str> = it.map(|s| s.as_ref()).collect();

    // Get the path to return
    // Note the question mark at the end
    let path: PathBuf = match args.get(path_idx) {
        Some(s) => Ok(PathBuf::from(*s)),
        None => Err(ExecutableFactoryError::PathNotFound),
    }?;

    // Get the arguments
    // Note the question mark at the end
    let args: Vec<CString> = match args.get(args_start_idx..) {
        Some(ss) => {
            // Try to convert everything to a CString
            let rs: Vec<Result<CString, NulError>> = ss.iter().map(|s| CString::new(*s)).collect();
            // If any one failed, return an error
            match rs.iter().position(|r| r.is_err()) {
                Some(i) => Err(ExecutableFactoryError::ArgMalformed { position: i }),
                None => Ok(rs
                    .into_iter()
                    .collect::<Result<Vec<CString>, NulError>>()
                    .unwrap()),
            }
        }
        None => Err(ExecutableFactoryError::ArgNotFound {
            position: args_start_idx,
        }),
    }?;

    Ok(Executable { path, args })
}
