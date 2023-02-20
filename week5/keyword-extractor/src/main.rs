use std::collections::HashMap;
use std::io;

fn main() {
    println!("Enter a paragraph:");
    let mut paragraph = String::new();
    io::stdin()
        .read_line(&mut paragraph)
        .expect("Failed to read line");

    let keywords = get_keywords(paragraph);
    println!("Keywords: {:?}", keywords);
}

fn get_keywords(paragraph: String) -> Vec<String> {
    let mut keywords: Vec<String> = Vec::new();
    let mut word_count: HashMap<String, i32> = HashMap::new();

    for word in paragraph.split_whitespace() {
        let cleaned_word = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();

        if cleaned_word.len() <= 2 {
            continue;
        }

        let count = word_count.entry(cleaned_word).or_insert(0);
        *count += 1;
    }

    let mut word_count: Vec<_> = word_count.iter().collect();
    word_count.sort_by(|a, b| b.1.cmp(a.1));

    for i in 0..5 {
        if i < word_count.len() {
            keywords.push(word_count[i].0.to_string());
        }
    }

    keywords
}
