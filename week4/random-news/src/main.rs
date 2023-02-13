use reqwest::blocking::get;
use select::document::Document;
use select::predicate::Class;
use rand::seq::SliceRandom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Make a GET request to the BBC News website and retrieve the response
    let res = get("https://www.bbc.com/news")?.text()?;
    
    // Parse the HTML document using the `select` crate
    let doc = Document::from(res.as_str());
    
    // Find all elements with the "gs-c-promo-heading__title" class, which correspond to the top news headlines
    let mut headlines = Vec::new();
    for headline in doc.find(Class("gs-c-promo-heading__title")) {
        headlines.push(headline);
    }
    
    // Choose a random headline from the list and print its text and URL
    let mut rng = rand::thread_rng();
    let random_headline = headlines.choose(&mut rng).unwrap();
    let headline_text = random_headline.text();
    let headline_link = random_headline
        .parent()
        .and_then(|parent| parent.attr("href"))
        .map(|link| format!("https://www.bbc.com{}", link))
        .unwrap_or_default();
    println!("{}\n{}", headline_text, headline_link);
    
    Ok(())
}
