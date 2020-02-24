extern crate clap;

use clap::{load_yaml, App};
use wiki::wiki::*;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();
    if let Some(subcommand) = matches.subcommand_name() {
        let matches = matches.subcommand_matches(subcommand).unwrap(); //intentionally shadowing
        match subcommand {
            "search" => {
                if let Some(keywords) = matches.value_of("keywords") {
                    let mut limit = "10";
                    if let Some(lim) = matches.value_of("limit") {
                        if lim.parse::<u32>().is_ok() {
                            limit = lim;
                        } else {
                            println!("Invalid limit, defaulting to 10.\n--------------------------------\n");
                        }
                    }
                    let request = Request::new(Query::Search, keywords, Some(limit));
                    let response = request.search();
                    match matches.is_present("snippet") {
                        true => for (result, snippet) in response { println!("{}{}", result, snippet) }
                        false => for (result, _) in response { println!("{}", result) }
                    }
                } else {
                    println!("{}", matches.usage())
                }
            }
            "view" => {
                if let Some(title) = matches.values_of("title") {
                    let title = title
                        .collect::<Vec<&str>>()
                        .join(" ");
                    let request = Request::new(Query::View, &title, None);
                    let response = request.view();
                    println!("{}", response);
                } else {
                    println!("{}", matches.usage())
                }
            }
            _ => unimplemented!()
        };
    } else {
        unimplemented!() //todo: implement non-subcommand stuff
    }
}
