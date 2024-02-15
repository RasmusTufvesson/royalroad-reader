#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::Manager;
mod royalroad;

struct AppState {
    manager: royalroad::StoryManager,
    read_page: ReadPage,
    story_page: StoryPage,
}

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[tauri::command]
fn add_story(state: tauri::State<Mutex<AppState>>, url: &str) {
    let _ = state.lock().unwrap().manager.add_story_from_url(url.to_string());
}

#[derive(serde::Serialize)]
struct ChapterResponse {
    content: String,
    start_note: String,
    end_note: String,
    title: String,
    author: String,
    before_exists: bool,
    after_exists: bool,
}

#[tauri::command]
fn get_chapter(state: tauri::State<Mutex<AppState>>, story_index: usize, chapter_index: usize) -> ChapterResponse {
    state.lock().unwrap().manager.stories[story_index].chapters[chapter_index].load_content();
    let state = state.lock().unwrap();
    let story = &state.manager.stories[story_index];
    let chap = &story.chapters[chapter_index];
    let chap_content = &chap.content.as_ref().unwrap();
    ChapterResponse {
        content: chap_content.chapter_content.clone(),
        start_note: chap_content.start_note.as_ref().or(Some(&("".to_string()))).unwrap().clone(),
        end_note: chap_content.end_note.as_ref().or(Some(&("".to_string()))).unwrap().clone(),
        title: chap.name.clone(),
        author: story.author.clone(),
        before_exists: chapter_index != 0,
        after_exists: chapter_index != story.chapters.len() - 1,
    }
}

#[derive(serde::Serialize)]
struct StoryResponse {
    title: String,
    author: String,
    index: usize,
}

#[tauri::command]
fn get_stories(state: tauri::State<Mutex<AppState>>) -> Vec<StoryResponse> {
    state.lock().unwrap().manager.stories.iter().enumerate().map(|(index, story)| {
        StoryResponse {
            title: story.title.clone(),
            author: story.author.clone(),
            index,
        }
    }).collect()
}

#[derive(serde::Serialize, Clone)]
struct ReadPage {
    story_index: usize,
    chapter_index: usize,
}

#[tauri::command]
fn set_read_page(state: tauri::State<Mutex<AppState>>, story_index: usize, chapter_index: usize) {
    let page = &mut state.lock().unwrap().read_page;
    page.story_index = story_index;
    page.chapter_index = chapter_index;
}

#[tauri::command]
fn get_read_page(state: tauri::State<Mutex<AppState>>) -> ReadPage {
    state.lock().unwrap().read_page.clone()
}

#[derive(serde::Serialize)]
struct StoryInfoResponse {
    title: String,
    author: String,
    description: String,
}

#[derive(serde::Serialize, Clone)]
struct StoryPage {
    story_index: usize,
}

#[tauri::command]
fn get_story_info(state: tauri::State<Mutex<AppState>>) -> StoryInfoResponse {
    let index = state.lock().unwrap().story_page.story_index;
    let story = &state.lock().unwrap().manager.stories[index];
    StoryInfoResponse {
        title: story.title.clone(),
        author: story.author.clone(),
        description: story.description.clone()
    }
}

#[tauri::command]
fn set_story_page(state: tauri::State<Mutex<AppState>>, story_index: usize) {
    let page = &mut state.lock().unwrap().story_page;
    page.story_index = story_index;
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.center()?;
            Ok(())
        })
        .manage(Mutex::new(AppState { manager: royalroad::StoryManager::new(), read_page: ReadPage { story_index: 0, chapter_index: 0 }, story_page: StoryPage { story_index: 0 } }))
        .invoke_handler(tauri::generate_handler![
            add_story,
            get_chapter,
            get_stories,
            set_read_page,
            get_read_page,
            get_story_info,
            set_story_page,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
