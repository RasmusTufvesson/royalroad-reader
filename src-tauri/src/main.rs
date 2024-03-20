#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use tauri::Manager;
mod royalroad;

const SAVE_FILE: &'static str = "save.bin";

struct AppState {
    manager: royalroad::StoryManager,
    read_page: ReadPage,
    story_page: StoryPage,
}

#[tauri::command]
fn add_story(state: tauri::State<Arc<Mutex<AppState>>>, url: &str) {
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
fn get_chapter(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize, chapter_index: usize) -> ChapterResponse {
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
fn get_stories(state: tauri::State<Arc<Mutex<AppState>>>) -> Vec<StoryResponse> {
    state.lock().unwrap().manager.stories.iter().enumerate().map(|(index, story)| {
        StoryResponse {
            title: story.title.clone(),
            author: story.author.clone(),
            index,
        }
    }).collect()
}

#[tauri::command]
fn get_follows(state: tauri::State<Arc<Mutex<AppState>>>) -> Vec<StoryResponse> {
    let state = state.lock().unwrap();
    let mut stories = vec![];
    for index in &state.manager.follows {
        let story = &state.manager.stories[*index];
        stories.push(StoryResponse {
            title: story.title.clone(),
            author: story.author.clone(),
            index: *index,
        });
    }
    stories
}

#[tauri::command]
fn get_read_later(state: tauri::State<Arc<Mutex<AppState>>>) -> Vec<StoryResponse> {
    let state = state.lock().unwrap();
    let mut stories = vec![];
    for index in &state.manager.read_later {
        let story = &state.manager.stories[*index];
        stories.push(StoryResponse {
            title: story.title.clone(),
            author: story.author.clone(),
            index: *index,
        });
    }
    stories
}

#[tauri::command]
fn get_finished(state: tauri::State<Arc<Mutex<AppState>>>) -> Vec<StoryResponse> {
    let state = state.lock().unwrap();
    let mut stories = vec![];
    for index in &state.manager.finished {
        let story = &state.manager.stories[*index];
        stories.push(StoryResponse {
            title: story.title.clone(),
            author: story.author.clone(),
            index: *index,
        });
    }
    stories
}

#[derive(serde::Serialize, Clone)]
struct ReadPage {
    story_index: usize,
    chapter_index: usize,
}

#[tauri::command]
fn set_read_page(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize, chapter_index: usize) {
    let page = &mut state.lock().unwrap().read_page;
    page.story_index = story_index;
    page.chapter_index = chapter_index;
}

#[tauri::command]
fn set_read_page_continue(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize) {
    let st = &mut state.lock().unwrap();
    let chapter_index = st.manager.stories[story_index].progress;
    let page = &mut st.read_page;
    page.story_index = story_index;
    page.chapter_index = chapter_index;
}

#[tauri::command]
fn get_read_page(state: tauri::State<Arc<Mutex<AppState>>>) -> ReadPage {
    state.lock().unwrap().read_page.clone()
}

#[derive(serde::Serialize)]
struct StoryInfoResponse {
    title: String,
    author: String,
    description: String,
    index: usize,
}

#[derive(serde::Serialize, Clone)]
struct StoryPage {
    story_index: usize,
}

#[tauri::command]
fn get_story_info(state: tauri::State<Arc<Mutex<AppState>>>) -> StoryInfoResponse {
    let index = state.lock().unwrap().story_page.story_index;
    let story = &state.lock().unwrap().manager.stories[index];
    StoryInfoResponse {
        title: story.title.clone(),
        author: story.author.clone(),
        description: story.description.clone(),
        index,
    }
}

#[tauri::command]
fn set_story_page(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize) {
    let page = &mut state.lock().unwrap().story_page;
    page.story_index = story_index;
}

#[tauri::command]
fn set_story_progress(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize, progress: usize) {
    state.lock().unwrap().manager.stories[story_index].progress = progress;
}

#[tauri::command]
fn get_story_progress(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize) -> usize {
    state.lock().unwrap().manager.stories[story_index].progress
}

#[derive(serde::Serialize)]
struct ChapterTitleResponse {
    title: String,
    index: usize,
}

#[tauri::command]
fn get_chapters(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize) -> Vec<ChapterTitleResponse> {
    state.lock().unwrap().manager.stories[story_index].chapters.iter().enumerate().map(|(index, chapter)| {
        ChapterTitleResponse {
            title: chapter.name.clone(),
            index,
        }
    }).collect()
}

fn main() {
    let state = Arc::new(Mutex::new(AppState { manager: royalroad::StoryManager::load_or_new(SAVE_FILE), read_page: ReadPage { story_index: 0, chapter_index: 0 }, story_page: StoryPage { story_index: 0 } }));
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.center()?;
            Ok(())
        })
        .manage(state.clone())
        .invoke_handler(tauri::generate_handler![
            add_story,
            get_chapter,
            get_stories,
            set_read_page,
            get_read_page,
            get_story_info,
            set_story_page,
            set_read_page_continue,
            set_story_progress,
            get_story_progress,
            get_chapters,
            get_follows,
            get_read_later,
            get_finished,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { .. } => {
                state.lock().unwrap().manager.save(SAVE_FILE);
            }
            _ => {}
        });
}
