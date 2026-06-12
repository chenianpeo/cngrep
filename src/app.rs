use crate::cli::args;
use crate::error::Error;
use crate::matcher::search;
use crate::printer::output;

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
