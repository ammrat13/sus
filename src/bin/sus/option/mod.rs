//! Module for handling the options the user passes
//!
//! The user can pass various [CommandLineOptions] to the `sus` binary to tell
//! it what command to execute. These [CommandLineOptions] need to further be
//! parsed to map it to the [Options] the `sus-kernel` needs to take. This
//! module houses all of that functionality.

pub mod commandline;
pub use commandline::CommandLineOptions;

use nix::errno::Errno;
use nix::unistd::{Gid, Uid};
use std::collections::HashSet;
use std::ffi::CString;

/// The options to pass to the `sus-kernel`
///
/// This structure is used internally to find out how to execute the kernel. It
/// is created by parsing [CommandLineOptions] then converting them to this
/// structure.
#[derive(Debug)]
pub struct Options {
    /// The User to execute as
    uid: Uid,
    /// The Primary Group to change to
    primary_gid: Gid,
    /// The list of Secondary Groups to use
    secondary_gids: HashSet<Gid>,

    /// The name of the executable to run
    binary: CString,
    /// The arguments to pass to the executable
    args: Vec<CString>,
}

/// Trait to define things that can be parsed into [Options]
///
/// It might be that the user can input data in a format that needs to be
/// converted to an [Options] structure. For instance, command line arguments a
/// probably not explicit in all the parameter. Thus, this trait allows
/// different methods of collecting parameters from a user and merging them into
/// a common iterface.
pub trait OptionsLike {
    /// Function to get the UID
    fn uid(&self) -> Result<Uid, OptionsError>;
    /// Function to get the Primary GID
    fn primary_gid(&self) -> Result<Gid, OptionsError>;
    /// Function to get the Secondary GIDs
    fn secondary_gids(&self) -> Result<HashSet<Gid>, OptionsError>;
    /// Function to ge the path to the binary to run
    fn binary(&self) -> Result<CString, OptionsError>;
    /// Function to get the arguments to the binary
    fn args(&self) -> Result<Vec<CString>, OptionsError>;
}

/// Type for reporting errors when working with [Options]
///
/// Things to generate [Options] may fail. For instance, even though the user
/// enters some arguments successfully at the command line, they may still be
/// invalid. For example, they may not input a number where one is required, or
/// they may provide the name of a nonexisting group. This enumeration handles
/// those failure cases.
#[derive(Debug)]
pub enum OptionsError {
    /// Could not parse something
    BadParse { string: Option<String> },

    /// User does not exist
    UserNotFound { name: Option<String> },
    /// Group does not exist
    GroupNotFound { name: Option<String> },

    /// Binary was not found
    BinaryNotFound { name: Option<String> },

    /// Generic failure of a system call
    SyscallFailure {
        syscall_name: Option<&'static str>,
        err: Option<Errno>,
    },
}

impl Options {
    /// Create an [Options] from an [OptionsLike]
    ///
    /// `TryFrom` would be better here, but we can't use that with generics.
    /// See: https://github.com/rust-lang/rust/issues/50133
    pub fn parse_options_like<T>(ol: T) -> Result<Options, OptionsError>
    where
        T: OptionsLike,
    {
        // Return the results of the function calls
        Ok(Options {
            uid: ol.uid()?,
            primary_gid: ol.primary_gid()?,
            secondary_gids: ol.secondary_gids()?,
            binary: ol.binary()?,
            args: ol.args()?,
        })
    }
}
