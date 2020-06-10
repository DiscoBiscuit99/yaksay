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
fn fail_on_both_dead_and_bored()
    -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("yaksay")
        .expect("Binary exists...")
        .args(&["-d", "-b"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Assuming the yak is quantum, it collapsed to one state..."));
    Ok(())
}

#[test]
fn fail_on_both_dead_and_surprised()
    -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("yaksay")
        .expect("Binary exists...")
        .args(&["-d", "-s"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Assuming the yak is quantum, it collapsed to one state..."));
    Ok(())
}

#[test]
fn fail_on_both_bored_and_surprised()
    -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("yaksay")
        .expect("Binary exists...")
        .args(&["-b", "-s"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Assuming the yak is quantum, it collapsed to one state..."));
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

