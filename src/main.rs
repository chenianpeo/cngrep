/*
this is the entry point
*/

// Exit Code and Error Handle module
use std::process::ExitCode;

fn main() -> ExitCode {
    let status = cngrep::app::run();

    match status {
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(1)
        }

        Ok(_) => ExitCode::from(0),
    }
}
