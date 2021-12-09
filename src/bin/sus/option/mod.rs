//! Module for handling the options the user passes
//!
//! The user can pass various [CommandLineOptions] to the `sus` binary to tell
//! it what command to execute. These [CommandLineOptions] need to further be
//! parsed to map it to the [Options] the `sus-kernel` needs to take. This
//! module houses all of that functionality.

pub mod commandline;
pub use commandline::CommandLineOptions;

use core::convert::Infallible;
use nix::errno::Errno;
use nix::unistd;
use nix::unistd::{Gid, Uid};
use std::collections::HashSet;
use std::error::Error;
use std::ffi::CString;
use std::fmt;
use std::fmt::{Display, Formatter};

use crate::config;

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
    /// See: <https://github.com/rust-lang/rust/issues/50133>
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

    /// Call into the kernel with these [Options]
    ///
    /// This method will generate the command line arguments using
    /// [Options::to_kernel_commandline], then call `execvp` with those
    /// arguments using the configured kernel path.
    pub fn execute(self) -> Result<Infallible, OptionsError> {
        unistd::execvp(
            &make_cstring(config::KERNEL_PATH.to_string())?,
            &self.to_kernel_commandline()?,
        )
        .map_err(|n| OptionsError::SyscallFailure {
            name: Some("execvp"),
            err: Some(n),
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
    fn to_kernel_commandline(&self) -> Result<Vec<CString>, OptionsError> {
        // Create the return vector
        let mut ret: Vec<CString> = vec![make_cstring(config::KERNEL_PATH.to_string())?];
        // Populate the rest with empty strings
        for _ in 1..config::KERNEL_COMMANDLINE_ARG_START_IDX {
            ret.push(make_cstring("".to_string())?);
        }

        // Write the arguments
        ret[config::KERNEL_COMMANDLINE_UID_IDX] = make_cstring(self.uid.as_raw().to_string())?;
        ret[config::KERNEL_COMMANDLINE_PRIMARY_GID_IDX] =
            make_cstring(self.primary_gid.as_raw().to_string())?;
        ret[config::KERNEL_COMMANDLINE_SECONDARY_GID_IDX] = make_cstring(
            self.secondary_gids
                .iter()
                .map(|g| g.as_raw().to_string())
                .collect::<Vec<String>>()
                .join(","),
        )?;
        ret[config::KERNEL_COMMANDLINE_BINARY_IDX] = self.binary.clone();

        // Push arguments
        ret.extend(self.args.clone());

        // Return
        Ok(ret)
    }
}

/// Convenience function that handles CString failures
fn make_cstring(s: String) -> Result<CString, OptionsError> {
    CString::new(s.as_bytes()).map_err(|_| OptionsError::BadParse { string: None })
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
        name: Option<&'static str>,
        err: Option<Errno>,
    },
}

impl Display for OptionsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            OptionsError::BadParse { string } => {
                match string {
                    None => write!(f, "Failed to parse string")?,
                    Some(s) => write!(f, "Failed to parse string - {}", s)?,
                };
            }
            OptionsError::UserNotFound { name } => {
                match name {
                    None => write!(f, "User not found")?,
                    Some(n) => write!(f, "User not found - {}", n)?,
                };
            }
            OptionsError::GroupNotFound { name } => {
                match name {
                    None => write!(f, "Group not found")?,
                    Some(n) => write!(f, "Group not found - {}", n)?,
                };
            }
            OptionsError::BinaryNotFound { name } => {
                match name {
                    None => write!(f, "Target binary not found")?,
                    Some(n) => write!(f, "Target binary not found - {}", n)?,
                };
            }
            OptionsError::SyscallFailure { name, err } => {
                // Write the header
                write!(f, "Failed to make system call")?;
                // Write a dash if needed
                if name.is_some() || err.is_some() {
                    write!(f, " - ")?;
                }
                // Write the name
                match name {
                    Some(n) => write!(f, "`{}`", n)?,
                    None => (),
                };
                // Write the error
                match err {
                    Some(e) => write!(f, " {}", e)?,
                    None => (),
                };
            }
        };

        Ok(())
    }
}

impl Error for OptionsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
