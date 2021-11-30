//! Module representing [Request]s the user makes
//!
//! The user will make [Request]s for applications to run, and the application
//! will service them. This module provides a structure to represent these
//! [Request]s, and provides a method for executing them. Ultimately, it serves
//! as the main library for this application that the [main](crate::main)
//! function will call into.

use crate::executable::run::AbstractRunner;
use crate::executable::run::RunError;
use crate::executable::Executable;
use crate::permission::verify::Verifier;
use crate::permission::verify::VerifyError;
use crate::permission::verify::VerifyResult;
use crate::permission::Permission;

#[cfg(feature = "log")]
use crate::log::{AbstractLogger, LogError};

use std::convert::Infallible;

/// Structure representing a user request
///
/// The structure is composed of information needed to service the request. It
/// contains the [Executable] the user wishes to run, as well as the
/// [Permission]s the user wishes to run it as. It also has the current
/// [Permission]s of the user. Finally, it has a list of [Verifier][vf]s to check,
/// and an [Runner][rn] to run the [Executable].
///
/// [vf]: crate::permission::verify::Verifier
/// [rn]: crate::executable::run::Runner
pub struct Request {
    /// The [Executable] to run
    pub executable: Executable,

    /// The current [Permission]s of the user
    pub current_permissions: Permission,
    /// The [Permission]s to run the [Executable] with
    pub requested_permissions: Permission,

    /// A list of [Verifier][vf]s to check
    ///
    /// Note that *all* the checks have to pass for the [Executable] to be run.
    /// Effectively, these checks are `AND`ed together. As a corollary, if this
    /// list is empty, the [Executable] will be run unconditionally.
    ///
    /// [vf]: crate::permission::verify::Verifier
    pub verifiers: Vec<Box<Verifier>>,
    /// How to run the [Executable]
    pub runner: Box<AbstractRunner>,

    /// How to log [Request]s
    ///
    /// Regardless of whether it passed all the [Verifiers][vf], this function
    /// will be called with the status. This function can then log the result
    /// somewhere for administration purposes.
    ///
    /// [vf]: crate::permission::verify::Verifier
    #[cfg(feature = "log")]
    pub logger: Box<AbstractLogger>,
}

impl Request {
    /// Function to service a [Request]
    ///
    /// This function consumes the [Request] object and runs it if all the
    /// [Verifier][vf]s pass. Ideally, this function never returns. If it
    /// returns, it always returns in error.
    ///
    /// If a [RequestError::Run] is returned, the application should be taken to
    /// be in an undefined state. As such, the correct course of action is to
    /// terminate the program as soon as possible.
    ///
    /// [vf]: crate::permission::verify::Verifier
    pub fn service(mut self) -> RequestResult {
        // Assert that all the verifications pass
        // Note the question mark to unwrap the result
        let verify_res = {
            let mut res: VerifyResult = Err(VerifyError::NotAllowed {err: None});
            for v in &mut self.verifiers {
                let verifier_result = v(
                    &self.current_permissions,
                    &self.requested_permissions,
                    &self.executable,
                );
                res = res.or(verifier_result);
            }
            // Return
            res
        };
        // Log the attempt result
        // Fail out immediately if we can't
        #[cfg(feature = "log")]
        {
            (self.logger)(
                &self.executable,
                &self.current_permissions,
                &self.requested_permissions,
                &verify_res,
            )
            .map_err(|e| RequestError::Log { cause: e })?;
        }
        // Fail out if we didn't verify
        verify_res.map_err(|e| RequestError::Verify { cause: e })?;
        // Execute and unwrap
        (self.runner)(&self.requested_permissions, &self.executable)
            .map_err(|e| RequestError::Run { cause: e })
    }
}

/// Convinience type for an error returned from the [Request::service] method
///
/// The [service][sv] method will ideally never return. If it returns, it always
/// returns in error. This type is a concise way to write the return type of the
/// [service][sv] method.
///
/// [sv]: Request::service
pub type RequestResult = Result<Infallible, RequestError>;

/// Error type for the [Request::service] method
///
/// The [service][sv] method will ideally never return. If it returns, it always
/// returns in error. These are the types of errors that can be returned. The
/// method will either fail during verification or when it trys to execute the
/// binary. The two possibilities in this `enum` are those cases.
///
/// [sv]: Request::service
#[derive(Debug)]
pub enum RequestError {
    /// An error occured during verification
    Verify { cause: VerifyError },
    /// An error occured when trying to log
    #[cfg(feature = "log")]
    Log { cause: LogError },
    /// An error occurred when trying to run the [Executable]
    Run { cause: RunError },
}
