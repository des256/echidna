# sys-sys

Interface to underlying C or Objective-C APIs. All referred libraries become available from one local crate.

## Usage

First run `make.sh` to generate `src/bindings.rs` according to the includes mentioned in `wrapper.h`. Once `bindings.rs` exists, compile the crate normally with `cargo build`.

Note that `bindings.rs` is different for different platforms.

`build.rs` makes certain that `cargo build` will also link with the necessary libraries.
