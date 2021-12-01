//! Module to log accesses
//!
//! Often, system administrators will want to log when a user uses this binary
//! to elevate privileges. As such, this module provides methods to do so.
//! The core of this module is the [Logger] and corresponding [AbstractLogger],
//! which log accesses using a [Request][rq] and a [VerifyResult].
//!
//! [rq]: crate::request::Request

#![cfg(feature = "log")]

pub mod file;
pub use file::to_file;

mod write;
use write::to_write;

use crate::executable::Executable;
use crate::permission::verify::VerifyResult;
use crate::permission::Permission;

use std::error::Error;

/// Type for logging functions
///
/// These functions effectively take in the [Request][rq] that was serviced and
/// the [VerifyResult] that came out of it. When called, they will log their
/// parameters in some way.
///
/// Note that they do not take in the [Request][rq] directly. That leads to
/// issues with mutablility. The [Logger] can modify its state, but the
/// [Request][rq] is logically borrowed immutably. Thus, we pass the parameters
/// we actually need.
///
/// [rq]: crate::request::Request
pub type Logger = fn(&Executable, &Permission, &Permission, &VerifyResult) -> LogResult;
/// Abstract supetype of [Logger]
///
/// Keeping with how this crate handles verification and execution, this
/// supertype exists so that we can construct [Logger]s at runtime, not being
/// restricted to the [Sized] function pointer type. This might be useful when
/// constructing tests.
pub type AbstractLogger =
    dyn FnMut(&Executable, &Permission, &Permission, &VerifyResult) -> LogResult;

/// Result type for [Logger]s
///
/// [Logger]s may return arbitrary errors in the process of writing the data
/// out. On success, they don't return anything. This type captures that.
pub type LogResult = Result<(), LogError>;
/// Error type for [Logger]s
///
/// [Logger]s may return arbitrary errors in the process of writing the data
/// out. As such, this type is really only provided for convenience. It's an
/// alias to a general error [Box].
pub type LogError = Box<dyn Error>;
