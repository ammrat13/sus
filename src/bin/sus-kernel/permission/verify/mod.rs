//! Module containing methods to verify [Permission]s
//!
//! After some [Permission]s are created, we need to verify that the user is
//! able to invoke the [Executable] they are trying to. There are various checks
//! that might need to be performed. This module holds the methods for doing
//! that. It also defines common types for verification.

use super::Permission;
use crate::executable::Executable;

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Type for verification functions
///
/// These functions should take in the user's current [Permission], as well as
/// the [Permission] they request and the [Executable] the user wishes to run.
/// They should then return a [VerifyResult] signalling whether the user is
/// allowed to run it.
pub type Verifier = fn(&Permission, &Permission, &Executable) -> VerifyResult;
/// Abstract supertype of [Verifier]
///
/// For testing purposes, we might want to have [Verifier]s signal other parts
/// of the code. This trait allows for that. Since it's a `dyn` type, we can't
/// create variables with it. However, it will work for automatically generated
/// closures.
pub type AbstractVerifier = dyn FnMut(&Permission, &Permission, &Executable) -> VerifyResult;

/// Convinience type for the result of a [Verifier]
///
/// Verification may succeed or fail, so the return value of a [Verifier] is a
/// [Result]. For convinience, this type aliases to the expected return type.
pub type VerifyResult = Result<(), VerifyError>;

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
    NotAllowed { err: Box<dyn Error> },
    /// Some component needed for verification was not found
    NotFound { err: Box<dyn Error> },
    /// Some component needed for verification could not be parsed
    Malformed { err: Box<dyn Error> },
}

impl Display for VerifyError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Write different things depending on the type
        match self {
            // Special message if not allowed
            VerifyError::NotAllowed { err } => {
                write!(f, "Access Denied - {}", err)?;
            }
            // Internal errors
            VerifyError::NotFound { err } => {
                write!(f, "Internal Error NotFound - {}", err)?;
            }
            VerifyError::Malformed { err } => {
                write!(f, "Internal Error Malformed - {}", err)?;
            }
        }
        // Return
        Ok(())
    }
}
