# `sudo` in UserSpace

This project was created for the Fall 2021 Semester of CS 3210 at Georgia Tech.
It is a re-implementation of `sudo` that experiments with moving as much
computation off the `root` user as possible. That is, it aims to have most of
the code in unprivileged binaries, with those programs only `exec`ing to
escalate privilege when absolutely necessary.

See also the
* [Proposal](https://docs.google.com/document/d/17gTYkPF2tVHxffUQAEohYeq5u5XpvCrTnsoOFiNkf6c/edit?usp=sharing)
* [Final Report](https://docs.google.com/document/d/1k9ENTLjWKn68N2SxBkzi5DJK2EWbTG-0JbeMDbRMtxE/edit?usp=sharing)


## Obtaining a Copy

### Downloading from Releases

The binaries and documentation for this project can be obtained from the
[Releases](https://github.com/ammrat13/sus/releases) section. The versions
present there were compiled with all features enabled. If this is undesireable,
it is possible to compile the binary manually.

### Compiling

#### Build Environment

This project has a [Docker container](https://hub.docker.com/repository/docker/ammrat13/sus)
with the environment necesary to build this project. Simply clone this
repository and mount it as a volume at `/usr/local/src/sus/` - the initial
working directory.

Alternatively, it is possible to set up the environment manually. Like most Rust
projects, `sus` uses `cargo` as a build system. The container comes with version
`1.56.0`. It also uses the `rustfmt` and `clippy` components for formatting and
linting respectively, as well as `cargo-audit` version `0.15.2` to check for
known vulnerable dependencies. To automate the build process, `sus` also uses
`cargo-make` version `0.35.5`.

#### Build Process

Once the environment is set up, `sus` can be built with
```
$ cargo make sus-dist-build
```
This will compile the project with the default features, unlike the Releases
page which compiles with all features.

At the moment, it's necessary to edit the `Makefile.toml` to select which
features to build. Under `[tasks.sus-dist-build]`, edit
```
env = { "CARGO_MAKE_CARGO_BUILD_TEST_FLAGS" = "" }
```
to instead contain the arguments to be passed to `cargo-build`.

#### Installation Process

The `Makefile.toml` has an `install` target. It automatically builds the
application and installs it to the configured location. This can be invoked with
```
$ cargo make install
```

Alternatively, it is possible to manually install the project by copying over
the binaries from the `target/release/` directory and changing their
permissions.

### Configuring

A default configuration is given in the `config/` folder, and the builds on the
Releases page use it. However, the parameters in those files can be edited.


## Running

As this program is written in Rust, the binaries can be run like any other
program once they are compiled.

The arguments to the `sus-kernel` binary are defined in `config/sus-kernel.rs`
by all the parameters ending in `_IDX`. However, ideally the kernel should not
be run directly. Instead, a wrapper program should be used to parse user
arguments and massage them into the "computer-friendly" format used by the
kernel. Sadly, no such program exists at the moment.
