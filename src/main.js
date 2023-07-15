const { invoke } = window.__TAURI__.tauri;

window.addEventListener("DOMContentLoaded", async () => {
  let chapEl = document.querySelector("#chapter-container");
  let startNoteEl = document.querySelector("#start-note-container");
  let endNoteEl = document.querySelector("#end-note-container");
  await invoke("add_story", { url: "https://www.royalroad.com/fiction/15935/there-is-no-epic-loot-here-only-puns" });
  let res = await invoke("get_chapter", { storyIndex: 0, chapterIndex: 201 });
  chapEl.innerHTML = res.content;
  if (res.start_note.length > 0) {
    startNoteEl.innerHTML = res.start_note;
    startNoteEl.classList.remove("empty");
  } else {
    startNoteEl.classList.add("empty");
  }
  if (res.end_note.length > 0) {
    endNoteEl.innerHTML = res.end_note;
    endNoteEl.classList.remove("empty");
  } else {
    endNoteEl.classList.add("empty");
  }
});
