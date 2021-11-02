use std::env;
use nix::unistd;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    for argument in args.iter() {
        println!("{}", argument);
    }
}
