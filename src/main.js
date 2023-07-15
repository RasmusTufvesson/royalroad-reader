const { invoke } = window.__TAURI__.tauri;

window.addEventListener("DOMContentLoaded", async () => {
  let chapEl = document.querySelector("#chapter-container");
  let startNoteEl = document.querySelector("#start-note");
  let endNoteEl = document.querySelector("#end-note");
  let startNoteContEl = document.querySelector("#start-note-container");
  let endNoteContEl = document.querySelector("#end-note-container");
  let titleEl = document.querySelector("#chapter-title");
  await invoke("add_story", { url: "https://www.royalroad.com/fiction/15935/there-is-no-epic-loot-here-only-puns" });
  let res = await invoke("get_chapter", { storyIndex: 0, chapterIndex: 201 });
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
});
