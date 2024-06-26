use std::{cmp::min, fs::File, io::{Read, Write}};
use reqwest;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use bincode::{serialize, deserialize};

#[derive(PartialEq, Deserialize, Serialize)]
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
            static ref CONTENT_REGEX: Regex = Regex::new(
                r#"(?:<div class="portlet solid author-note-portlet">[\t\n ]*<div class="portlet-title">[\t\n ]*<div class="caption">[\t\n ]*<i class="fa fa-sticky-note"><\/i>[\t\n ]*<span class="caption-subject bold uppercase">.*?<\/span>[\t\n ]*<\/div>[\t\n ]*<\/div>[\t\n ]*<div class="portlet-body author-note">([\w\W]*?)<\/div>[\t\n ]*<\/div>[\t\n ]*)?(?:<div class="margin-bottom-20 portlet light t-center-3" style="padding-top: 5px !important;position: relative"><div class="bold uppercase">Advertisement<\/div>[\w\W]*?)?<div class="chapter-inner chapter-content">([\s\S]*?)<\/div>[\t\n ]*<div class="portlet light t-center-3" style="padding-top: 5px !important;position: relative">[\s\S]*?(?:<div class="portlet solid author-note-portlet">[\t\n ]*<div class="portlet-title">[\t\n ]*<div class="caption">[\t\n ]*<i class="fa fa-sticky-note"><\/i>[\t\n ]*<span class="caption-subject bold uppercase">.*?<\/span>[\t\n ]*<\/div>[\t\n ]*<\/div>[\t\n ]*<div class="portlet-body author-note">([\w\W]*?)<\/div>[\t\n ]*<\/div>)?[\t\n ]*<hr \/>"#
            ).unwrap();
            static ref HIDDEN_CLASS_REGEX: Regex = Regex::new(
                r#"<style>[\t\n ]*\.(.*?)\{[\t\n ]*display: none;[\t\n ]*speak: never;[\t\n ]*\}[\t\n ]*<\/style>"#
            ).unwrap();
        }
        let caps = CONTENT_REGEX.captures(&content).unwrap();
        let start_note = caps.get(1).and_then(|x| Some(x.as_str().to_string()));
        let end_note = caps.get(3).and_then(|x| Some(x.as_str().to_string()));
        let chap_content = caps.get(2).unwrap().as_str();
        if let Some(hidden) = HIDDEN_CLASS_REGEX.captures(&content) {
            let replace_regex = Regex::new(
                &format!(r#"<p class="{}"[\w\W]*?<\/p>"#, hidden.get(1).unwrap().as_str())
            ).unwrap();
            Self {
                start_note,
                chapter_content: replace_regex.replace_all(chap_content, "").to_string(),
                end_note,
            }
        } else {
            Self {
                start_note,
                chapter_content: chap_content.to_string(),
                end_note,
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Chapter {
    pub content: Option<ChapterContent>,
    pub name: String,
    url: String,
    id: u32,
}

impl Chapter {
    pub fn from_name_and_url(name: String, url: String, id: u32) -> Self {
        Self { content: None, name, url, id }
    }

    pub fn load_content(&mut self) {
        if self.content == None {
            self.content = Some(ChapterContent::from_url(&self.url));
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Story {
    pub title: String,
    pub id: u32,
    pub page_url: String,
    pub description: String,
    pub chapters: Vec<Chapter>,
    pub author: String,
    pub progress: usize,
}

impl Story {
    pub fn from_page_url(url: String) -> Self {
        let content = reqwest::blocking::get(&url)
            .expect("Coundnt get page")
            .text()
            .expect("Couldnt get text from page");
        lazy_static! {
            static ref TITLE_REGEX: Regex = Regex::new(r#"<meta name="twitter:title" content="(.*?)">"#).unwrap();
            static ref ID_REGEX: Regex = Regex::new(r#"https://www\.royalroad\.com/fiction/([0-9]*?)/.*?"#).unwrap();
            static ref DESCRIPTION_REGEX: Regex = Regex::new(
                r#"<div class="description">[\s\S]*?<div class="hidden-content">[\t\n ]*([\s\S]*?)[\t\n ]*<\/div>[\s\S]*?<\/div>"#
            ).unwrap();
            static ref CHAPTER_REGEX: Regex = Regex::new(
                r#"<tr style="cursor: pointer" data-url="(.*?)" data-volume-id=".*?" class="chapter-row">[\t\n ]*<td>[\t\n ]*<a href=".*?">[\t\n ]*(.*?)[\t\n ]*<\/a>[\t\n ]*<\/td>[\t\n ]*<td data-content="[0-9]*?" class="text-right">[\t\n ]*<a href=".*?" data-content=".*?">[\t\n ]*<time unixtime="[0-9]*" title=".*?" datetime=".*?" format="agoshort">.*?<\/time> ago[\t\n ]*<\/a>[\t\n ]*<\/td>[\t\n ]*<\/tr>"#
            ).unwrap();
            static ref AUTHOR_REGEX: Regex = Regex::new(
                r#"<meta property="books:author" content="(.*?)"\/>"#
            ).unwrap();
            static ref CHAPTER_ID_REGEX: Regex = Regex::new(
                r#"fiction/[0-9]*/[^/]*/chapter/([0-9]*)/[^/]*"#
            ).unwrap();
        }
        let title = TITLE_REGEX.captures(&content).unwrap().get(1).unwrap().as_str();
        let id = ID_REGEX.captures(&url).unwrap().get(1).unwrap().as_str();
        let description = DESCRIPTION_REGEX.captures(&content).unwrap().get(1).unwrap().as_str();
        let chapters = CHAPTER_REGEX.captures_iter(&content).map(|x|
            Chapter::from_name_and_url(x.get(2).unwrap().as_str().to_string(), "https://www.royalroad.com".to_string() + x.get(1).unwrap().as_str(), CHAPTER_ID_REGEX.captures(x.get(1).unwrap().as_str()).unwrap().get(1).unwrap().as_str().parse().unwrap())
        ).collect();
        let author = AUTHOR_REGEX.captures(&content).unwrap().get(1).unwrap().as_str();
        Self {
            title: title.to_string(),
            id: id.parse().unwrap(),
            page_url: url,
            description: description.to_string(),
            chapters,
            author: author.to_string(),
            progress: 0,
        }
    }

    pub fn update(&mut self) {
        let content = reqwest::blocking::get(&self.page_url)
            .expect("Coundnt get page")
            .text()
            .expect("Couldnt get text from page");
        lazy_static! {
            static ref CHAPTER_REGEX: Regex = Regex::new(
                r#"<tr style="cursor: pointer" data-url="(.*?)" data-volume-id=".*?" class="chapter-row">[\t\n ]*<td>[\t\n ]*<a href=".*?">[\t\n ]*(.*?)[\t\n ]*<\/a>[\t\n ]*<\/td>[\t\n ]*<td data-content="([0-9]*?)" class="text-right">[\t\n ]*<a href=".*?" data-content=".*?">[\t\n ]*<time unixtime="[0-9]*" title=".*?" datetime=".*?" format="agoshort">.*?<\/time> ago[\t\n ]*<\/a>[\t\n ]*<\/td>[\t\n ]*<\/tr>"#
            ).unwrap();
            static ref CHAPTER_ID_REGEX: Regex = Regex::new(
                r#"fiction/[0-9]*/[^/]*/chapter/([0-9]*)/[^/]*"#
            ).unwrap();
        }
        let mut expected_index = 0;
        for cap in CHAPTER_REGEX.captures_iter(&content) {
            let name = cap.get(2).unwrap().as_str().to_string();
            if self.chapters[expected_index].name == name {
                if expected_index + 1 != self.chapters.len() {
                    expected_index += 1;
                }
            } else {
                if let Some((index, _)) = self.chapters.iter().enumerate().find(|(_, chapter)| chapter.name == name) {
                    expected_index = min(index + 1, self.chapters.len() - 1);
                } else {
                    self.chapters.push(Chapter::from_name_and_url(name, "https://www.royalroad.com".to_string() + cap.get(1).unwrap().as_str(), CHAPTER_ID_REGEX.captures(cap.get(1).unwrap().as_str()).unwrap().get(1).unwrap().as_str().parse().unwrap()));
                }
            }
        }
    }

    pub fn download_all(&mut self) {
        for chapter in &mut self.chapters {
            chapter.load_content();
        }
    }

    pub fn update_details(&mut self) {
        let content = reqwest::blocking::get(&self.page_url)
            .expect("Coundnt get page")
            .text()
            .expect("Couldnt get text from page");
        lazy_static! {
            static ref TITLE_REGEX: Regex = Regex::new(r#"<meta name="twitter:title" content="(.*?)">"#).unwrap();
            static ref DESCRIPTION_REGEX: Regex = Regex::new(
                r#"<div class="description">[\s\S]*?<div class="hidden-content">[\t\n ]*([\s\S]*?)[\t\n ]*<\/div>[\s\S]*?<\/div>"#
            ).unwrap();
        }
        self.title = TITLE_REGEX.captures(&content).unwrap().get(1).unwrap().as_str().to_string();
        self.description = DESCRIPTION_REGEX.captures(&content).unwrap().get(1).unwrap().as_str().to_string();
    }
}

#[derive(Deserialize, Serialize)]
pub struct StoryManager {
    pub stories: Vec<Story>,
    pub follows: Vec<usize>,
    pub finished: Vec<usize>,
    pub read_later: Vec<usize>,
}

impl StoryManager {
    pub fn new() -> Self {
        Self {
            stories: vec![],
            follows: vec![],
            finished: vec![],
            read_later: vec![],
        }
    }

    pub fn add_story_from_url(&mut self, url: String) -> Result<usize, &'static str> {
        for story in &self.stories {
            if story.page_url == url {
                return Err("Already loaded");
            }
        }
        self.stories.push(Story::from_page_url(url));
        Ok(self.stories.len()-1)
    }
    
    pub fn save(&self, file: &str) {
        let serialized_data = serialize(self).unwrap();
        let mut file = File::create(file).unwrap();
        file.write_all(&serialized_data).unwrap();
    }

    pub fn load_or_new(file: &str) -> Self {
        match File::open(file) {
            Ok(mut file) => {
                let mut serialized_data = Vec::new();
                file.read_to_end(&mut serialized_data).unwrap();
                let data: Self = deserialize(&serialized_data).unwrap();
                data
            }
            Err(_) => {
                Self::new()
            }
        }
    }

    pub fn update_all(&mut self) {
        for story in &mut self.stories {
            story.update();
        }
    }
}