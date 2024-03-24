const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

let storyEl;
let windowEl;
let updateEl;

window.addEventListener("DOMContentLoaded", async () => {
    windowEl = document.querySelector("#window");
    storyEl = document.querySelector("#stories-container");
    updateEl = document.querySelector("#update");
    updateEl.addEventListener('click', async () => {
        await invoke("update_stories");
    });
    let stories = await invoke("get_unread_follows");
    stories.forEach(story => {
        let el = document.createElement("div");
        let div = document.createElement("div");
        let title = document.createElement("h2");
        title.innerHTML = "<strong>" + story.title + "</strong>";
        div.appendChild(title);
        let smallDiv = document.createElement("div");
        let author = document.createElement("p");
        author.innerHTML = "<small>By " + story.author + "</small>";
        smallDiv.appendChild(author);
        let lastRead = document.createElement("p");
        lastRead.innerHTML = "<small>" + story.last_read + "</small>";
        smallDiv.appendChild(lastRead);
        div.appendChild(smallDiv);
        el.appendChild(div);
        el.classList.add("inner-container");
        let readDiv = document.createElement("div");
        let read = document.createElement("button");
        let readImg = document.createElement("img");
        readImg.src = "/assets/play.svg";
        readImg.alt = "read";
        read.appendChild(readImg);
        readDiv.appendChild(read);
        el.appendChild(readDiv);
        storyEl.appendChild(el);

        read.addEventListener('click', async () => {
            await invoke("set_read_page_continue", { storyIndex: story.index });
            window.location.href = "/chapter";
        });
        title.addEventListener('click', async () => {
            await invoke("set_story_page", { storyIndex: story.index });
            window.location.href = "/story";
        });
    });
    windowEl.style.display = null;
});