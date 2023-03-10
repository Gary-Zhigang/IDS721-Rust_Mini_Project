use reqwest::blocking::get;
use select::document::Document;
use select::predicate::{Class, Name};

fn main() {
    let url = "https://nogibjj.github.io/rust-tutorial/chapter_1.html";
    let response = get(url).unwrap().text().unwrap();
    let document = Document::from(response.as_str());
    let chapters = document.find(Class("chapter-item")).filter(|n| n.attr("class").unwrap_or_default().contains("expanded"));
    for chapter in chapters {
        let chapter_title = chapter.find(Name("a")).next().unwrap().text();
        println!("{}", chapter_title);
    }
}
