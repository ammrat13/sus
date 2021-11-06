//! Module containing methods to execute [Executable]s
//!
//! [Executable]s have to be run somehow. This module defines methods for
//! invoking them, as well as auxilary types related to those functions.

pub mod exec;
pub use exec::exec;

use super::Executable;
use crate::permission::Permission;

use std::convert::Infallible;

/// Type for functions that run [Executable]s
///
/// These functions take in the [Permission]s the user wishes to execute as, and
/// runs the [Executable] with those permissions. Ideally, this function never
/// returns. If it returns, it always returns a [Result::Err].
pub type Runner = fn(&Permission, &Executable) -> RunResult;
/// Abstract supertype of [Runner]
///
/// For testing purposes, we might want to have [Runner]s signal other parts of
/// the code. This trait allows for that. Since it's a `dyn` type, we can't
/// create variables with it. However, it will work for automatically generated
/// closures.
#[allow(dead_code)]
pub type AbstractRunner = dyn FnMut(&Permission, &Executable) -> RunResult;

/// Convinience type for the result of a [Runner]
///
/// For convinience, this type aliases to the expected return type. Ideally,
/// [Runner]s never return. If they return, they always return in error. As
/// such, the [Ok](Result::Ok) branch of this type is [Infallible] and cannot be
/// explicitly constructed.
///
/// TODO: Change the error type
pub type RunResult = Result<Infallible, ()>;
