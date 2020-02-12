extern crate reqwest;
extern crate url;
extern crate serde_json;

pub mod interface {
    use reqwest::blocking::get;
    use url::form_urlencoded::byte_serialize;
    use serde_json::Value;
    use std::io::Read;

    pub fn find_article(query: &str) -> Vec<String> { //todo: return a result
        let url: String = format!(
            "https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json",
            byte_serialize(query
                .as_bytes())
                .collect::<String>() //convert to valid URL format (" " to "%20", for instance)
        );
        let mut response: String = String::new();
        get(&url).unwrap().read_to_string(&mut response);
        let results: Value = serde_json::json!(response);
        let result_list: Vec<String> = results["query"]["search"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v: &Value| v["title"]
                .as_str()
                .unwrap()
                .to_owned()) //primary culprit if something goes wrong
            .collect(); //todo: optimize this, jesus fuck
        result_list
    }

    pub fn fetch_contents() {
        unimplemented!()
    }
}