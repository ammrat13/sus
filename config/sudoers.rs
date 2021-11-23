use std::ffi::CString;

pub enum Option { SETENV, AUTHENTICATE }

pub enum PolicyString {
  All,
  from(CString)
}

pub struct CmdSpec {
  runas_users: Vec<PolicyString>,
  runas_groups: Vec<PolicyString>,
  options: Vec<Option>,
  commands: Vec<PolicyString>
}

pub struct Policy {
  users: Vec<PolicyString>,
  hosts: Vec<PolicyString>,
  cmd_specs: Vec<CmdSpec> 
}

// Command spec for root user. If a Policy struct uses this spec, the users specified in 
// the policy can run any commands as root
const ROOT_CMD_SPEC: CmdSpec = CmdSpec {
  runas_users: vec![PolicyString::All],
  runas_groups: vec![PolicyString::All],
  options: vec![Option::SETENV],
  cmd_specs: vec![PolicyString::All]
};

// Policy definition for root, same as `root    ALL=(ALL) ALL` in /etc/sudoers
const ROOT_POLICY: Policy = Policy {
  users: vec![PolicyString::All],
  hosts: vec![PolicyString::All],
  cmd_specs: vec![ROOT_CMD_SPEC]
};

// Policy definition for luke, same as `luke   ALL=(ALL) ALL` in /etc/sudoers
const POLICY_A: Policy = Policy {
  users: vec![PolicyString::from("luke")],
  hosts: vec![PolicyString::All],
  cmd_specs: vec![ROOT_CMD_SPEC]
};

pub const POLICIES: Vec<Policy> = vec![ROOT_POLICY, POLICY_A];