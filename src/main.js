const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

let chapEl;
let startNoteEl;
let endNoteEl;
let startNoteContEl;
let endNoteContEl;
let titleEl;
let nextEl;
let previousEl;
let storyIndex = 0;
let chapterIndex = 201;
let windowEl;

appWindow.onResized(({ payload: size }) => {
  windowEl.style.height = (size.height - 38) + "px";
});

window.nextPage = async function() {
  chapterIndex += 1;
  await loadPage();
  window.scrollTo(0, 0);
}

window.prevPage = async function() {
  chapterIndex -= 1;
  await loadPage();
  window.scrollTo(0, 0);
}

async function loadPage() {
  let res = await invoke("get_chapter", { storyIndex: storyIndex, chapterIndex: chapterIndex });
  chapEl.innerHTML = res.content;
  if (res.start_note.length > 0) {
    startNoteEl.innerHTML = res.start_note;
    startNoteContEl.classList.remove("empty");
  } else {
    startNoteContEl.classList.add("empty");
  }
  if (res.end_note.length > 0) {
    endNoteEl.innerHTML = res.end_note;
    endNoteContEl.classList.remove("empty");
  } else {
    endNoteContEl.classList.add("empty");
  }
  titleEl.innerText = res.title;
  let author_refs = document.getElementsByClassName("author");
  for(var i = 0; i < author_refs.length; i++) {
    author_refs[i].innerText = res.author;
  }
  nextEl.disabled = !res.after_exists;
  previousEl.disabled = !res.before_exists;
}

window.addEventListener("DOMContentLoaded", async () => {
  chapEl = document.querySelector("#chapter-container");
  startNoteEl = document.querySelector("#start-note");
  endNoteEl = document.querySelector("#end-note");
  startNoteContEl = document.querySelector("#start-note-container");
  endNoteContEl = document.querySelector("#end-note-container");
  titleEl = document.querySelector("#chapter-title");
  nextEl = document.querySelector("#next");
  previousEl = document.querySelector("#prev");
  windowEl = document.querySelector("#window");
  document
    .getElementById('titlebar-minimize')
    .addEventListener('click', () => appWindow.minimize());
  document
    .getElementById('titlebar-maximize')
    .addEventListener('click', () => appWindow.toggleMaximize());
  document
    .getElementById('titlebar-close')
    .addEventListener('click', () => appWindow.close());
  await invoke("add_story", { url: "https://www.royalroad.com/fiction/15935/there-is-no-epic-loot-here-only-puns" });
  await loadPage();
});
