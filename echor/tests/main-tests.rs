use assert_cmd::Command;
use predicates::prelude::*;

const EXECUTABLE: &str = "echor";
const TEXT_ARGS: [&str; 4] = ["Hello", "World", "Goodbye", "World"];
const FLAGS: [&str; 1] = ["-n"];
const NEWLINE: &str = "\n";
const SEPARATOR: &str = " ";

type TestResult = Result <(), Box<dyn std::error::Error>>;

#[test]
fn exits_with_no_args() -> TestResult {
    // Arrange

    // Act
    let mut command: Command = Command::cargo_bin(EXECUTABLE)?;

    // Assert
    command.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
    
    return Ok(());
}

#[test]
fn runs_with_one_text_arg() -> TestResult {
    let expected: String = TEXT_ARGS[0].to_string() + NEWLINE;
    
    return 
        run(&[TEXT_ARGS[0]], expected);
}

#[test]
fn runs_with_multiple_text_args() -> TestResult {
    let expected: String = TEXT_ARGS.join(SEPARATOR) + NEWLINE;

    return 
        run(&TEXT_ARGS, expected);
}

#[test]
fn omits_newline_with_flag() -> TestResult{
    let expected: String = TEXT_ARGS.join(SEPARATOR);

    return 
        run(&[&FLAGS[0], &TEXT_ARGS.join(SEPARATOR).as_str()], expected);
}

/* =================== Helper function =================== */

fn run(args: &[&str], expected_result: String) -> TestResult {
    Command::cargo_bin(EXECUTABLE)?
        .args(args)
        .assert()
        .success()
        .stdout(expected_result);

    return Ok(());
}