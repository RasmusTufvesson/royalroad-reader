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
fn add_story(state: tauri::State<Arc<Mutex<AppState>>>, url: &str) -> bool {
    let state = &mut state.lock().unwrap();
    match state.manager.add_story_from_url(url.to_string()) {
        Ok(index) => {
            state.story_page.story_index = index;
            true
        }
        Err(_) => {
            false
        }
    }
}

#[tauri::command]
fn remove_story(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize) {
    let manager = &mut state.lock().unwrap().manager;
    manager.stories.remove(story_index);
    for (index, story) in manager.follows.iter().enumerate() {
        if story == &story_index {
            manager.follows.remove(index);
            break;
        }
    }
    for (index, story) in manager.read_later.iter().enumerate() {
        if story == &story_index {
            manager.read_later.remove(index);
            break;
        }
    }
    for (index, story) in manager.finished.iter().enumerate() {
        if story == &story_index {
            manager.finished.remove(index);
            break;
        }
    }
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
    last_read: String,
}

#[tauri::command]
fn get_stories(state: tauri::State<Arc<Mutex<AppState>>>) -> Vec<StoryResponse> {
    state.lock().unwrap().manager.stories.iter().enumerate().map(|(index, story)| {
        StoryResponse {
            title: story.title.clone(),
            author: story.author.clone(),
            index,
            last_read: story.chapters[story.progress].name.clone(),
        }
    }).collect()
}

#[tauri::command]
fn get_follows(state: tauri::State<Arc<Mutex<AppState>>>) -> Vec<StoryResponse> {
    let state = state.lock().unwrap();
    let mut stories_unread = vec![];
    let mut stories_read = vec![];
    for index in &state.manager.follows {
        let story = &state.manager.stories[*index];
        if story.progress != story.chapters.len() - 1 {
            stories_unread.push(StoryResponse {
                title: story.title.clone(),
                author: story.author.clone(),
                index: *index,
                last_read: story.chapters[story.progress].name.clone(),
            });
        } else {
            stories_read.push(StoryResponse {
                title: story.title.clone(),
                author: story.author.clone(),
                index: *index,
                last_read: story.chapters[story.progress].name.clone(),
            });
        }
    }
    stories_unread.append(&mut stories_read);
    stories_unread
}

#[tauri::command]
fn get_unread_follows(state: tauri::State<Arc<Mutex<AppState>>>) -> Vec<StoryResponse> {
    let state = state.lock().unwrap();
    let mut stories = vec![];
    for index in &state.manager.follows {
        let story = &state.manager.stories[*index];
        if story.progress != story.chapters.len() - 1 {
            stories.push(StoryResponse {
                title: story.title.clone(),
                author: story.author.clone(),
                index: *index,
                last_read: story.chapters[story.progress].name.clone(),
            });
        }
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
            last_read: story.chapters[story.progress].name.clone(),
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
            last_read: story.chapters[story.progress].name.clone(),
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
    let mut chapter_index = st.manager.stories[story_index].progress;
    if chapter_index != st.manager.stories[story_index].chapters.len() - 1 {
        chapter_index += 1;
        st.manager.stories[story_index].progress = chapter_index;
    }
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
    followed: bool,
    read_later: bool,
    finished: bool,
}

#[derive(serde::Serialize, Clone)]
struct StoryPage {
    story_index: usize,
}

#[tauri::command]
fn get_story_info(state: tauri::State<Arc<Mutex<AppState>>>) -> StoryInfoResponse {
    let state = state.lock().unwrap();
    let index = state.story_page.story_index;
    let story = &state.manager.stories[index];
    StoryInfoResponse {
        title: story.title.clone(),
        author: story.author.clone(),
        description: story.description.clone(),
        index,
        followed: state.manager.follows.contains(&index),
        read_later: state.manager.read_later.contains(&index),
        finished: state.manager.finished.contains(&index),
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

#[tauri::command]
fn get_last_read_chapter(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize) -> ChapterTitleResponse {
    let story = &state.lock().unwrap().manager.stories[story_index];
    ChapterTitleResponse {
        title: story.chapters[story.progress].name.clone(),
        index: story.progress,
    }
}

#[tauri::command]
fn change_followed(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize) {
    let mut state = state.lock().unwrap();
    for (index, story) in state.manager.follows.iter().enumerate() {
        if story == &story_index {
            state.manager.follows.remove(index);
            return;
        }
    }
    state.manager.follows.push(story_index);
}

#[tauri::command]
fn change_read_later(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize) {
    let mut state = state.lock().unwrap();
    for (index, story) in state.manager.read_later.iter().enumerate() {
        if story == &story_index {
            state.manager.read_later.remove(index);
            return;
        }
    }
    state.manager.read_later.push(story_index);
}

#[tauri::command]
fn change_finished(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize) {
    let mut state = state.lock().unwrap();
    for (index, story) in state.manager.finished.iter().enumerate() {
        if story == &story_index {
            state.manager.finished.remove(index);
            return;
        }
    }
    state.manager.finished.push(story_index);
}

#[tauri::command]
fn update_stories(state: tauri::State<Arc<Mutex<AppState>>>) {
    state.lock().unwrap().manager.update_all();
}

#[tauri::command]
fn update_follows(state: tauri::State<Arc<Mutex<AppState>>>) {
    let manager = &mut state.lock().unwrap().manager;
    for follow in &manager.follows {
        manager.stories[*follow].update();
    }
}

#[tauri::command]
fn update_story(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize) {
    state.lock().unwrap().manager.stories[story_index].update();
}

#[tauri::command]
fn download_story(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize) {
    state.lock().unwrap().manager.stories[story_index].download_all();
}

#[derive(serde::Serialize)]
struct UpdateStoryResponse {
    title: String,
    description: String,
}

#[tauri::command]
fn update_story_details(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize) -> UpdateStoryResponse {
    let story = &mut state.lock().unwrap().manager.stories[story_index];
    story.update_details();
    UpdateStoryResponse {
        title: story.title.clone(),
        description: story.description.clone(),
    }
}

#[tauri::command]
fn delete_start_note(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize, chapter_index: usize) {
    let chapter = &mut state.lock().unwrap().manager.stories[story_index].chapters[chapter_index];
    if let Some(content) = &mut chapter.content {
        content.start_note = None;
    }
}

#[tauri::command]
fn delete_end_note(state: tauri::State<Arc<Mutex<AppState>>>, story_index: usize, chapter_index: usize) {
    let chapter = &mut state.lock().unwrap().manager.stories[story_index].chapters[chapter_index];
    if let Some(content) = &mut chapter.content {
        content.end_note = None;
    }
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
            get_last_read_chapter,
            change_followed,
            change_read_later,
            change_finished,
            remove_story,
            update_stories,
            update_story,
            download_story,
            get_unread_follows,
            update_story_details,
            update_follows,
            delete_end_note,
            delete_start_note,
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
