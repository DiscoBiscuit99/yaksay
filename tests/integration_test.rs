extern crate assert_cmd;

use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn run_with_defaults()
    -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("yaksay")
        .expect("Binary exists...")
        .assert()
        .success()
        .stdout(predicate::str::contains("Mooh!"));
    Ok(())
}

#[test]
fn fail_on_non_existing_file()
    -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("yaksay")
        .expect("Binary exists...")
        .args(&["-f", "no/such/file.txt"])
        .assert()
        .failure();
    Ok(())
}
