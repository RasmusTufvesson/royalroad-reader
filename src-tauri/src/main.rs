#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::Manager;
mod royalroad;

struct AppState {
    manager: royalroad::StoryManager,
}

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[tauri::command]
fn add_story(state: tauri::State<Mutex<AppState>>, url: &str) {
    state.lock().unwrap().manager.add_story_from_url(url.to_string());
}

#[derive(serde::Serialize)]
struct ChapterResponse {
    content: String,
    start_note: String,
    end_note: String,
    title: String,
    author: String,
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
        start_note: chap_content.start_note.as_ref().ok_or("").unwrap().clone(),
        end_note: chap_content.end_note.as_ref().ok_or("").unwrap().clone(),
        title: chap.name.clone(),
        author: story.author.clone(),
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.center()?;
            Ok(())
        })
        .manage(Mutex::new(AppState { manager: royalroad::StoryManager::new() }))
        .invoke_handler(tauri::generate_handler![add_story, get_chapter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
