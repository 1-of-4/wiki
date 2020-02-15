extern crate reqwest;
extern crate url;
extern crate serde_json;

pub mod wiki {
    use reqwest::blocking::get;
    use url::form_urlencoded::byte_serialize;
    use serde_json::{Value};

    type Safe<T> = Result<T, Box<dyn std::error::Error>>;

    pub enum Query {
        Search,
        View,
        Download,
    }

    pub struct Request {
        url: String,
    }

    impl Request {
        pub fn new(query: Query, keywords: &str) -> Request {
            let keywords = byte_serialize(keywords.as_bytes())
                .collect::<String>(); //convert to valid URL format (" " to "+", for instance)
            let url = match query {
                Query::Search => "https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json",
                Query::View => "https://en.wikipedia.org/w/api.php?action=parse&page={}&prop=text&format=json",
                Query::Download => "https://en.wikipedia.org/w/index.php?title={}&action=raw"
            }.replace("{}", keywords.as_ref()); //kinda jank but format! doesnt work with &str
            Request {
                url
            }
        }

        fn json(&self) -> Safe<Value> {
            Ok(
                get(&self.url)?
                    .json()?
            )
        }

        pub fn search(&self) -> Safe<Vec<(String, String)>> {
            let results = self.json()?["query"]["search"] //get from endpoint and navigate down to list of results
                .as_array().unwrap() //convert to array
                .iter()
                .map(|result: &Value| {(
                    result["title"]
                        .as_str()
                        .unwrap_or("No Title")
                        .to_string(),
                    result["snippet"]
                        .as_str()
                        .unwrap_or("No Snippet")
                        .replace("<span class=\"searchmatch\">", "")
                        .replace(r#"</span>"#, "")
                        .replace(r#"&quot;"#, "\"")
                )})
                .collect(); //shove all the elements into a nice little vector
            Ok(results)
        }

        pub fn view(&self) -> Safe<String> {
            unimplemented!()
        }

        pub fn download(&self) -> Safe<String> {
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
        let results = Request::new(Query::Search, "ricardo").search().unwrap();
        assert_eq!(results[1], (
            String::from("David Ricardo"),
            String::from("David Ricardo (18 April 1772 – 11 September 1823) was a British political economist, one of the most influential of the classical economists along with")
        ));
    }
}