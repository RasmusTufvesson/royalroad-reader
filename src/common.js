const { appWindow } = window.__TAURI__.window;

let windowEl;

window.addEventListener("DOMContentLoaded", async () => {
    windowEl = document.querySelector("#window");
    let size = await appWindow.innerSize();
    windowEl.style.height = (size.height - 30) + "px";
    windowEl.style.width = size.width + "px";
    document
        .getElementById('titlebar-minimize')
        .addEventListener('click', () => appWindow.minimize());
    document
        .getElementById('titlebar-maximize')
        .addEventListener('click', () => appWindow.toggleMaximize());
    document
        .getElementById('titlebar-close')
        .addEventListener('click', () => appWindow.close());
});

appWindow.onResized(({ payload: size }) => {
    windowEl.style.height = (size.height - 30) + "px";
    windowEl.style.width = size.width + "px";
});