//! Parse [Executable][sE]s from the command line
//!
//! This module implements a method to parse [Executable][sE]s from command line
//! arguments. It's useful for the main goal of this binary.
//!
//! [sE]: super::Executable

use super::from_iterator;
use super::ExecutableFactoryResult;

use crate::config;

/// Function to make an [Executable][sE] from command line arguments
///
/// It's essentially a wrapper around [from_iterator](super::from_iterator),
/// passing in the values stored in the configuration file. In particular, it
/// passes in:
///   * [EXECUTABLE_COMMANDLINE_PATH_IDX][cpi] for `path_idx`
///   * [EXECUTABLE_COMMANDLINE_ARG_START_IDX][cai] for `arg_start_idx`
///
/// [sE]: super::Executable
/// [cpi]: crate::config::EXECUTABLE_COMMANDLINE_PATH_IDX
/// [cai]: crate::config::EXECUTABLE_COMMANDLINE_ARG_START_IDX
#[allow(dead_code)]
pub fn from_commandline() -> ExecutableFactoryResult {
    from_iterator(
        std::env::args(),
        config::EXECUTABLE_COMMANDLINE_PATH_IDX,
        config::EXECUTABLE_COMMANDLINE_ARG_START_IDX,
    )
}
