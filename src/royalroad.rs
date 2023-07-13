use reqwest;
use lazy_static::lazy_static;
use regex::Regex;


#[derive(Debug)]
pub struct ChapterContent {
    pub start_note: Option<String>,
    pub chapter_content: String,
    pub end_note: Option<String>,
}

impl ChapterContent {
    pub fn from_url(url: &str) -> Self {
        let content = reqwest::blocking::get(url)
            .expect("Coundnt get page")
            .text()
            .expect("Couldnt get text from page");
        lazy_static! {
            static ref content_regex: Regex = Regex::new(
                r#"(?:<div class="portlet solid author-note-portlet">[\t\n ]*<div class="portlet-title">[\t\n ]*<div class="caption">[\t\n ]*<i class="fa fa-sticky-note"><\/i>[\t\n ]*<span class="caption-subject bold uppercase">.*?<\/span>[\t\n ]*<\/div>[\t\n ]*<\/div>[\t\n ]*<div class="portlet-body author-note">(.*?)<\/div>[\t\n ]*<\/div>[\t\n ]*)?<div class="chapter-inner chapter-content">([\s\S]*?)<\/div>(?:[\t\n ]*<h6 class="bold uppercase text-center">Advertisement<\/h6>[\t\n ]*<div class="wide text-center ">[\t\n ]*<div id="Content_Mid">[\t\n ]*<\/div>[\t\n ]*<\/div>[\t\n ]*)?(?:<div class="portlet solid author-note-portlet">[\t\n ]*<div class="portlet-title">[\t\n ]*<div class="caption">[\t\n ]*<i class="fa fa-sticky-note"><\/i>[\t\n ]*<span class="caption-subject bold uppercase">.*?<\/span>[\t\n ]*<\/div>[\t\n ]*<\/div>[\t\n ]*<div class="portlet-body author-note">(.*?)<\/div>[\t\n ]*<\/div>)?"#
            ).unwrap();
        }
        let caps = content_regex.captures(&content).unwrap();
        let start_note = caps.get(1).and_then(|x| Some(x.as_str().to_string()));
        let end_note = caps.get(3).and_then(|x| Some(x.as_str().to_string()));
        let chap_content = caps.get(2).unwrap().as_str();
        Self {
            start_note,
            chapter_content: chap_content.to_string(),
            end_note,
        }
    }
}

#[derive(Debug)]
pub struct Chapter {
    pub content: Option<ChapterContent>,
    pub name: String,
    url: String,
}

impl Chapter {
    pub fn from_name_and_url(name: String, url: String) -> Self {
        Self { content: None, name, url }
    }

    pub fn load_content(&mut self) {
        self.content = Some(ChapterContent::from_url(&self.url));
    }
}

#[derive(Debug)]
pub struct Story {
    pub title: String,
    id: u32,
    pub page_url: String,
    pub desctiption: String,
    pub chapters: Vec<Chapter>,
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
            Chapter::from_name_and_url(x.get(2).unwrap().as_str().to_string(), "https://www.royalroad.com".to_string() + x.get(1).unwrap().as_str())
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