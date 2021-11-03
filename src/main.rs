//! SUS kernel
//!
//! This is an experiment with making as much of `sudo` run unprivileged as
//! possible. The goal is to run "Sudo in UserSpace." As such, this binary is
//! the SUS "kernel," which does the minimal amount of work required to have a
//! working `sudo`.

mod config;
mod lib;

/// Main method for the kernel
///
/// This is the main method for the SUS kernel. As is standard practice in Rust,
/// most of the work is done by an internal library, and this function simply
/// calls into that library.
fn main() {
    println!("Hello, world!");
}
