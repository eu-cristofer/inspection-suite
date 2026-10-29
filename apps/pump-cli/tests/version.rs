use std::process::Command;

#[test]
fn version_flag_prints_name_and_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_pump-cli")).arg("--version").output().expect("run pump-cli");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf-8 stdout");
    assert_eq!(stdout, format!("pump-cli {}\n", env!("CARGO_PKG_VERSION")));
}

#[test]
fn no_args_exits_with_code_2() {
    let output = Command::new(env!("CARGO_BIN_EXE_pump-cli")).output().expect("run pump-cli");
    assert_eq!(output.status.code(), Some(2));
}
