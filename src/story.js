const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

let chaptersEl;
let infoEl;
let titleEl;
let authorEl;
let windowEl;
let continueEl;
let followEl;
let finishedEl;
let laterEl;
let removeEl;
let updateEl;
let downloadEl;

function change_active(element) {
    element.classList.toggle("active-button")
}

async function load_chapters(index) {
    chaptersEl.innerHTML = '';
    let chapters = await invoke("get_chapters", { storyIndex: index });
    chapters.forEach(chapter => {
        let el = document.createElement("p");
        el.innerHTML = "<strong>" + chapter.title + "</strong>";
        el.classList.add("chapter")
        chaptersEl.appendChild(el);

        el.addEventListener('click', async () => {
            await invoke("set_read_page", { storyIndex: index, chapterIndex: chapter.index });
            window.location.href = "/chapter";
        });
    });
}

window.addEventListener("DOMContentLoaded", async () => {
    windowEl = document.querySelector("#window");
    chaptersEl = document.querySelector("#chapters-container");
    infoEl = document.querySelector("#story-info");
    titleEl = document.querySelector("#story-title");
    authorEl = document.querySelector("#story-author");
    continueEl = document.querySelector("#continue-button");
    followEl = document.querySelector("#follow-button");
    finishedEl = document.querySelector("#finished-button");
    laterEl = document.querySelector("#read-later-button");
    removeEl = document.querySelector("#remove-button");
    updateEl = document.querySelector("#update");
    downloadEl = document.querySelector("#download");
    let story = await invoke("get_story_info");
    titleEl.innerHTML = "<strong>" + story.title + "</strong>";
    authorEl.innerHTML = "<small>By " + story.author + "</small>";
    infoEl.innerHTML = story.description;
    continueEl.addEventListener('click', async () => {
        await invoke("set_read_page_continue", { storyIndex: story.index });
        window.location.href = "/chapter";
    });
    if (story.followed) {
        followEl.classList.add("active-button")
    }
    if (story.finished) {
        finishedEl.classList.add("active-button")
    }
    if (story.read_later) {
        laterEl.classList.add("active-button")
    }
    finishedEl.addEventListener('click', async () => {
        await invoke("change_finished", { storyIndex: story.index });
        change_active(finishedEl);
    });
    followEl.addEventListener('click', async () => {
        await invoke("change_followed", { storyIndex: story.index });
        change_active(followEl);
    });
    laterEl.addEventListener('click', async () => {
        await invoke("change_read_later", { storyIndex: story.index });
        change_active(laterEl);
    });
    removeEl.addEventListener('click', async () => {
        await invoke("remove_story", { storyIndex: story.index });
        window.location = '/';
    });
    updateEl.addEventListener('click', async () => {
        await invoke("update_story", { storyIndex: story.index });
        await load_chapters(story.index);
    });
    downloadEl.addEventListener('click', async () => {
        await invoke("download_story", { storyIndex: story.index });
    });
    await load_chapters(story.index);
    windowEl.style.display = null;
});