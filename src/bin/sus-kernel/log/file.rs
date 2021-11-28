//! Log [Request][rq]s to a pre-configured file
//!
//! The function in this module is used to log [Request][rq]s to a file. The
//! path to file is configured at compile time in the [config] module.
//!
//! [rq]: crate::request::Request

use super::to_write;
use super::LoggerResult;

use crate::config;
use crate::executable::Executable;
use crate::permission::verify::VerifyResult;
use crate::permission::Permission;

use std::fs::OpenOptions;

/// Function to log a given [Request][rq] and [VerifyResult] to a file
///
/// Effectively, this function is a wrapper around the [to_write] logger, which
/// writes to an arbitrary [Write][w] object. This can be done since [File][f]s
/// implement [Write][w]. This method will open the [File][f], pass it off, and
/// close it when done.
///
/// As stated in the top-level documentation, this function does not take in the
/// [Request][rq] directly since that leads to conflicts with mutability.
/// Instead, it takes in the components it needs: the actual [Executable] run,
/// the Current [Permission], and the Requested [Permission].
///
/// The path to the file to be used is pre-configured. It is given by
/// [config::LOG_FILE_PATH].
///
/// This function can error out on its own if it fails to open the file for
/// appending
///
/// [f]: std::fs::File
/// [w]: std::io::Write
/// [rq]: crate::request::Request
pub fn to_file(
    ex: &Executable,
    cur_p: &Permission,
    req_p: &Permission,
    res: &VerifyResult,
) -> LoggerResult {
    // Open the file in append mode
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(config::LOG_FILE_PATH)?;

    // Pass it to the logger and return
    // Don't need to close the file. It will automatically be closed when the
    //  scope ends
    to_write(&mut f, ex, cur_p, req_p, res)
}
