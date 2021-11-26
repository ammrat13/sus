use super::sudoers_type::{Sudoers, User};
use super::Verifier;
use std::fs::File;
use std::io::BufReader;
use users::{get_group_by_name, get_user_by_name};

#[allow(dead_code)]
pub fn from_sudoers() -> Vec<Verifier> {
    let file = File::open("sudoers.json").unwrap();
    let reader = BufReader::new(file);
    let u: Sudoers = serde_json::from_reader(reader).unwrap();
    for user_spec in u.user_specs {
        let list = user_spec.user_list;
        for user in list {
            println!("{:#?}", user);
            match user {
                User::Username(x) => {
                    println!("{}", get_user_by_name(&x).unwrap().uid());
                }
                User::Usergroup(x) => {
                    println!("{}", get_group_by_name(&x).unwrap().gid());
                }
                User::Useralias(x) => {
                    for aliased_user in &u.user_aliases[&x] {
                        if let User::Username(username) = aliased_user {
                            let retrieved_user = get_user_by_name(username);
                            match retrieved_user {
                                Some(x) => {
                                    println!("{}", x.uid());
                                }
                                None => {
                                    println!("User {} does not exist", username);
                                }
                            }
                        };
                    }
                } // TODO: Error handling
            }
        }
    }
    Vec::new()
}
