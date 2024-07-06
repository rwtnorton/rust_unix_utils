use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_no_args_no_omit_newline() -> TestResult {
    let mut cmd = Command::cargo_bin("echo_rust")?;
    cmd.assert().success();
    let output = cmd.output()?;
    let got = String::from_utf8(output.stdout)?;
    assert_eq!(got, "\n");
    Ok(())
}


#[test]
fn test_no_args_and_omit_newline() -> TestResult {
    let mut base_cmd = Command::cargo_bin("echo_rust")?;
    let cmd = base_cmd.arg("-n");
    cmd.assert().success();
    let output = cmd.output()?;
    let got = String::from_utf8(output.stdout)?;
    assert_eq!(got, "");
    Ok(())
}


#[test]
fn test_with_args_no_omit_newline() -> TestResult {
    let mut base_cmd = Command::cargo_bin("echo_rust")?;
    let cmd = base_cmd.arg("Hello").arg("there!");
    cmd.assert().success();
    let output = cmd.output()?;
    let got = String::from_utf8(output.stdout)?;
    assert_eq!(got, "Hello there!\n");
    Ok(())
}

#[test]
fn test_with_args_and_omit_newline() -> TestResult {
    let mut base_cmd = Command::cargo_bin("echo_rust")?;
    let cmd = base_cmd.arg("-n").arg("Hello").arg("there!");
    cmd.assert().success();
    let output = cmd.output()?;
    let got = String::from_utf8(output.stdout)?;
    assert_eq!(got, "Hello there!");
    Ok(())
}

#[test]
fn test_with_spaced_args_no_omit_newline() -> TestResult {
    let mut base_cmd = Command::cargo_bin("echo_rust")?;
    let cmd = base_cmd.arg("Hello  there!");
    cmd.assert().success();
    let output = cmd.output()?;
    let got = String::from_utf8(output.stdout)?;
    assert_eq!(got, "Hello  there!\n");
    Ok(())
}
