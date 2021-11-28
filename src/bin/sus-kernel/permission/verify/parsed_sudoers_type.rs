// Since sudoers store users by username and not uid,
// we use special types in this file to easily query sudoers using ids
use super::sudoers_type;
use super::sudoers_type::Command;
use super::sudoers_type::User::{Useralias, Usergroup, Username};
use super::Permission;
use super::ALL;
use nix::unistd::{Gid, Uid};
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::CString;
use users::{get_group_by_name, get_user_by_name};

#[derive(Debug)]
pub struct AllowedCmd {
    pub users: HashSet<Uid>,
    pub allow_all_users: bool,
    pub groups: HashSet<Gid>,
    pub allow_all_groups: bool,
    pub paths: HashSet<CString>,
    pub allow_all_cmds: bool,
    pub options: Vec<sudoers_type::Option>,
}

impl AllowedCmd {
    pub fn new() -> Self {
        AllowedCmd {
            users: HashSet::new(),
            groups: HashSet::new(),
            paths: HashSet::new(),
            options: Vec::new(),
            allow_all_cmds: false,
            allow_all_users: false,
            allow_all_groups: false,
        }
    }
    pub fn is_relevant(&self, req_perm: &Permission) -> bool {
        self.users.contains(&req_perm.uid)
            || self.groups.contains(&req_perm.primary_gid)
            || !self.groups.is_disjoint(&req_perm.secondary_gids)
            || self.allow_all_users
            || self.allow_all_groups
    }
}
#[derive(Debug)]
pub struct Rule {
    pub users: HashSet<Uid>,
    pub allow_all_users: bool,
    pub groups: HashSet<Gid>,
    pub allow_all_groups: bool,
    pub allowed_cmds: Vec<AllowedCmd>,
    pub allow_all_cmds: bool,
}

fn get_uid_from_username(username: &str) -> Option<Uid> {
    get_user_by_name(username).map(|user| Uid::from_raw(user.uid()))
}

fn get_gid_from_groupname(groupname: &str) -> Option<Gid> {
    get_group_by_name(groupname).map(|user| Gid::from_raw(user.gid()))
}

impl Rule {
    pub fn new() -> Self {
        Rule {
            users: HashSet::new(),
            groups: HashSet::new(),
            allowed_cmds: Vec::new(),
            allow_all_groups: false,
            allow_all_users: false,
            allow_all_cmds: false,
        }
    }

    pub fn is_relevant(&self, curr_perm: &Permission) -> bool {
        self.users.contains(&curr_perm.uid)
            || self.groups.contains(&curr_perm.primary_gid)
            || !self.groups.is_disjoint(&curr_perm.secondary_gids)
            || self.allow_all_users
            || self.allow_all_groups
    }

    pub fn from_userspec(
        userspec: &sudoers_type::UserSpec,
        useraliases: &HashMap<String, Vec<sudoers_type::User>>,
    ) -> Self {
        let mut rule = Rule::new();
        // Populate rule.users and rule.groups with uid and gid
        for user in &userspec.user_list {
            match user {
                Username(username) => {
                    if username.eq(&ALL) {
                        rule.allow_all_users = true;
                    } else if let Some(uid) = get_uid_from_username(username) {
                        rule.users.insert(uid);
                    }
                }
                Usergroup(groupname) => {
                    if groupname.eq(&ALL) {
                        rule.allow_all_groups = true;
                    } else if let Some(gid) = get_gid_from_groupname(groupname) {
                        rule.groups.insert(gid);
                    }
                }
                Useralias(alias) => {
                    for user in &useraliases[alias] {
                        if let Username(username) = user {
                            if username.eq(&ALL) {
                                rule.allow_all_users = true;
                            } else if let Some(uid) = get_uid_from_username(username) {
                                rule.users.insert(uid);
                            }
                        }
                    }
                }
            }
        }
        for cmd_spec in &userspec.cmd_specs {
            let mut allowed_cmd = AllowedCmd::new();
            for runasuser in &cmd_spec.run_as_users {
                if let Username(username) = runasuser {
                    if username.eq(&ALL) {
                        allowed_cmd.allow_all_users = true;
                    } else if let Some(uid) = get_uid_from_username(username) {
                        allowed_cmd.users.insert(uid);
                    }
                }
            }
            for runasgroup in &cmd_spec.run_as_groups {
                if let Usergroup(groupname) = runasgroup {
                    if groupname.eq(&ALL) {
                        allowed_cmd.allow_all_users = true;
                    } else if let Some(gid) = get_gid_from_groupname(groupname) {
                        allowed_cmd.groups.insert(gid);
                    }
                }
            }
            for option in &cmd_spec.options {
                allowed_cmd.options.push(option.clone());
            }
            for command in &cmd_spec.commands {
                let Command::CmdPath(path) = command;

                if path.to_str().unwrap().eq(ALL) {
                    allowed_cmd.allow_all_cmds = true;
                } else {
                    allowed_cmd.paths.insert(path.clone());
                }
            }
            rule.allowed_cmds.push(allowed_cmd);
        }
        rule
    }
}

pub struct ParsedSudoers {
    pub rules: Vec<Rule>,
    pub user_aliases: HashMap<String, Vec<Uid>>,
}
