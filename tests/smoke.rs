use std::{
    io::Write,
    process::{Command, Stdio},
};

fn cg() -> Command {
    Command::new(env!("CARGO_BIN_EXE_cg"))
}

#[test]
fn stdin_search() {
    let mut child = cg()
        .arg("rust")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"hello world\nrust is good\nHello Rust\n")
        .unwrap();

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "rust is good\n");
}

#[test]
fn stdin_ignore_case() {
    let mut child = cg()
        .args(["rust", "-i"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"rust\nRUST\nRust\nhello\n")
        .unwrap();

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "rust\nRUST\nRust\n"
    );
}

#[test]
fn stdin_line_number() {
    let mut child = cg()
        .args(["rust", "-n"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"hello\nrust\nworld\nrust again\n")
        .unwrap();

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "2:rust\n4:rust again\n"
    );
}

#[test]
fn stdin_count() {
    let mut child = cg()
        .args(["rust", "-c"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"rust\nhello\nrust\nRust\n")
        .unwrap();

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "2\n");
}

#[test]
fn stdin_empty() {
    let mut child = cg()
        .arg("rust")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"hello\nworld\n")
        .unwrap();

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn help_output() {
    let output = cg().arg("--help").output().unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Usage"));
}

#[test]
fn version_output() {
    let output = cg().arg("--version").output().unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}
