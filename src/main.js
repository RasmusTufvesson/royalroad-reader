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
let storyIndex;
let chapterIndex;
let windowEl;
let skipEl;
let reverseEl;
let storedProgress;

async function set_progress() {
  await invoke("set_story_progress", { storyIndex: storyIndex, progress: chapterIndex });
  storedProgress = chapterIndex;
}

window.nextPage = async function() {
  chapterIndex += 1;
  let progress_diff = chapterIndex - storedProgress;
  console.log(progress_diff)
  if (progress_diff == 1) {
    await set_progress();
    reverseEl.classList.add("empty")
    skipEl.classList.add("empty")
  } else if (progress_diff > 1) {
    reverseEl.classList.add("empty")
    skipEl.classList.remove("empty")
    skipEl.addEventListener('click', async () => {
      await set_progress();
      skipEl.classList.add("empty")
    });
  } else if (progress_diff < 0) {
    console.log("reverse")
    skipEl.classList.add("empty")
    reverseEl.classList.remove("empty")
    reverseEl.addEventListener('click', async () => {
      await set_progress();
      reverseEl.classList.add("empty")
    });
  }
  await loadPage();
  windowEl.scrollTo(0, 0);
}

window.prevPage = async function() {
  chapterIndex -= 1;
  let progress_diff = chapterIndex - storedProgress;
  console.log(progress_diff)
  if (progress_diff == 1) {
    await set_progress();
    reverseEl.classList.add("empty")
    skipEl.classList.add("empty")
  } else if (progress_diff > 1) {
    reverseEl.classList.add("empty")
    skipEl.classList.remove("empty")
    skipEl.addEventListener('click', async () => {
      await set_progress();
      skipEl.classList.add("empty")
    });
  } else if (progress_diff < 0) {
    skipEl.classList.add("empty")
    reverseEl.classList.remove("empty")
    reverseEl.addEventListener('click', async () => {
      await set_progress();
      reverseEl.classList.add("empty")
    });
  }
  await loadPage();
  windowEl.scrollTo(0, 0);
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
  skipEl = document.querySelector("#set-progress-skip-container");
  reverseEl = document.querySelector("#set-progress-reverse-container");
  const page = await invoke("get_read_page")
  storyIndex = page.story_index
  chapterIndex = page.chapter_index
  storedProgress = await invoke("get_story_progress", { storyIndex: storyIndex });
  let progress_diff = chapterIndex - storedProgress;
  if (progress_diff > 1) {
    skipEl.classList.remove("empty")
    skipEl.addEventListener('click', async () => {
      await set_progress();
      skipEl.classList.add("empty")
    });
  } else if (progress_diff < 0) {
    reverseEl.classList.remove("empty")
    reverseEl.addEventListener('click', async () => {
      await set_progress();
      reverseEl.classList.add("empty")
    });
  }
  await loadPage();
  windowEl.style.display = null;
});
