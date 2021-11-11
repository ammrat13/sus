pub mod sudoers;
pub use sudoers::from_sudoers;

use super::Policy;

pub type AutoPolicyFactory = fn() -> PolicyFactoryResult;
pub type PolicyFactoryResult = Result<Policy, PolicyFactoryError>;

#[derive(Debug)]
pub enum PolicyFactoryError {
    /// Malformed Policy config
    PolicyMalformed {content: String},
}
