use std::env::args;
use roogle::util::parse_args;

fn main() {
    let args: Vec<String> = args().collect();
    println!("{:?}", args);
    let query: String = parse_args(args);
}