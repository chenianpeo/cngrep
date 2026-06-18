use crate::cli::args;
use crate::config::{Args, InputSource, MatchResult};
use crate::error::Error;
use crate::reader::{FileReader, Reader};

pub fn run() -> Result<(), Error> {
    use std::io::BufRead;
    let cli = args()?;

    let args = Args::from_cli(cli)?;

    match args.input_source {
        InputSource::File(path) => {
            let file = FileReader { path };
            let content = file.read()?;

            let mut search_result: Vec<MatchResult> = Vec::new();

            for (line_no, line) in content.lines().enumerate() {
                let line = line?;
                let line_no = line_no + 1;

                if line.contains(&args.query.to_string()) {
                    search_result.push(MatchResult {
                        path: "path".to_string(),
                        line_no,
                        content: line,
                    })
                }
            }

            println!("{:#?}", search_result);
        }
        _ => println!("undone"),
    }

    Ok(())
}
