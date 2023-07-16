const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

let storyEl;
let windowEl;

appWindow.onResized(({ payload: size }) => {
    windowEl.style.height = (size.height - 38) + "px";
});

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
    let stories = await invoke("get_stories");
    stories.forEach(story => {
        let el = document.createElement("div");
        let title = document.createElement("h2");
        title.innerHTML = "<strong>" + story.title + "</strong>";
        el.appendChild(title);
        let author = document.createElement("p");
        author.innerHTML = "<small>By " + story.author + "</small>";
        el.appendChild(author);
        el.classList.add("inner-container")
        storyEl.appendChild(el);
    });
});