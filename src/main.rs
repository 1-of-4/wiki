#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app: App = App::from_yaml(yaml);
    let matches = app.get_matches();
    let query: &str = matches.value_of("query").unwrap();
}