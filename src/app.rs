use crate::cli::args;
use crate::config::{Args, InputSource};
use crate::error::Error;
use crate::matcher::{FileMatch, Match};
use crate::reader::{FileReader, Reader};

pub fn run() -> Result<(), Error> {
    let cli = args()?;

    let args = Args::from_cli(cli)?;

    match args.input_source {
        InputSource::File(path) => {
            let file = FileReader { path };

            let content = file.read()?;

            let mut file_match = FileMatch {
                query: args.query,
                file: content,
            };

            let search_result = file_match.search()?;
            println!("{:#?}", search_result);
        }
        InputSource::Stdin => {
            println!("stdin");
        }
        InputSource::CurrentDir => {
            println!("current dir")
        }
    };

    Ok(())
}
