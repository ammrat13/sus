use crate::executable::Executable;
use crate::permission::verify::VerifyResult;
use crate::permission::Permission;

use super::sudoers_type::{Sudoers, User};
use super::Verifier2;
use nix::unistd::{Gid, Uid};
use std::ffi::CString;
use std::fs::File;
use std::io::BufReader;
use users::{get_group_by_name, get_user_by_name};

#[allow(dead_code)]
#[derive(Debug)]
struct Policy {
    users: Vec<Uid>,
    groups: Vec<Gid>,
    commands: Vec<CString>,
}

#[allow(dead_code)]
pub fn from_sudoers() -> Vec<Box<Verifier2>> {
    let file = File::open("sudoers.json").unwrap();
    let reader = BufReader::new(file);
    let u: Sudoers = serde_json::from_reader(reader).unwrap();
    let mut verifiers = Vec::new();
    for user_spec in u.user_specs {
        let mut policy = Policy {
            users: Vec::new(),
            groups: Vec::new(),
            commands: Vec::new(),
        };

        let list = user_spec.user_list;
        for user in list {
            match user {
                User::Username(x) => {
                    if let Some(user) = get_user_by_name(&x) {
                        policy.users.push(Uid::from_raw(user.uid()))
                    }
                }
                User::Usergroup(x) => {
                    if let Some(group) = get_group_by_name(&x) {
                        policy.groups.push(Gid::from_raw(group.gid()))
                    }
                }
                User::Useralias(x) => {
                    for alias in &u.user_aliases[&x] {
                        if let User::Username(x) = alias {
                            if let Some(group) = get_group_by_name(&x) {
                                policy.groups.push(Gid::from_raw(group.gid()))
                            }
                        }
                    }
                }
            }
        }
        let x = Box::new( |curr_perm, req_perm, exe| -> VerifyResult { Ok(()) });
        verifiers.push(x);

    }
    return verifiers;
}

fn returns_closure() -> Box<dyn FnMut(i32) -> i32> {
    let x = Box::new(|x| x + 1);
    return x;
}
