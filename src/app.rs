use crate::cli::args;
use crate::config::{Args, InputSource};
use crate::error::Error;

use crate::reader::{Read, ReadDir, ReadFile, ReadStdin};

// Software Operation
pub fn run() -> Result<(), Error> {
    // arguments input and conduct
    let arg: Vec<String> = std::env::args().collect();
    let cli = args(arg)?;
    let args = Args::from_cli(cli)?;

    // match input source and mode
    let reader: Box<dyn Read> = match args.input_source {
        InputSource::File(path) => Box::new(ReadFile {
            query: args.query,
            path,
        }),
        InputSource::Stdin => Box::new(ReadStdin { query: args.query }),
        InputSource::CurrentDir => Box::new(ReadDir { query: args.query }),
    };

    // read content
    let mut content = reader.read()?;

    // query search
    let result = content.search()?;

    // result output
    result.print();

    Ok(())
}
