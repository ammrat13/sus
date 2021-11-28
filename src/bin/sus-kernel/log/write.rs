//! Log [Request]s to an existing [Write] object
//!
//! Even though we will likely be logging to [File][f]s most of the time, it
//! would be nice to be able to log to a generic object implementing [Write].
//! That is what this module does. It will log a pretty-printed string to the
//! [Write] object.
//!
//! For logging, this module uses [SystemTime] instead of [Instant][inst]. As a
//! result, the timestamps are not monotonic. Be aware of that.
//!
//! Note that the results of this module are intended for human use. Everything
//! will be pretty-printed.
//!
//! [f]: std::fs::File
//! [inst]: std::time::Instant

use super::LoggerResult;

use crate::permission::verify::VerifyResult;
use crate::request::Request;

use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn to_write<W>(w: &mut W, _: &Request, _: &VerifyResult) -> LoggerResult
where
    W: Write,
{
    // Get the Duration since the epoch
    // Don't fail if we're before the epoch. Instead, just print a negative
    //  number.
    let (tstamp_is_neg, tstamp) = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(res) => (false, res),
        Err(e) => (true, e.duration()),
    };

    // Write out
    write!(
        w,
        "{tstamp_sign}{tstamp_secs}.{tstamp_nanos:0<9}",
        tstamp_sign = if tstamp_is_neg { "-" } else { " " },
        tstamp_secs = tstamp.as_secs(),
        tstamp_nanos = tstamp.subsec_nanos(),
    )?;

    Ok(())
}
