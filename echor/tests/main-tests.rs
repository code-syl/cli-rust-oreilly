use assert_cmd::Command;
use predicates::prelude::*;

const EXECUTABLE: &str = "echor";
const TEXT_ARGS: [&str; 4] = ["Hello", "World", "Goodbye", "World"];
const FLAGS: [&str; 1] = ["-n"];
const NEWLINE: &str = "\n";
const SEPARATOR: &str = " ";

#[test]
fn exits_with_no_args() {
    // Arrange
    let mut command: Command = Command::cargo_bin(EXECUTABLE).unwrap();

    // Assert
    command.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn runs_with_one_text_arg() {
    // Arrange
    let mut command: Command = Command::cargo_bin(EXECUTABLE).unwrap();
    let expected: String = TEXT_ARGS[0].to_string() + NEWLINE;
    command.arg(TEXT_ARGS[0]);

    // Assert
    command.assert()
        .success()
        .stdout(expected);
}

#[test]
fn runs_with_multiple_text_args() {
    // Arrange
    let mut command: Command = Command::cargo_bin(EXECUTABLE).unwrap();
    let expected: String = TEXT_ARGS.join(SEPARATOR) + NEWLINE;
    command.args(&TEXT_ARGS);

    // Assert
    command.assert()
        .success()
        .stdout(expected);
}

#[test]
fn omits_newline_with_flag() {
    // Arrange
    let mut command: Command = Command::cargo_bin(EXECUTABLE).unwrap();
    let expected: String = TEXT_ARGS.join(SEPARATOR);
    command.args([FLAGS[0], TEXT_ARGS.join(SEPARATOR).as_str()]);

    // Assert
    command.assert()
        .success()
        .stdout(expected);
}