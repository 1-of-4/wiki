extern crate reqwest;
extern crate url;
extern crate serde_json;

pub mod wiki {
    use reqwest::blocking::get;
    use url::form_urlencoded::byte_serialize;
    use serde_json::{Value};
    use std::error::Error;
    type GenErr = Box<dyn Error> ;

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
        pub fn new(query: Query, keywords: &str) -> Result<Request, GenErr> {
            let keywords: String = byte_serialize(keywords.as_bytes())
                .collect::<String>(); //convert to valid URL format (" " to "+", for instance)
            let url: String = match query {
                Query::Search => format!("https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json", keywords),
                Query::Content => format!("https://en.wikipedia.org/w/api.php?action=parse&page={}&prop=text&format=json", keywords),
            };
            let json = get(&url)?.json()?;
            Ok(Request {
                query,
                keywords,
                json,
            })
        }

        pub fn search(&self) -> Result<Vec<(String, String)>, GenErr> {
            let results = self.json["query"]["search"] //get from endpoint and navigate down to list of results
                .as_array().unwrap() //convert to array
                .iter()
                .map(|result: &Value| {(
                    result["title"].as_str().unwrap().to_string(),
                    result["snippet"].as_str().unwrap().to_string()
                    )}
                )
                .collect(); //shove all the elements into a nice little vector
            Ok(results)
        }

        pub fn fetch(&self) -> Result<Vec<(String, Option<String>)>, GenErr> {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::wiki::*;
    use url::form_urlencoded::byte_serialize;

    #[test]
    fn url_encode() {
        let keywords: String = byte_serialize("some words!".as_bytes()).collect();
        assert_eq!(keywords, "some+words%21")
    }

    #[test]
    fn search() {
        let request = Request::new(Query::Search, "johnson").unwrap();
        let results = request.search()?;
        assert_eq!(results[0], ("Johnson", r#"<span class=\"searchmatch\">Johnson</span> is a surname of English, Scottish origin. The name itself is a patronym of the given name John, literally meaning &quot;son of John&quot;. The name John"#))
    }
}