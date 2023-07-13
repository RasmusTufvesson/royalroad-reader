mod royalroad;

fn main() {
    let mut story = royalroad::Story::from_page_url("https://www.royalroad.com/fiction/15935/there-is-no-epic-loot-here-only-puns".to_string());
    story.chapters[201].load_content();
    println!("{:?}", story.chapters[201].content);
}
