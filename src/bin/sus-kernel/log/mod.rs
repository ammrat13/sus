//! Module to log accesses
//!
//! Often, system administrators will want to log when a user uses this binary
//! to elevate privileges. As such, this module provides methods to do so.
//! The core of this module is the [Logger] and corresponding [AbstractLogger],
//! which log accesses using a [Request] and a [VerifyResult].

pub mod file;
pub mod write;
pub use file::to_file;
pub use write::to_write;

use crate::permission::verify::VerifyResult;
use crate::request::Request;

use std::error::Error;

/// Type for logging functions
///
/// These functions take in the [Request] that was serviced and the
/// [VerifyResult] that came out of it. When called, they will log their
/// parameters in some way.
pub type Logger = fn(&Request, &VerifyResult) -> LoggerResult;
/// Abstract supetype of [Logger]
///
/// Keeping with how this crate handles verification and execution, this
/// supertype exists so that we can construct [Logger]s at runtime, not being
/// restricted to the [Sized] function pointer type. This might be useful when
/// constructing tests.
pub type AbstractLogger = dyn FnMut(&Request, &VerifyResult) -> LoggerResult;

/// Result type for [Logger]s
///
/// Loggers may return arbitrary errors in the process of writing the data out.
/// As such, it will either succeed with no message, or return a [Box]
/// containing an [Error]. It makes little sense to have a `LoggerError` type
/// since it would provide no other information above the [Error].
pub type LoggerResult = Result<(), Box<dyn Error>>;
