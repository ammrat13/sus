use super::Policy;
use super::PolicyFactoryError;
use super::PolicyFactoryResult;

use nix::unistd;

#[allow(dead_code)]
pub fn from_sudoers() -> PolicyFactoryResult {
  // TODO: 
    // Read policy from config
    // create policy structs
    // Return an OK of Policy structs array
  OK( Policy {
    // TODO: Fill this out
  })
}