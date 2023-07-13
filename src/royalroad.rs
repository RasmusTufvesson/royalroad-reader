use reqwest;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Story {
    title: String,
    id: u32,
    page_url: String,
}

impl Story {
    pub fn from_page_url(url: String) -> Self {
        let content = reqwest::blocking::get(&url)
            .expect("Coundnt get page")
            .text()
            .expect("Couldnt get text from page");
        lazy_static! {
            static ref title_regex: Regex = Regex::new(r#"<meta name="twitter:title" content="(.*?)">"#).unwrap();
            static ref id_regex: Regex = Regex::new(r#"https://www\.royalroad\.com/fiction/([0-9]*?)/.*?"#).unwrap();
        }
        let title = title_regex.captures(&content).unwrap().get(1).unwrap().as_str();
        let id = id_regex.captures(&url).unwrap().get(1).unwrap().as_str();
        Self {
            title: title.to_string(),
            id: id.parse().unwrap(),
            page_url: url,
        }
    }
}