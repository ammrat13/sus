//! Module to log accesses
//!
//! Often, system administrators will want to log when a user uses this binary
//! to elevate privileges. As such, this module provides methods to do so.
//! The core of this module is the [Logger] and corresponding [AbstractLogger],
//! which log accesses using a [Request] and a [VerifyResult].
