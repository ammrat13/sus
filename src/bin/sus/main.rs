mod option;

use option::CommandLineOptions;
use option::Options;
use structopt::StructOpt;

fn main() {
    let command_line_opts = CommandLineOptions::from_args();
    let opts = Options::parse_options_like(command_line_opts);
    println!("{:?}", opts);
}
