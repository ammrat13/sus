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
use std::fmt;
use std::fmt::{Display, Formatter};

/// Type for verification functions
///
/// These functions should take in the user's current [Permission], as well as
/// the [Permission] they request and the [Executable] the user wishes to run.
/// They should then return a [VerifyResult] signalling whether the user is
/// allowed to run it.
pub type Verifier = dyn FnMut(&Permission, &Permission, &Executable) -> VerifyResult;

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
    NotAllowed { err: Option<Box<dyn Error>> },
    /// Some component needed for verification was not found
    NotFound { err: Option<Box<dyn Error>> },
    /// Some component needed for verification could not be parsed
    Malformed { err: Option<Box<dyn Error>> },
}

impl Display for VerifyError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Store both the message and the "wrapped" error
        // Match on how we were constructed to get these
        let (err, msg) = match self {
            VerifyError::NotAllowed { err: e } => (e, "Access Denied"),
            VerifyError::NotFound { err: e } => (e, "Internal Error NotFound"),
            VerifyError::Malformed { err: e } => (e, "Internal Error Malformed"),
        };
        // Print out the message
        // Also print details if needed
        match err {
            Some(e) => {
                write!(f, "{} - {}", msg, e)?;
            }
            None => {
                write!(f, "{}", msg)?;
            }
        }
        // Return
        Ok(())
    }
}
