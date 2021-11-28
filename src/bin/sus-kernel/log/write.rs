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

use crate::config;
use crate::executable::Executable;
use crate::permission::verify::VerifyResult;
use crate::permission::Permission;

use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn to_write<W>(
    w: &mut W,
    ex: &Executable,
    cur_p: &Permission,
    req_p: &Permission,
    res: &VerifyResult,
) -> LogResult
where
    W: Write,
{
    // Get the Duration since the epoch
    // Don't fail if we're before the epoch. Instead, just print a negative
    //  number.
    let (tstamp_negation, tstamp) = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(res) => (1, res),
        Err(e) => (-1, e.duration()),
    };

    // Write out
    // Use different format strings for success and failure. This causes code
    //  duplication, but it doesn't seem to be avoidable. The two branches of
    //  this `match` are effectively identical.
    // Note the questionmark at the end to unwrap.
    match res {
        Ok(_) => write!(
            w,
            config::LOG_WRITE_SUCCESS_MSG!(),
            tstamp_secs = tstamp_negation * (tstamp.as_secs() as i128),
            tstamp_nanos = tstamp.subsec_nanos(),
            execable = ex,
            cur_perm = cur_p,
            req_perm = req_p,
        ),
        Err(e) => write!(
            w,
            config::LOG_WRITE_FAILURE_MSG!(),
            tstamp_secs = tstamp_negation * (tstamp.as_secs() as i128),
            tstamp_nanos = tstamp.subsec_nanos(),
            execable = ex,
            cur_perm = cur_p,
            req_perm = req_p,
            failure = e,
        ),
    }?;

    Ok(())
}
