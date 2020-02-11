extern crate reqwest;
use reqwest::blocking;

pub mod interface {
    pub fn find_article(query: &str) {
        let url = format!("https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=jsonfm",
            query
        );
        let response = reqwest::blocking::get(url)?;
    }
    pub fn fetch_contents() {
        unimplemented!()
    }
}