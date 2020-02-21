extern crate reqwest;
extern crate url;
extern crate serde_json;
extern crate html2text;

pub mod wiki {
    use reqwest::blocking::get;
    use url::form_urlencoded::byte_serialize;
    use serde_json::Value;
    use html2text::from_read;

    pub enum Query {
        Search,
        View,
        Download,
    }

    pub struct Request {
        keywords: String,
        url: String,
    }

    impl Request {
        pub fn new(query: Query, keywords: &str) -> Request {
            let keywords = byte_serialize(keywords.as_bytes()) //convert to valid URL format (" " to "+", for instance)
                .collect::<String>(); //the API *should* do this for us, but it doesn't hurt to make sure
            let url = match query {
                Query::Search => "https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json",
                Query::View => "https://en.wikipedia.org/w/api.php?action=query&prop=extracts&format=json&titles={}&redirects",
                Query::Download => "https://en.wikipedia.org/w/index.php?title={}&action=raw"
            }.replace("{}", keywords.as_ref()); //kinda jank but format! doesnt work with &str
            Request {
                keywords,
                url,
            }
        }

        fn json(&self) -> Value {
            get(&self.url) //Option<???>
                .unwrap() //http???
                .json() //Option<Value>
                .unwrap() //Value
        }

        pub fn search(&self) -> Vec<(String, String)> {
            let results = self.json()["query"]["search"] //get from endpoint and navigate down to list of results
                .as_array().unwrap() //convert to array
                .iter()
                .map(|result| {
                    (
                        from_read(result["title"]
                                      .as_str()
                                      .unwrap()
                                      .as_bytes(), 80),
                        from_read(result["snippet"]
                                      .as_str()
                                      .unwrap()
                                      .as_bytes(), 80),
                    )
                })
                .collect(); //shove all the elements into a nice little vector
            results
        }

        pub fn view(&self) -> String {
            let json = &self.json()["query"]["pages"];
            if let Some(id) = json.as_object().unwrap().keys().nth(0) {
                from_read(json[id]["extract"]
                              .as_str()
                              .unwrap()
                              .as_bytes(),
                          120)
            } else {
                format!("No article found with name \"{}\"", self.keywords)
            }
        }

        pub fn download(&self) -> String {
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
        let results = Request::new(Query::Search, "ricardo").search();
        assert_eq!(results[1], (
            String::from("David Ricardo\n"),
            String::from("David Ricardo (18 April 1772 – 11 September 1823) was a British political\neconomist, one of the most influential of the classical economists along with\n")
        ));
    }

    #[test]
    fn view() {
        let request = Request::new(Query::View, "animal");
        let results = request.view();
        assert_ne!(String::from("No article found with name \"animal\""), results)
    }
}