const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

let storyEl;
let windowEl;
let regUrlEl;
let modalEl;

async function load_stories() {
    storyEl.innerHTML = "";
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
}

window.addEventListener("DOMContentLoaded", async () => {
    windowEl = document.querySelector("#window");
    storyEl = document.querySelector("#stories-container");
    modalEl = document.querySelector("#modal");
    regUrlEl = document.querySelector("#register-url");
    document.querySelector("#update").addEventListener('click', async () => {
        await invoke("update_stories");
        await load_stories();
    });
    document.querySelector("#update2").addEventListener('click', async () => {
        await invoke("update_follows");
        await load_stories();
    });
    document.querySelector("#register").addEventListener('click', async () => {
        modalEl.style.display = "block";
    });
    document.querySelector("#register-cancel").addEventListener('click', async () => {
        modalEl.style.display = "none";
    });
    document.querySelector("#register-confirm").addEventListener('click', async () => {
        let success = await invoke("add_story", { url: regUrlEl.value });
        if (success) {
            window.location = '/story';
        } else {
            modalEl.style.display = "none";
        }
    });
    await load_stories();
    windowEl.style.display = null;
});