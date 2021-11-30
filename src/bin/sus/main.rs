mod option;

use option::CommandLineOptions;
use structopt::StructOpt;

fn main() {
    let command_line_opts = CommandLineOptions::from_args();
    println!("{:?}", command_line_opts);
}
