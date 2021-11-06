//! Module containing a function that always succeeds verification
//!
//! This function is useful for debugging purposes. It always succeeds
//! verification.
//!
//! TODO: Remove once no longer needed

use super::Permission;
use super::VerifyResult;
use crate::executable::Executable;

/// Function that always passes verification
pub fn succeed(_: Permission, _: Permission, _: Executable) -> VerifyResult {
    Ok(())
}
