//! Parse command line options from the user
//!
//! This structure defines an [CommandLineOptions] struct, which is populated
//! from the command line using the `structopt` library. This can then be
//! converted to a list of arguments to pass to the kernel via `exec`.

use std::ffi::CString;
use structopt::StructOpt;

/// The `sus` interface
///
/// The options here are used by the `sus` binary to interface with the
/// `sus-kernel`. It's modeled after `sudo`. However, some options have been
/// stripped away because the kernel can't support them.
#[derive(Debug, StructOpt)]
#[structopt(name = "sus")]
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

    /// Run the given command in the background
    #[structopt(short = "b")]
    background: bool,

    /// The command to execute
    #[structopt(parse(try_from_str = CString::new))]
    command: Vec<CString>,
}
