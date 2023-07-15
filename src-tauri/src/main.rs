#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
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

#[tauri::command]
fn get_chapter(state: tauri::State<Mutex<AppState>>, story_index: usize, chapter_index: usize) -> String {
    state.lock().unwrap().manager.stories[story_index].chapters[chapter_index].load_content();
    state.lock().unwrap().manager.stories[story_index].chapters[chapter_index].content.as_ref().unwrap().chapter_content.clone()
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState { manager: royalroad::StoryManager::new() }))
        .invoke_handler(tauri::generate_handler![add_story, get_chapter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
