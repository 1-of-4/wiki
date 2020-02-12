#[macro_use]
extern crate clap;
use clap::App;
use roogle::interface::find_article;

fn main() { //todo: redo args to reflect new structure
    let yaml = load_yaml!("cli.yml");
    let app: App = App::from_yaml(yaml);
    let matches = app.get_matches();
    let query: &str = matches.value_of("query").unwrap();

    let results: Vec<String> = find_article(query);
    println!("{:?}", results)
}