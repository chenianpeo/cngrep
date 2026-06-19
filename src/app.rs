use crate::cli::args;
use crate::config::Args;
use crate::config::InputSource;
use crate::config::Mode;
use crate::error::Error;
use crate::matcher::FileMatch;
use crate::matcher::Match;
use crate::reader::FileReader;
use crate::reader::Reader;

pub fn run() -> Result<(), Error> {
    let arg: Vec<String> = std::env::args().collect();
    let cli = args(arg)?;
    println!("{:?}", cli);

    let args = Args::from_cli(cli)?;
    match args.mode {
        Mode::Normal => match args.input_source {
            InputSource::File(path) => {
                let file = FileReader { path };
                let content = file.read()?;

                let mut file_match = FileMatch {
                    query: args.query,
                    file: content,
                };

                let search_result = file_match.search_normal()?;
                println!("{:#?}", search_result);
            }
            InputSource::Stdin => {
                println!("stdin");
            }
            InputSource::CurrentDir => {
                println!("current dir")
            }
        },
        Mode::CountOnly => match args.input_source {
            InputSource::File(path) => {
                let file = FileReader { path };
                let content = file.read()?;

                let mut file_match = FileMatch {
                    query: args.query,
                    file: content,
                };

                let search_result = file_match.search_count_only()?;
                println!("{:#?}", search_result);
            }
            InputSource::Stdin => {
                println!("stdin");
            }
            InputSource::CurrentDir => {
                println!("current dir")
            }
        },
    }

    Ok(())
}
