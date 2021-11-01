//! Build script for SUS
//!
//! We need to copy over the configuration module. We put it with all the other
//! configuration files for ease of use. However, we need it to be in `src/` to
//! actually use it in our code.

/// The original configuration file's path
const ORG: &str = "config/build.rs";
/// The path the configuration file should be copied to
const NEW: &str = "src/config.rs";

fn main() -> Result<(), std::io::Error> {
    // Only need to do this if the configuration has changed
    // Overwrite any changes to the copy in `src/` as well
    println!("cargo:rerun-if-changed={}", ORG);
    println!("cargo:rerun-if-changed={}", NEW);

    // Copy it over
    std::fs::copy(ORG, NEW)?;
    // Done
    Ok(())
}
