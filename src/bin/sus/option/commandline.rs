//! Parse command line options from the user
//!
//! This structure defines an [CommandLineOptions] struct, which is populated
//! from the command line using the `structopt` library. This can then be
//! converted to a list of arguments to pass to the kernel via `exec`.

use std::collections::HashSet;
use std::ffi::CString;
use std::os::unix::ffi::OsStringExt;

use structopt::clap::AppSettings;
use structopt::StructOpt;

use nix::libc::{gid_t, uid_t};
use nix::unistd;
use nix::unistd::{Gid, Uid};
use users;
use which;

use super::OptionsError;
use super::OptionsLike;

/// The `sus` interface
///
/// The options here are used by the `sus` binary to interface with the
/// `sus-kernel`. It's modeled after `sudo`. However, some options have been
/// stripped away because the kernel can't support them.
// We have to convince `clap` to not put the version. See:
// <https://github.com/TeXitoi/structopt/issues/81
#[derive(Debug, StructOpt)]
#[structopt(
    name = "sus",
    global_settings = &[AppSettings::DisableVersion]
)]
pub struct CommandLineOptions {
    /// The User to run as
    #[structopt(short = "u")]
    user: Option<String>,

    /// The Primary Group to run as
    #[structopt(short = "g")]
    primary_group: Option<String>,

    /// Preserve the Secondary Groups vector
    #[structopt(short = "P")]
    preserve_secondary_groups: bool,

    /// Whether to just run the shell
    #[structopt(short = "s")]
    shell: bool,
    /// Whether to run the shell as login
    #[structopt(short = "i")]
    shell_login: bool,

    /// The binary to execute and the arguments to give it
    #[structopt(parse(try_from_str = CString::new))]
    command: Vec<CString>,
}

impl OptionsLike for CommandLineOptions {
    /// Function to get the UID from the user's name
    ///
    /// This function parses the name the user gives. If they don't give
    /// anything, the default is root. Otherwise, if the user prefixes the name
    /// with a `#`, it's interpreted as a number. Otherwise, we look up the
    /// name.
    fn uid(&self) -> Result<Uid, OptionsError> {
        // Root if not provided
        // Otherwise, extract a &str
        let user_req = match &self.user {
            None => {
                return Ok(Uid::from_raw(0));
            }
            Some(u) => u,
        };

        // If the string starts with `#`, parse the number
        match user_req.strip_prefix('#') {
            None => {}
            Some(id_str) => {
                // Parse to an integer and return failure if can't
                return match id_str.parse::<uid_t>() {
                    Err(_) => Err(OptionsError::BadParse {
                        string: Some(id_str.to_string()),
                    }),
                    Ok(id) => Ok(Uid::from_raw(id)),
                };
            }
        };

        // Otherwise, look up
        match users::get_user_by_name(user_req) {
            None => Err(OptionsError::UserNotFound {
                name: Some(user_req.to_string()),
            }),
            Some(u) => Ok(Uid::from_raw(u.uid())),
        }
    }

    /// Function to get the Primary GID
    ///
    /// Very similar to [Uid], with much the same semantics. Allows the user to
    /// set the primary group to execute as.
    fn primary_gid(&self) -> Result<Gid, OptionsError> {
        // Root if not provided
        // Otherwise, extract a &str
        let group_req = match &self.primary_group {
            None => {
                return Ok(Gid::from_raw(0));
            }
            Some(u) => u,
        };

        // If the string starts with `#`, parse the number
        match group_req.strip_prefix('#') {
            None => {}
            Some(gid_str) => {
                // Parse to an integer and return failure if can't
                return match gid_str.parse::<gid_t>() {
                    Err(_) => Err(OptionsError::BadParse {
                        string: Some(gid_str.to_string()),
                    }),
                    Ok(gid) => Ok(Gid::from_raw(gid)),
                };
            }
        };

        // Otherwise look up
        match users::get_group_by_name(group_req) {
            None => Err(OptionsError::GroupNotFound {
                name: Some(group_req.to_string()),
            }),
            Some(g) => Ok(Gid::from_raw(g.gid())),
        }
    }

