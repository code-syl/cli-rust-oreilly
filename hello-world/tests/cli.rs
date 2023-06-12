use assert_cmd::Command;

const EXECUTABLE_NAME: &str = "hello-world";

#[test]
fn runs () {
    let mut command: Command = Command::cargo_bin(EXECUTABLE_NAME).unwrap();
    command.assert().success();
}