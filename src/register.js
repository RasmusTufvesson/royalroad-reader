const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

let registerEl;
let urlEl;
let windowEl;

window.addEventListener("DOMContentLoaded", async () => {
    windowEl = document.querySelector("#window");
    registerEl = document.querySelector("#register");
    urlEl = document.querySelector("#url");
    registerEl.addEventListener('click', async () => {
        let success = await invoke("add_story", { url: urlEl.value });
        if (success) {
            window.location = '/story';
        } else {
            window.location = '/';
        }
    });
    windowEl.style.display = null;
});