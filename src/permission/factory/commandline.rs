//! Parse [Permission][p]s from the command line
//!
//! This module implements a method to parse [Permission][p]s from command line
//! arguments. It's useful for the main goal of this binary, as the user has to
//! be able to specify what permissions they wish to run as.
//!
//! [p]: super::Permission

use super::from_iterator;
use super::PermissionFactoryResult;

use crate::config;

/// Function to make a [Permission][p] from commandline arguments
///
/// It's essentially a wrapper around [from_iterator](super::from_iterator),
/// passing in the values stored in the configuration file. In particular, it
/// passes in:
///   * [PERMISSION_COMMANDLINE_UID_IDX][cui] for `uid_idx`
///   * [PERMISSION_COMMANDLINE_PRIMARY_GID_IDX][cpgi] for `gid1_idx`
///   * [PERMISSION_COMMANDLINE_SECONDARY_GID_IDX][csgi] for `gid2_idx`
///
/// [p]: super::Permission
/// [cui]: crate::config::PERMISSION_COMMANDLINE_UID_IDX
/// [cpgi]: crate::config::PERMISSION_COMMANDLINE_PRIMARY_GID_IDX
/// [csgi]: crate::config::PERMISSION_COMMANDLINE_SECONDARY_GID_IDX
#[allow(dead_code)]
pub fn from_commandline() -> PermissionFactoryResult {
    from_iterator(
        std::env::args(),
        config::PERMISSION_COMMANDLINE_UID_IDX,
        config::PERMISSION_COMMANDLINE_PRIMARY_GID_IDX,
        config::PERMISSION_COMMANDLINE_SECONDARY_GID_IDX,
    )
}
