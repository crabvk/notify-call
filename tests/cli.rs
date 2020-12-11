use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use std::{error, path, str};

fn call(args: Vec<&str>) -> Command {
    let mut cmd = Command::cargo_bin("notify-call").unwrap();
    cmd.args(args);
    cmd
}

#[test]
fn show_simple_n11() {
    call(vec!["hello", "simple"]).assert().success();
}

#[test]
fn show_and_print_id() {
    let is_nid = predicate::function(|o: &str| o[..o.len() - 1].parse::<u32>().is_ok());
    call(vec!["-p", "hello", "print id"])
        .assert()
        .stdout(is_nid);
}

#[test]
fn show_with_urgency_and_expire() {
    call(vec!["-u", "critical", "-t", "2000", "hello", "critical"])
        .assert()
        .success();
}

#[test]
fn show_with_app_name_and_icon() {
    call(vec![
        "-a",
        "notify-call-test",
        "-i",
        "firefox-developer-edition",
        "hello",
        "icon",
    ])
    .assert()
    .success();
}

#[test]
fn show_with_hint() {
    call(vec!["-h", "int:value:50", "hello progress"])
        .assert()
        .success();
}

#[test]
fn show_with_actions() {
    call(vec![
        "-d",
        "echo default-action",
        "-o",
        "echo action1:one",
        "-o",
        "echo action2:two",
        "hello",
        "actions",
    ])
    .assert()
    .success();
}

#[test]
fn show_and_replace() -> Result<(), Box<dyn error::Error>> {
    let output = call(vec!["-p", "hello", "to be replaced by id"])
        .output()?
        .stdout;
    let nid: u32 = str::from_utf8(&output)?.trim().parse()?;
    call(vec![
        "-r",
        nid.to_string().as_str(),
        "hello",
        "replaced by id",
    ])
    .assert()
    .success();
    Ok(())
}

#[test]
fn show_and_replace_with_file() {
    call(vec!["-R", "file1", "hello", "to be replaced with file"])
        .assert()
        .success();

    let path = dirs::runtime_dir().unwrap().join(path::Path::new("file1"));
    assert!(path.exists());
    assert!(path.is_file());

    call(vec!["-R", "file1", "hello", "replaced with file"])
        .assert()
        .success();
}
