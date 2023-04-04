use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut attractions: HashMap<&str, &str> = HashMap::new();

    // Add tourist attractions and their corresponding travel tips
    attractions.insert("Yellowstone National Park", "Pack warm clothes, and be aware of wildlife.");
    attractions.insert("Statue of Liberty", "Book your tickets in advance, and wear comfortable shoes.");
    attractions.insert("Grand Canyon", "Bring sunscreen, stay hydrated, and wear hiking shoes.");
    attractions.insert("Disney World", "Buy tickets online, visit during off-peak times, and use FastPass+.");
    attractions.insert("Golden Gate Bridge", "Wear layers, and bring a camera for the amazing views.");

    let attraction_keys: Vec<&str> = attractions.keys().copied().collect();
    let random_attraction = rand::thread_rng().gen_range(0..attraction_keys.len());
    let random_key = attraction_keys[random_attraction];

    println!("Random Tourist Attraction: {}", random_key);
    println!("Travel Tips: {}", attractions[random_key]);
}
