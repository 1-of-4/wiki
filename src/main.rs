extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("wiki")
        .version("0.1.0")
        .author("Aubrey Landau <aubreylandau@gmail.com>")
        .about("get a wikipedia page")
        .arg(Arg::with_name("query")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("the article you want to find on wikipedia"))
        .get_matches();
    let query: &str = matches.value_of("query").unwrap();
}