    /// Function to get the Secondary GIDs
    ///
    /// These parameters aren't given explicitly. Instead, the user supplies the
    /// UID they want to run as. If they don't preserve their current groups,
    /// they run as the groups of the target user.
    ///
    /// If the user does not provide a name for the user,
    fn secondary_gids(&self) -> Result<HashSet<Gid>, OptionsError> {
        // If we need to preserve GIDs, do that
        if self.preserve_secondary_groups {
            return match unistd::getgroups() {
                Err(n) => Err(OptionsError::SyscallFailure {
                    syscall_name: Some("getgroups"),
                    err: Some(n),
                }),
                Ok(v) => Ok(v.into_iter().collect()),
            };
        }

        // Otherwise, get the groups of the target user
        let uname = match &self.user {
            Some(u) => u,
            None => "root",
        };
        let gid = self.primary_gid()?.as_raw();
        match users::get_user_groups(uname, gid) {
            None => Err(OptionsError::GroupNotFound { name: None }),
            Some(v) => {
                // Collect the results
                let mut gids: HashSet<Gid> =
                    v.into_iter().map(|g| Gid::from_raw(g.gid())).collect();
                // Remove root
                // No idea why it's being returned
                gids.remove(&Gid::from_raw(0));
                // Return
                Ok(gids)
            }
        }
    }

    /// Function to ge the path to the binary to run
    ///
    /// The user only enters the name of the binary, so we have to look through
    /// the `PATH` environment variable. If we can't find it, we error out.
    ///
    /// Additionally, if the user wants to run in a shell, we honor that by
    /// returning "/bin/sh".
    fn binary(&self) -> Result<CString, OptionsError> {
        // If the user wants to run a shell, give a hard-coded result
        if self.shell || self.shell_login {
            return CString::new("/bin/sh").map_err(|_| OptionsError::BadParse { string: None });
        }

        // Parse the command to a &str
        let cmd = match self.command.get(0) {
            None => {
                return Err(OptionsError::BinaryNotFound {
                    name: Some("Not supplied".to_string()),
                })
            }
            Some(c) => match c.to_str() {
                Err(_) => return Err(OptionsError::BadParse { string: None }),
                Ok(s) => s,
            },
        };

        // Try to find it
        match which::which(cmd) {
            Err(_) => Err(OptionsError::BinaryNotFound {
                name: Some(cmd.to_string()),
            }),
            Ok(p) => CString::new(p.into_os_string().into_vec())
                .map_err(|_| OptionsError::BadParse { string: None }),
        }
    }

    /// Function to get the arguments to the binary
    ///
    /// This function returns the arguments to pass to the binary, including
    /// argument zero. If the user doesn't want a shell, this essentially passes
    /// the arguments the user gave. If they do, it concatenates all of them
    /// together with spaces.
    fn args(&self) -> Result<Vec<CString>, OptionsError> {
        // Handle the easier case
        if !self.shell && !self.shell_login {
            return Ok(self.command.clone());
        }

        // Create the return vector
        // Push the shell binary, failing if conversion fails
        let mut ret =
            vec![CString::new("/bin/sh").map_err(|_| OptionsError::BadParse { string: None })?];

        // Run as login if needed, failing if the conversion fails
        if self.shell_login {
            ret.push(CString::new("-l").map_err(|_| OptionsError::BadParse { string: None })?);
        }

        // Execute a particular command if we're not just executing a shell
        if !self.command.is_empty() {
            ret.push(CString::new("-c").map_err(|_| OptionsError::BadParse { string: None })?);
            ret.push(
                CString::new(
                    self.command
                        .iter()
                        .map(|s| s.as_bytes())
                        .collect::<Vec<&[u8]>>()
                        .join(&0x20),
                )
                .map_err(|_| OptionsError::BadParse { string: None })?,
            );
        }

        Ok(ret)
    }
}
