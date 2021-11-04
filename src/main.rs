//! SUS kernel
//!
//! This is an experiment with making as much of `sudo` run unprivileged as
//! possible. The goal is to run "Sudo in UserSpace." As such, this binary is
//! the SUS "kernel," which does the minimal amount of work required to have a
//! working `sudo`.

mod config;
mod executable;

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
}
