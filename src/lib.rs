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
            let formatted = byte_serialize(self.keywords.as_bytes())
                .collect::<String>(); //convert to valid URL format (" " to "%20", for instance)
            match self.query {
                Query::Search => format!("https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json", formatted),
                Query::Content => format!("https://en.wikipedia.org/w/api.php?action=parse&page={}&prop=text&format=json", formatted),
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

    pub fn search(request: Request) -> Vec<String> { //todo: return a Result<Vector<String>, ?>
        return request
            .get_json()["query"]["search"] //get from endpoint and navigate down to list of results
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