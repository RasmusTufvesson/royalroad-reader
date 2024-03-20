const { appWindow } = window.__TAURI__.window;

let windowEl;

window.addEventListener("DOMContentLoaded", async () => {
    windowEl = document.querySelector("#window");
    let size = await appWindow.innerSize();
    windowEl.style.height = (size.height - 30) + "px";
    windowEl.style.width = size.width + "px";
});

appWindow.onResized(({ payload: size }) => {
    windowEl.style.height = (size.height - 30) + "px";
    windowEl.style.width = size.width + "px";
});