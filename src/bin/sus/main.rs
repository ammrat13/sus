mod option;

use std::error::Error;
use structopt::StructOpt;

use option::CommandLineOptions;
use option::Options;

fn main() -> Result<(), Box<dyn Error>> {
    let command_line_opts = CommandLineOptions::from_args();
    let opts = Options::parse_options_like(command_line_opts)?;
    println!("{:?}", opts.to_kernel_commandline()?);

    Ok(())
}
