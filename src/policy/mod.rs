//! Module representing [Policy] objects
//!
//! In order to understand what permission user has, we need to parse /etc/sudoers.
//! Entries in /etc/sudoers follow the following pattern, 
//!   
//!   Pattern: User_List Host_List=(Runas_User:Runas_Group) Commands
//!   ex) root    ALL=(ALL:ALL) ALL
//!       %sudo   ALL=(ALL:ALL) ALL
//!       SYSADMINS ALL=(ALL:ALL) ALL
//!       SYSADMINS ALL=(john:sudo) NOPASSWD:SETENV: /usr/bin/cat, (tom) /etc/shadow
//! 
//! We represent each entry using UserSpec struct. An array of UserSpec structs will represent
//! our /etc/sudoers. Notice that UserSpec has an attribute called cmd_specs. Command specs are 
//! described in sudoers file at the last in each entries and are comma separated. 
//! 
pub mod factory;

use std::ffi::CString;
use nix::unistd::{Gid, Uid};

// Struct representing the command spec for each sudoers entry. This struct
// describes what commands that the give user/group is allowed to run. 
pub struct CmdSpec {
  // Usernames that command can run as
  pub runasusers: Vec<String>,
  // Usergroups that command can run as
  pub runasgroups: Vec<String>,
  // Path to the executable
  pub commands: CString,
  // Specifies whether the process can set environment variables
  pub setenv: bool,
  // Prompts for requesting user's password if true
  pub passwd: bool
}

// Policy Struct
pub struct Policy {
  // uids that applies to this user spec
  pub username_list: Vec<Uid>,
  // list of hosts that the user spec is applied. 
  // For now, we only assume ALL for hosts_list
  pub host_list: Vec<String>,
  // list of command specs that users/groups in the lists above can run 
  pub cmd_specs: Vec<CmdSpec>
}