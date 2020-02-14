extern crate clap;
use clap::{load_yaml, App};
use wiki::wiki::*;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some("search") => {
            let request = Request::new(Query::Search, matches.value_of("keywords").unwrap());
            let with_snippet = matches.is_present("snippet");
            for (result, snippet) in request.search(with_snippet).unwrap() {
                if let Some(i) = snippet {
                    println!("{}\t{}", result, i)
                } else {
                    println!("{}", result)
                }
            }
        },
        _ => unimplemented!()
    };
}