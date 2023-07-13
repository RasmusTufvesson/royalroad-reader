mod royalroad;

fn main() {
    let story = royalroad::Story::from_page_url("https://www.royalroad.com/fiction/15935/there-is-no-epic-loot-here-only-puns".to_string());
    println!("{:?}", story);
}
