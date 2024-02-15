const { appWindow } = window.__TAURI__.window;

appWindow.onResized(({ payload: size }) => {
    windowEl.style.height = (size.height - 30) + "px";
    windowEl.style.width = size.width + "px";
});