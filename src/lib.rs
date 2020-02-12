extern crate reqwest;
extern crate url;
extern crate serde_json;

pub mod wiki {
    use reqwest::blocking::get;
    use url::form_urlencoded::byte_serialize;
    use serde_json::{Value, json};
    use std::io::Read;
    use std::fmt::Error;

    pub enum Query {
        Search,
        Content,
    }

    pub struct Request {
        query: Query,
        keywords: String,
        json: Value,
    }

    impl Request {
        pub fn new(query: Query, keywords_: &str) -> Request {
            let keywords: String = byte_serialize(keywords_.as_bytes())
                .collect::<String>(); //convert to valid URL format (" " to "%20", for instance)
            let url: String = match query {
                Query::Search => format!("https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json", keywords),
                Query::Content => format!("https://en.wikipedia.org/w/api.php?action=parse&page={}&prop=text&format=json", keywords),
            };
            let json: Value = {
                let mut response: String = String::new();
                get(&url)
                    .unwrap()
                    .read_to_string(&mut response);
                json!(response)
            };
            Request {
                query,
                keywords,
                json
            }
        }

        fn search(&self) -> Vec<String> { //todo: handle errors
            return self
                .json["query"]["search"] //get from endpoint and navigate down to list of results
                .as_array() //convert to array
                .unwrap()
                .iter() //iterate so that we can...
                .map(|result: &Value| result["title"] //get the title field of each result
                    .as_str() //convert title to string
                    .unwrap()
                    .to_owned() //ensure ownership (primary culprit if something goes wrong)
                )
                .collect(); //shove all the elements into a nice little vector. todo: optimize this, jesus fuck
        }

        pub fn fetch(&self) -> Result<Vec<(String, Option<String>)>, Error> {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::wiki::*;

    #[test]
    fn search() {
        let request = Request::new(Query::Search, "johnson");
        println!("{:?}", find_article(request))
    }
}