use crate::cli::args;
use crate::config::Args;
use crate::config::Mode;
use crate::error::Error;

pub fn run() -> Result<(), Error> {
    let arg: Vec<String> = std::env::args().collect();
    let cli = args(arg)?;
    println!("{:?}", cli);

    let args = Args::from_cli(cli)?;

    match args.mode {
        Mode::Normal => {
            println!("running by normal mode");
            
        }

        Mode::CountOnly => {
            println!("running by count only mode");
        }
    }

    // match args.input_source {
    //     InputSource::File(path) => {
    //         let file = FileReader { path };

    //         let content = file.read()?;

    //         let mut file_match = FileMatch {
    //             query: args.query,
    //             file: content,
    //         };

    //         let search_result = file_match.search()?;
    //         println!("{:#?}", search_result);
    //     }
    //     InputSource::Stdin => {
    //         println!("stdin");
    //     }
    //     InputSource::CurrentDir => {
    //         println!("current dir")
    //     }
    // };

    Ok(())
}
