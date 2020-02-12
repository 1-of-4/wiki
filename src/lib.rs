extern crate reqwest;
extern crate url;
extern crate serde_json;

pub mod interface {
    use reqwest::blocking::get;
    use url::form_urlencoded::byte_serialize;
    use serde_json::{Value, json};
    use std::io::Read;

    pub enum Query {
        Search,
        Content,
    }

    pub struct Request {
        query: Query,
        keywords: String,
    }

    impl Request {
        pub fn new(query: Query, keywords_: &str) -> Request {
            let keywords: String = String::from(keywords_);
            Request {
                query,
                keywords,
            }
        }

        fn url(&self) -> String {
            match &self.query {
                Query::Search => format!(
                    "https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json",
                    byte_serialize(self.keywords
                        .as_bytes())
                        .collect::<String>() //convert to valid URL format (" " to "%20", for instance)
                    ),
                Query::Content => String::new(), //todo: content url
            }
        }

        fn get_json(&self) -> Value {
            let mut response: String = String::new();
            get(&self.url())
                .unwrap()
                .read_to_string(&mut response);
            return json!(response);
        }
    }

    pub fn find_article(request: Request) -> Vec<String> { //todo: return a Result<Vector<String>, ?>
        return request
            .get_json()["query"]["search"]
            .as_array()
            .unwrap()
            .iter()
            .map(|result: &Value| result["title"]
                .as_str()
                .unwrap()
                .to_owned() //primary culprit if something goes wrong
            )
            .collect(); //todo: optimize this, jesus fuck
    }

    pub fn fetch_contents(request: Request) -> Option<String> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::interface::*;

    #[test]
    fn search() {
        let request = Request::new(Query::Search, "johnson");
        println!("{:?}", find_article(request))
    }
}