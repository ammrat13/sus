//! Parse [Executable]s from an [Iterator]
//!
//! This module implements a method to parse an [Executable] using an [Iterator]
//! object. This functionality can be useful when parsing command line
//! arguments, as those can be turned into an [Iterator] easily. It also works
//! on generic [Vec]s for the same reason.

#[allow(unused_imports)]
use super::Executable;
use super::ExecutableFactoryError;
use super::ExecutableFactoryResult;

#[allow(dead_code)]
pub fn from_iterator<I, S>(
    _it: I,
    _path_idx: usize,
    _args_start_idx: usize,
) -> ExecutableFactoryResult
where
    I: Iterator<Item = S>,
    S: AsRef<String>,
{
    Err(ExecutableFactoryError::ArgNotFound { position: 0 })
}
