use std::fs::{self, remove_dir_all, remove_file};
use std::process::{Command, Stdio};
use std::{fs::File, io::Write};

fn new_file(path: &str, content: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

#[test]
fn smoke_test_stdin() {
    use crate::*;

    let cg = env!("CARGO_BIN_EXE_cg");

    let mut child = Command::new(cg)
        .arg("rust")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start cg");

    let stdin = child.stdin.as_mut().expect("failed to open stdin");

    stdin
        .write_all(b"hello world\nrust is good\nHello Rust")
        .expect("failed to write stdin");

    drop(child.stdin.take());

    let output = child.wait_with_output().expect("failed to wait for cg");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "2:rust is good\n");
}

#[test]
fn smoke_test_file() {
    use crate::*;

    let content = "hello world\nHello World";
    let path = "./tests/test_file.txt";
    new_file(path, content);

    let cg = env!("CARGO_BIN_EXE_cg");

    let output_normal = Command::new(cg)
        .args(["hello", path])
        .output()
        .expect("failed to execute cg file");

    let output_ignore = Command::new(cg)
        .args(["hello", path, "-i"])
        .output()
        .expect("failed to execute cg file ignore");

    let output_count = Command::new(cg)
        .args(["hello", path, "-c"])
        .output()
        .expect("failed to execute cg file count");

    let output_ignore_count = Command::new(cg)
        .args(["hello", path, "-c", "-i"])
        .output()
        .expect("failed to execute cg file ignore count");

    remove_file(path).expect("remove file error");

    assert!(output_normal.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output_normal.stdout),
        "1:hello world\n"
    );

    assert!(output_ignore.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output_ignore.stdout),
        "1:hello world\n2:Hello World\n"
    );

    assert!(output_count.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output_count.stdout),
        "./tests/test_file.txt: 1\n"
    );

    assert!(output_ignore_count.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output_ignore_count.stdout),
        "./tests/test_file.txt: 2\n"
    );
}

fn new_dir(path: &str, content: Vec<&str>) {
    fs::create_dir_all(path).unwrap();

    for (no, file) in content.iter().enumerate() {
        let path = &format!("{}/test_file_{}", path, no);
        new_file(path, file);
    }
}

#[test]
fn smoke_test_dir() {
    use crate::*;

    let path_1 = "tests/test_dir_1";
    let content_1_1 = "hello world\nHello World";
    let content_1_2 = "hello Rust\nHello rust";

    new_dir(path_1, vec![content_1_1, content_1_2]);

    let cg = env!("CARGO_BIN_EXE_cg");

    let output_normal = Command::new(cg)
        .args(["hello", path_1])
        .output()
        .expect("failed to execute cg file");

    let output_ignore = Command::new(cg)
        .args(["hello", path_1, "-i"])
        .output()
        .expect("failed to execute cg file ignore");

    let output_count = Command::new(cg)
        .args(["hello", path_1, "-c"])
        .output()
        .expect("failed to execute cg file count");

    let output_ignore_count = Command::new(cg)
        .args(["hello", path_1, "-c", "-i"])
        .output()
        .expect("failed to execute cg file ignore count");

    remove_dir_all(path_1).expect("remove dir error");

    assert!(output_normal.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output_normal.stdout),
        "tests/test_dir_1/test_file_0\n1:hello world\n\ntests/test_dir_1/test_file_1\n1:hello Rust\n"
    );

    assert!(output_ignore.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output_ignore.stdout),
        "tests/test_dir_1/test_file_0\n1:hello world\n2:Hello World\n\ntests/test_dir_1/test_file_1\n1:hello Rust\n2:Hello rust\n"
    );

    assert!(output_count.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output_count.stdout),
        "tests/test_dir_1/test_file_0: 1\ntests/test_dir_1/test_file_1: 1\n"
    );

    assert!(output_ignore_count.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output_ignore_count.stdout),
        "tests/test_dir_1/test_file_0: 2\ntests/test_dir_1/test_file_1: 2\n"
    );
}
