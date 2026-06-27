/*
this is the entry point

mvp version end is 0.1.0
current stage unfinished: error handle is weak and not have test
next stage: refactor and design standard running flow
*/
fn main() {
    let stats = cngrep::app::run();

    match stats {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
}
