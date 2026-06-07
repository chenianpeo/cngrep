fn main() {
    let stats = cngrep::app::run();

    match stats {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
}