#[macro_use]
extern crate clap;
use clap::{load_yaml, App};
use roogle::wiki::*;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app: App = App::from_yaml(yaml);
    let matches = app.get_matches();

    //todo: parse args
}