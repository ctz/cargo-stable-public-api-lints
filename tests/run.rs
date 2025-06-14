use std::mem;
use std::process::{Command, Output};

#[test]
fn non_exhaustive_enum_fixed() {
    insta::assert_debug_snapshot!(run(
        "tests/non_exhaustive/",
        "exhaustive-enum",
        "enum_fixed"
    ));
}

#[test]
fn non_exhaustive_enum_warn() {
    insta::assert_debug_snapshot!(run("tests/non_exhaustive/", "exhaustive-enum", "enum_warn"));
}

#[test]
fn non_exhaustive_enum_allow() {
    insta::assert_debug_snapshot!(run(
        "tests/non_exhaustive/",
        "exhaustive-enum",
        "enum_allow"
    ));
}

#[test]
fn non_exhaustive_struct_fixed() {
    insta::assert_debug_snapshot!(run(
        "tests/non_exhaustive/",
        "exhaustive-struct",
        "struct_fixed"
    ));
}

#[test]
fn non_exhaustive_struct_warn() {
    insta::assert_debug_snapshot!(run(
        "tests/non_exhaustive/",
        "exhaustive-struct",
        "struct_warn"
    ));
}

#[test]
fn non_exhaustive_struct_allow() {
    insta::assert_debug_snapshot!(run(
        "tests/non_exhaustive/",
        "exhaustive-struct",
        "struct_allow"
    ));
}

#[test]
fn drop_struct_warn() {
    insta::assert_debug_snapshot!(run("tests/drop/", "drop", "struct_warn"));
}

#[test]
fn sorted_enum() {
    insta::assert_debug_snapshot!(run("tests/sorted_enum/", "sorted-enum", "default"));
}

fn run(path: &str, lint_id: &str, feature: &str) -> Output {
    let mut output = Command::new("cargo")
        .arg("-q")
        .arg("run")
        .arg("--")
        .arg("--features")
        .arg(feature)
        .arg("--allow")
        .arg("all")
        .arg("--warn")
        .arg(lint_id)
        .arg(format!("{path}/Cargo.toml"))
        .output()
        .unwrap();

    // currently the rustdoc subprocess writes unstable output
    // to stderr
    println!(
        "stderr: {}",
        String::from_utf8(mem::take(&mut output.stderr)).expect("stderr is not valid utf8")
    );
    println!(
        "stdout:\n{}",
        String::from_utf8(output.stdout.clone()).expect("stdout is not valid utf8")
    );
    output
}
