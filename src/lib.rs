extern crate reqwest;
extern crate url;

pub mod interface {
    use reqwest::Client;
    use url::form_urlencoded::byte_serialize;

    enum RequestType {
        Search,
        Fetch
    }

    pub fn find_article(query: &str) {
        let url = format!(
            "https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=jsonfm",
            byte_serialize(query.as_bytes()).collect() //convert to valid URL format (" " to "%20", for instance)
        );
        let response = reqwest::get(url)
            .await?
            .text()
            .await?;
    }

    pub fn fetch_contents() {
        unimplemented!()
    }
}