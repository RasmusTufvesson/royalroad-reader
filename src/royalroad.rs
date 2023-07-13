use reqwest;
use lazy_static::lazy_static;
use regex::Regex;


#[derive(Debug)]
pub struct ChapterContent {
    start_note: Option<String>,
    chapter_content: String,
    end_note: Option<String>,
}

#[derive(Debug)]
pub struct Chapter {
    content: Option<ChapterContent>,
    name: String,
    url: String,
}

impl Chapter {
    pub fn from_name_and_url(name: String, url: String) -> Self {
        Self { content: None, name, url }
    }
}

#[derive(Debug)]
pub struct Story {
    title: String,
    id: u32,
    page_url: String,
    desctiption: String,
    chapters: Vec<Chapter>,
}

impl Story {
    pub fn from_page_url(url: String) -> Self {
        let url = if url.ends_with("/") { url[..url.len()-1].to_string() } else { url };
        let content = reqwest::blocking::get(&url)
            .expect("Coundnt get page")
            .text()
            .expect("Couldnt get text from page");
        lazy_static! {
            static ref title_regex: Regex = Regex::new(r#"<meta name="twitter:title" content="(.*?)">"#).unwrap();
            static ref id_regex: Regex = Regex::new(r#"https://www\.royalroad\.com/fiction/([0-9]*?)/.*?"#).unwrap();
            static ref description_regex: Regex = Regex::new(
                r#"<div class="description">[\s\S]*?<div class="hidden-content">[\t\n ]*([\s\S]*?)[\t\n ]*<\/div>[\s\S]*?<\/div>"#
            ).unwrap();
            static ref chapter_regex: Regex = Regex::new(
                r#"<tr style="cursor: pointer" data-url="(.*?)" data-volume-id="null" class="chapter-row">[\t\n ]*<td>[\t\n ]*<a href=".*?">[\t\n ]*(.*?)[\t\n ]*<\/a>[\t\n ]*<\/td>[\t\n ]*<td data-content="[0-9]*?" class="text-right">[\t\n ]*<a href=".*?" data-content=".*?">[\t\n ]*<time unixtime="[0-9]*" title=".*?" datetime=".*?" format="agoshort">.*?<\/time> ago[\t\n ]*<\/a>[\t\n ]*<\/td>[\t\n ]*<\/tr>"#
            ).unwrap();
        }
        let title = title_regex.captures(&content).unwrap().get(1).unwrap().as_str();
        let id = id_regex.captures(&url).unwrap().get(1).unwrap().as_str();
        let description = description_regex.captures(&content).unwrap().get(1).unwrap().as_str();
        let chapters = chapter_regex.captures_iter(&content).map(|x|
            Chapter::from_name_and_url(x.get(2).unwrap().as_str().to_string(), url.clone() + x.get(1).unwrap().as_str())
        ).collect();
        Self {
            title: title.to_string(),
            id: id.parse().unwrap(),
            page_url: url,
            desctiption: description.to_string(),
            chapters,
        }
    }
}