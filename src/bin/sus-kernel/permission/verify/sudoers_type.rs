use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::CString;

#[derive(Deserialize, Serialize, Debug)]
pub enum User {
    #[serde(rename = "username")]
    Username(String),
    #[serde(rename = "usergroup")]
    Usergroup(String),
    #[serde(rename = "useralias")]
    Useralias(String),
}
#[derive(Deserialize, Serialize, Debug)]
pub enum Host {
    #[serde(rename = "hostname")]
    Hostname(String),
}
#[derive(Deserialize, Serialize, Debug)]
pub enum Option {
    #[serde(rename = "setenv")]
    Setenv(bool),
    #[serde(rename = "authenticate")]
    Authenticate(bool),
}
#[derive(Deserialize, Serialize, Debug)]
pub enum Command {
    #[serde(rename = "command")]
    Command(CString),
}
#[derive(Deserialize, Serialize, Debug)]
pub struct CmdSpec {
    #[serde(rename = "runasusers")]
    pub run_as_users: Vec<User>,
    #[serde(rename = "runasgroups")]
    #[serde(default)]
    pub run_as_groups: Vec<User>,
    #[serde(rename = "Options")]
    pub options: Vec<Option>,
    #[serde(rename = "Commands")]
    pub commands: Vec<Command>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct UserSpec {
    #[serde(rename = "User_List")]
    pub user_list: Vec<User>,
    #[serde(rename = "Host_List")]
    pub host_list: Vec<Host>,
    #[serde(rename = "Cmnd_Specs")]
    pub cmd_specs: Vec<CmdSpec>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sudoers {
    #[serde(rename = "User_Aliases")]
    pub user_aliases: HashMap<String, Vec<User>>,
    #[serde(rename = "User_Specs")]
    pub user_specs: Vec<UserSpec>,
}
