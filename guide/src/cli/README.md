# Command Line Tool

moenarchbook can be used either as a command line tool or a [Rust
crate](https://crates.io/crates/moenarchbook). Let's focus on the command line tool
capabilities first.

## Install From Binaries

Precompiled binaries are provided for major platforms on a best-effort basis.
Visit [the releases page](https://github.com/rust-lang/moenarchbook/releases)
to download the appropriate version for your platform.

## Install From Source

moenarchbook can also be installed by compiling the source code on your local machine.

### Pre-requisite

moenarchbook is written in **[Rust](https://www.rust-lang.org/)** and therefore needs
to be compiled with **Cargo**. If you haven't already installed Rust, please go
ahead and [install it](https://www.rust-lang.org/tools/install) now.

### Install Crates.io version

Installing moenarchbook is relatively easy if you already have Rust and Cargo
installed. You just have to type this snippet in your terminal:

```bash
cargo install moenarchbook
```

This will fetch the source code for the latest release from
[Crates.io](https://crates.io/) and compile it. You will have to add Cargo's
`bin` directory to your `PATH`.

Run `moenarchbook help` in your terminal to verify if it works. Congratulations, you
have installed moenarchbook!


### Install Git version

The **[git version](https://github.com/moenarch/moenarchbook)** contains all
the latest bug-fixes and features, that will be released in the next version on
**Crates.io**, if you can't wait until the next release. You can build the git
version yourself. Open your terminal and navigate to the directory of you
choice. We need to clone the git repository and then build it with Cargo.

```bash
git clone --depth=1 https://github.com/moenarch/moenarchbook.git
cd moenarchbook
cargo build --release
```

The executable `moenarchbook` will be in the `./target/release` folder, this should be
added to the path.
