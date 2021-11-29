//! Module containing methods to verify [Permission]s
//!
//! After some [Permission]s are created, we need to verify that the user is
//! able to invoke the [Executable] they are trying to. There are various checks
//! that might need to be performed. This module holds the methods for doing
//! that. It also defines common types for verification.

pub mod parsed_sudoers_type;
pub mod sudoers;
pub mod sudoers_type;
use super::Permission;
use crate::executable::Executable;
pub use sudoers::from_sudoers;

use std::error::Error;

/// Type for verification functions
///
/// These functions should take in the user's current [Permission], as well as
/// the [Permission] they request and the [Executable] the user wishes to run.
/// They should then return a [VerifyResult] signalling whether the user is
/// allowed to run it.
// pub type Verifier = fn(&Permission, &Permission, &Executable) -> VerifyResult;
pub type Verifier = dyn FnMut(&Permission, &Permission, &Executable) -> VerifyResult;

/// Abstract supertype of [Verifier]
///
/// For testing purposes, we might want to have [Verifier]s signal other parts
/// of the code. This trait allows for that. Since it's a `dyn` type, we can't
/// create variables with it. However, it will work for automatically generated
/// closures.
// pub type AbstractVerifier = dyn FnMut(&Permission, &Permission, &Executable) -> VerifyResult;

/// Convinience type for the result of a [Verifier]
///
/// Verification may succeed or fail, so the return value of a [Verifier] is a
/// [Result]. For convinience, this type aliases to the expected return type.
pub type VerifyResult = Result<(), VerifyError>;

/// String to match on ALL keyword in sudoers
pub const ALL: &str = "ALL";

/// Error for [Verifier]s
///
/// The user may or may not be allowed to run the [Executable] with the
/// [Permission]s they are trying to. It may also just be impossible to verify
/// that the user has credentials due to a system error. This `enum` provides
/// some possibilities.
#[allow(dead_code)]
#[derive(Debug)]
pub enum VerifyError {
    /// The user is not allowed to run the [Executable]
    NotAllowed,
    /// Some component needed for verification was not found
    NotFound { err: Box<dyn Error> },
    /// Some component needed for verification could not be parsed
    Malformed { err: Box<dyn Error> },
}
