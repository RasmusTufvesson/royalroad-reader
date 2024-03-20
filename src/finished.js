const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

let storyEl;
let windowEl;

window.addEventListener("DOMContentLoaded", async () => {
    windowEl = document.querySelector("#window");
    storyEl = document.querySelector("#stories-container");
    document
        .getElementById('titlebar-minimize')
        .addEventListener('click', () => appWindow.minimize());
    document
        .getElementById('titlebar-maximize')
        .addEventListener('click', () => appWindow.toggleMaximize());
    document
        .getElementById('titlebar-close')
        .addEventListener('click', () => appWindow.close());
    let stories = await invoke("get_finished");
    stories.forEach(story => {
        let el = document.createElement("div");
        let div = document.createElement("div");
        let title = document.createElement("h2");
        title.innerHTML = "<strong>" + story.title + "</strong>";
        div.appendChild(title);
        let author = document.createElement("p");
        author.innerHTML = "<small>By " + story.author + "</small>";
        div.appendChild(author);
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