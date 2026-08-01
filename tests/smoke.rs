use std::fs::{self, remove_dir_all};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn create_test_dir(name: &str) -> PathBuf {
    let mut path = std::env::current_dir().unwrap();

    path.push("tests");
    path.push(name);

    if path.exists() {
        fs::remove_dir_all(&path).unwrap();
    }

    fs::create_dir_all(&path).unwrap();

    path
}

fn create_test_files(path: &Path) {
    fs::write(
        path.join("content.txt"),
        "hello cngrep\n\
        ripgrep\n\
        bat or cat\n",
    )
    .unwrap();

    fs::write(
        path.join("rust.txt"),
        "rust\n\
        linux\n\
        cargo\n",
    )
    .unwrap();

    fs::write(path.join("empty.txt"), "").unwrap();

    let dir = path.join("dir");

    fs::create_dir_all(dir.join("subdir")).unwrap();

    fs::write(
        dir.join("a.txt"),
        "hello\n\
        rust\n",
    )
    .unwrap();

    fs::write(
        dir.join("b.txt"),
        "hello\n\
        opensuse\n",
    )
    .unwrap();

    fs::write(
        dir.join("subdir").join("c.txt"),
        "rust\n\
        system\n",
    )
    .unwrap();
}

fn cg() -> Command {
    Command::new(env!("CARGO_BIN_EXE_cg"))
}

#[test]
fn smoke_stdin() {
    let free_h = Command::new("free")
        .arg("-h")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout
        .take()
        .unwrap();

    let output_normal = cg().arg("Mem").stdin(free_h).output().unwrap();

    assert!(output_normal.status.success());
    assert!(String::from_utf8_lossy(&output_normal.stdout).contains("Mem"));

    let free_h = Command::new("free")
        .arg("-h")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout
        .take()
        .unwrap();

    let output_count = cg().arg("-c").arg("Mem").stdin(free_h).output().unwrap();

    assert!(output_count.status.success());
    assert!(String::from_utf8_lossy(&output_count.stdout).contains('1'));
}

#[test]
fn smoke_file() {
    let path = create_test_dir("test_file");
    create_test_files(&path);

    let file = path.join("content.txt");

    let output_normal = cg().arg("hello").arg(&file).output().unwrap();

    assert!(output_normal.status.success());
    assert!(String::from_utf8_lossy(&output_normal.stdout).contains("hello"));

    let output_count = cg().arg("-c").arg("hello").arg(&file).output().unwrap();

    assert!(output_count.status.success());
    assert!(String::from_utf8_lossy(&output_count.stdout).contains('1'));

    remove_dir_all(&path).unwrap();
}

#[test]
fn smoke_dir() {
    let path = create_test_dir("test_dir");
    create_test_files(&path);

    let output_normal = cg().arg("hello").arg(&path).output().unwrap();

    assert!(output_normal.status.success());
    assert!(String::from_utf8_lossy(&output_normal.stdout).contains("hello"));

    let output_count = cg().arg("hello").arg("-c").arg(&path).output().unwrap();

    assert!(output_count.status.success());
    assert!(String::from_utf8_lossy(&output_count.stdout).contains('3'));

    remove_dir_all(&path).unwrap();
}
