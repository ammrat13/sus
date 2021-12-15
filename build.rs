//! Build script for SUS
//!
//! We need to copy over the configuration files. We put them with all the other
//! configuration files for ease of use. However, we need them to be in `src/`
//! to actually use it in our code.

/// A structure describing set of files to copy
struct CopySet {
    old: &'static str,
    new: &'static str,
}

/// The list of copies to perform
const COPIES: &[CopySet] = &[
    CopySet {
        old: "config/sus-kernel.rs",
        new: "src/bin/sus-kernel/config.rs",
    },
    CopySet {
        old: "config/sus.rs",
        new: "src/bin/sus/config.rs",
    },
];

fn main() -> Result<(), std::io::Error> {
    // Do every copy
    for copy in COPIES {
        // Only rerun if the configurations have changed
        println!("cargo:rerun-if-changed={}", copy.old);
        println!("cargo:rerun-if-changed={}", copy.new);
        // Do the copy
        std::fs::copy(copy.old, copy.new)?;
    }

    // Done successfully
    Ok(())
}
