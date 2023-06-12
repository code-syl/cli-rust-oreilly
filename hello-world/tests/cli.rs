use assert_cmd::Command;

const HELLO_WORLD_EXECUTABLE: &str = "hello-world";
const TRUE_EXECUTABLE: &str = "true";
const FALSE_EXECUTABLE: &str = "false";

#[test]
fn runs() {
    let mut command: Command = Command::cargo_bin(HELLO_WORLD_EXECUTABLE).unwrap();
    command.assert().success();
}

#[test]
fn true_ok() {
    let mut command: Command = Command::cargo_bin(TRUE_EXECUTABLE).unwrap();
    command.assert().success();
}

#[test]
fn false_not_ok() {
    let mut command: Command = Command::cargo_bin(FALSE_EXECUTABLE).unwrap();
    command.assert().failure();
}

#[test]
fn output() {
    let mut command: Command = Command::cargo_bin(HELLO_WORLD_EXECUTABLE).unwrap();
    command.assert().stdout("Hello, world!\n");
}