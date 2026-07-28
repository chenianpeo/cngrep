use std::process::Command;

fn run_cg(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_cg"))
        .args(args)
        .output()
        .expect("failed to execute cg")
}

#[test]
fn search_file() {
    let output = run_cg(&["t", "/home/cn/Code/cngrep/content.txt"]);
    println!("{:?}", output);
    assert!(output.status.success());
}
