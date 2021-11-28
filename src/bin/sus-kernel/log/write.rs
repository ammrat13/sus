//! Log [Request][rq]s to an existing [Write] object
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
//! [rq]: crate::request::Request

use super::LogResult;

use crate::executable::Executable;
use crate::permission::verify::VerifyResult;
use crate::permission::Permission;
use crate::{CONFIG_LOG_FAILURE_MSG, CONFIG_LOG_SUCCESS_MSG};

use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn to_write<W>(
    w: &mut W,
    _: &Executable,
    _: &Permission,
    _: &Permission,
    res: &VerifyResult,
) -> LogResult
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
    // Use different format strings for success and failure. This causes code
    //  duplication, but it doesn't seem to be avoidable. The two branches of
    //  this `match` are effectively identical.
    // Note the questionmark at the end to unwrap.
    match res {
        Ok(_) => write!(
            w,
            CONFIG_LOG_SUCCESS_MSG!(),
            tstamp_sign = if tstamp_is_neg { "-" } else { " " },
            tstamp_secs = tstamp.as_secs(),
            tstamp_nanos = tstamp.subsec_nanos(),
        ),
        Err(_) => write!(
            w,
            CONFIG_LOG_FAILURE_MSG!(),
            tstamp_sign = if tstamp_is_neg { "-" } else { " " },
            tstamp_secs = tstamp.as_secs(),
            tstamp_nanos = tstamp.subsec_nanos(),
        ),
    }?;

    Ok(())
}
