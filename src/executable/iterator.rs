//! Parse [Executable]s from an [Iterator]
//!
//! This module implements a method to parse an [Executable] using an [Iterator]
//! object. This functionality can be useful when parsing command line
//! arguments, as those can be turned into an [Iterator] easily. It also works
//! on generic [Vec]s for the same reason, which is useful for testing.

use super::Executable;
use super::ExecutableFactoryError;
use super::ExecutableFactoryResult;

use std::ffi::{CString, NulError};
use std::path::PathBuf;

/// Function to make an [Executable] from an [Iterator]
///
/// This function will consume the [Iterator] it's given, and look at particular
/// indices of it to determine the path and the arguments to pass to the
/// executable. In particular, it looks at:
///   * `path_idx` to find the index of the iterator to go to for the path
///   * `args_start_idx` to find the first argument, with everything after being
///     subsequent arguments
///
/// This function will return the created executable, or an error on failure. It
/// will return a [PathNotFound][pnf] if the index for the path could not be
/// found. It will also produce a [ArgMalformed][am] error if any argument can't
/// be converted to a [CString].
///
/// [pnf]: ExecutableFactoryError::PathNotFound
/// [anf]: ExecutableFactoryError::ArgNotFound
/// [am]: ExecutableFactoryError::ArgMalformed
pub fn from_iterator<I, S>(it: I, path_idx: usize, args_start_idx: usize) -> ExecutableFactoryResult
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    // Collect the iterator into a vector
    let args: Vec<S> = it.collect();

    // Get the path to return
    // Note the question mark at the end
    let path: PathBuf = match args.get(path_idx) {
        Some(s) => Ok(PathBuf::from(s.as_ref())),
        None => Err(ExecutableFactoryError::PathNotFound),
    }?;

    // Get the arguments
    // Note the question mark at the end
    let args: Vec<CString> = match args.get(args_start_idx..) {
        Some(ss) => {
            // Try to convert everything to a CString
            let rs: Vec<Result<CString, NulError>> =
                ss.iter().map(|s| CString::new(s.as_ref())).collect();
            // If any one failed, return an error
            match rs.iter().position(|r| r.is_err()) {
                Some(i) => Err(ExecutableFactoryError::ArgMalformed {
                    position: i,
                    content: ss.get(i).unwrap().as_ref().to_string(),
                }),
                None => Ok(rs
                    .into_iter()
                    .collect::<Result<Vec<CString>, NulError>>()
                    .unwrap()),
            }
        }
        None => Ok(Vec::new()),
    }?;

    Ok(Executable { path, args })
}
