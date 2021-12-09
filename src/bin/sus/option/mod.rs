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
use std::error::Error;
use std::ffi::CString;
use std::fmt;
use std::fmt::{Display, Formatter};

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

    /// Function to convert to kernel arguments
    ///
    /// The function either returns a vector of [CString]s if the conversion
    /// succeeds, or an error. The only way for the conversion to fail is if the
    /// arguments don't convert to [CString]s, and the result is always a
    /// [BadParse][bp].
    ///
    /// [bp]: OptionsError::BadParse
    pub fn to_kernel_commandline(&self) -> Result<Vec<CString>, OptionsError> {
        // Create the return arguments
        let ret_str: Vec<String> = vec![
            self.uid.as_raw().to_string(),
            self.primary_gid.as_raw().to_string(),
            self.secondary_gids
                .iter()
                .map(|g| g.as_raw().to_string())
                .collect::<Vec<String>>()
                .join(","),
        ];

        // Convert what we have so far to CStrings
        let mut ret: Vec<CString> = ret_str
            .iter()
            .map(|s| {
                CString::new(s.as_bytes()).map_err(|_| OptionsError::BadParse { string: None })
            })
            .collect::<Result<_, OptionsError>>()?;

        // Add everything else
        ret.push(self.binary.clone());
        ret.append(&mut self.args.clone());

        // Return
        Ok(ret)
    }
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

impl Display for OptionsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error")
    }
}

impl Error for OptionsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
