use super::sudoers_type::Sudoers;
use super::{Verifier, VerifyError};
use crate::permission::verify::VerifyResult;
use nix::unistd::{Gid, Uid};
use std::ffi::CString;
use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
#[derive(Debug)]
struct Command {
    run_as_users: Vec<Uid>,
    run_as_groups: Vec<Gid>,
    commands: Vec<CString>,
}

#[derive(Debug)]
struct Policy {
    users: Vec<Uid>,
    groups: Vec<Gid>,
    cmd_specs: Vec<Command>,
}

#[allow(dead_code)]
pub fn from_sudoers() -> Vec<Box<Verifier>> {
    // Declare vector of verifiers to return
    let mut verifiers = Vec::new();
    // Parse sudoers.json using serde_json
    let file = File::open("sudoers.json").unwrap();
    let reader = BufReader::new(file);
    let sudoer: Sudoers = serde_json::from_reader(reader).unwrap();
    // Parse sudoer further and retrieve uids and gids
    let parsed_sudoer = sudoer.retrieve_ids();
    for rule in parsed_sudoer.rules {
        let x: Box<Verifier> = Box::new(move |curr_perm, req_perm, exe| -> VerifyResult {
            if rule.is_relevant(curr_perm) {
                for allowed_cmd in &rule.allowed_cmds {
                    if (allowed_cmd.is_relevant(req_perm) && allowed_cmd.paths.contains(&exe.path))
                        || allowed_cmd.allow_all_cmds
                    {
                        return Ok(());
                    }
                }
            }
            Err(VerifyError::NotAllowed)
        });
        verifiers.push(x);
    }
    verifiers
    // println!("curr_perm: {:?}\n", curr_perm);
    //         println!("req_perm: {:?}\n", req_perm);
    //         println!("exe: {:?}\n", exe);
    //         println!("rule: {:?}\n", rule);
}
