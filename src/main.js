const { invoke } = window.__TAURI__.tauri;

window.addEventListener("DOMContentLoaded", async () => {
  var chapEl = document.querySelector("#chapter-container");
  await invoke("add_story", { url: "https://www.royalroad.com/fiction/15935/there-is-no-epic-loot-here-only-puns" });
  chapEl.innerHTML = await invoke("get_chapter", { storyIndex: 0, chapterIndex: 201 });
});
