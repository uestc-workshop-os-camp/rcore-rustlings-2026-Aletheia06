use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn cicvverify() {
    Command::new(env!("CARGO_BIN_EXE_rustlings"))
        .args(&["--nocapture", "cicvverify"]) 
        // .current_dir("exercises")
        .assert()
        .success();
}
