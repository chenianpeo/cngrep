use crate::cli::{_args, args};
use crate::config::{Args, InputSource};
use crate::error::Error;
use crate::matcher::search;
use crate::printer::output;
use crate::reader::{FileReader, Reader};

pub fn run() -> Result<(), Error> {
    // conduct arguments
    let cli_args = args()?;

    // read file and content
    let read_result = cli_args.read()?;

    // content search
    let search_result = search(cli_args, read_result)?;

    // result output
    output(search_result);

    Ok(())
}

pub fn _run() -> Result<(), Error> {
    let cli = _args()?;
    let args = Args::from_cli(cli)?;

    println!("{:#?}", args);

    let input_source = args.input_source;
    if let InputSource::File(d) = input_source {
        let path = d;
        let file = FileReader { path };
        let a = file.read()?;
        println!("{:?}", a);
    }

    Ok(())
}
