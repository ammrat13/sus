//! Log [Request]s to an existing [Write] object
//!
//! Even though we will likely be logging to [File][f]s most of the time, it
//! would be nice to be able to log to a generic object implementing [Write].
//! That is what this module does. It will log a pretty-printed string to the
//! [Write] object.
//!
//! Note that the results of this module are intended for human use. Everything
//! will be pretty-printed.
//!
//! [f]: std::fs::File

use super::LoggerResult;

use crate::permission::verify::VerifyResult;
use crate::request::Request;

use std::io::Write;

pub fn to_write<W>(_: W, _: &Request, _: &VerifyResult) -> LoggerResult
where
    W: Write,
{
    Ok(())
}
