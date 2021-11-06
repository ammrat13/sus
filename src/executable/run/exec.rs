//! Module containing a method that [execve][eve]s an [Executable]
//!
//! Most commonly, the user will want to run the binary. This module provides a
//! way to do that. It will either change to the given [Executable], or it will
//! fail to do so and return to this application.
//!
//! [eve]: https://man7.org/linux/man-pages/man2/execve.2.html

use crate::permission::Permission;
use super::Executable;
use super::RunResult;

/// Function that calls [execve][eve] to run the [Executable] given
///
/// It will set the permissions to those given in the first parameter, then
/// execute the new binary. It only returns if any of those steps failed.
///
/// TODO: Implement
///
/// [eve]: https://man7.org/linux/man-pages/man2/execve.2.html
pub fn exec(_: &Permission, _: &Executable) -> RunResult {
    Err(())
}
