# ⛳ cargo-stable-public-api-lints

`cargo-stable-public-api-lints` is home to a few opinionated lints
for producing stable public APIs.  These are:

- are all public enums `#[non_exhaustive]`?
- are all public structs `#[non_exhaustive]`?
- do all types explicitly `impl Drop`?

This is built on top of the rustdoc JSON output and so requires
a nightly toolchain to be installed.

# Example output
```shell
$ cargo install --locked cargo-stable-public-api-lints
$ cargo stable-public-api-lints
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
------------------------------------------------------------
Marking a struct `#[non_exhaustive]` means future changes can add new fields.

Annotate this type with `#[cfg_attr(spal, allow(spal::exhaustive_struct))]` to ignore this lint
------------------------------------------------------------

⚠️ enum `pub enum drop::CopyEnum` should be marked #[non_exhaustive]
  at src/lib.rs:21
(...)
```
