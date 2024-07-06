use assert_cmd::Command;

#[test]
fn test_no_args_no_omit_newline() {
    let mut cmd = Command::cargo_bin("echo_rust").unwrap();
    cmd.assert().success();
    let output = cmd.output().expect("fail");
    let got = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(got, "\n");
}


#[test]
fn test_no_args_and_omit_newline() {
    let mut base_cmd = Command::cargo_bin("echo_rust").unwrap();
    let cmd = base_cmd.arg("-n");
    cmd.assert().success();
    let output = cmd.output().expect("fail");
    let got = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(got, "");
}


#[test]
fn test_with_args_no_omit_newline() {
    let mut base_cmd = Command::cargo_bin("echo_rust").unwrap();
    let cmd = base_cmd.arg("Hello").arg("there!");
    cmd.assert().success();
    let output = cmd.output().expect("fail");
    let got = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(got, "Hello there!\n");
}

#[test]
fn test_with_args_and_omit_newline() {
    let mut base_cmd = Command::cargo_bin("echo_rust").unwrap();
    let cmd = base_cmd.arg("-n").arg("Hello").arg("there!");
    cmd.assert().success();
    let output = cmd.output().expect("fail");
    let got = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(got, "Hello there!");
}

#[test]
fn test_with_spaced_args_no_omit_newline() {
    let mut base_cmd = Command::cargo_bin("echo_rust").unwrap();
    let cmd = base_cmd.arg("Hello  there!");
    cmd.assert().success();
    let output = cmd.output().expect("fail");
    let got = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(got, "Hello  there!\n");
}
