//! SUS Kernel
//!
//! This is an experiment with making as much of `sudo` run unprivileged as
//! possible. The goal is to run "Sudo in UserSpace." As such, this binary is
//! the SUS "kernel," which does the minimal amount of work required to have a
//! working `sudo`.

mod config;
mod executable;
mod log;
mod permission;
mod request;

use permission::verify::AbstractVerifier;
use request::Request;

/// Main method for the kernel
///
/// This is the main method for the SUS kernel. As is standard practice in Rust,
/// most of the work is done by internal libraries, and this function simply
/// calls into those libraries.
///
/// Note that this function does not return a [Result]. This is intentional. We
/// want this function to just panic and not print any debugging output.
fn main() {
    // Set up a panic handler
    // This way, we don't give any information
    std::panic::set_hook(Box::new(|_| {
        std::process::exit(1);
    }));

    // Get the executable to run
    let executable = config::EXECUTABLE_FACTORY().unwrap();
    // Get the current and requested permissions
    let current_permissions = config::CURRENT_PERMISSION_FACTORY().unwrap();
    let requested_permissions = config::REQUESTED_PERMISSION_FACTORY().unwrap();
    // Put the runner in a box
    // Do the same with the logger
    let runner = Box::new(config::RUNNER);
    let logger = Box::new(config::LOGGER);

    // Create the verifiers
    // We need to clone them from the slice reference
    let verifiers = {
        // Do the clone
        let mut vfers = Vec::new();
        vfers.extend_from_slice(config::VERIFIERS);
        // Create and return
        // Box everything up as well
        // See: https://newbedev.com/how-to-create-a-vector-of-boxed-closures-in-rust
        vfers
            .into_iter()
            .map(|f| Box::new(f) as Box<AbstractVerifier>)
            .collect()
    };

    let req = Request {
        executable,
        current_permissions,
        requested_permissions,
        verifiers,
        runner,
        logger,
    };
    req.service().unwrap();
}
