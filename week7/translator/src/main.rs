use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mode = get_mode_from_user_input()?;

        let mut buffer = String::new();

        println!("Enter the text to translate:");

        io::stdin().read_line(&mut buffer)?;

        let (from, to) = match mode {
            TranslationMode::EnglishToChinese => ("en", "zh"),
            TranslationMode::ChineseToEnglish => ("zh", "en"),
        };

        let response = reqwest::get(format!(
            "https://api.mymemory.translated.net/get?q={}&langpair={}|{}",
            buffer.trim(),
            from,
            to
        ))
        .await?
        .json::<serde_json::Value>()
        .await?;

        if let Some(translated) = response["responseData"]["translatedText"].as_str() {
            println!("Translation: {}", translated);
        } else {
            println!("Could not get translation.");
        }

        if !ask_user_to_continue()? {
            break;
        }
    }

    Ok(())
}

enum TranslationMode {
    EnglishToChinese,
    ChineseToEnglish,
}

fn get_mode_from_user_input() -> io::Result<TranslationMode> {
    loop {
        print!("Choose a translation mode (1 for English to Chinese, 2 for Chinese to English): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => return Ok(TranslationMode::EnglishToChinese),
            "2" => return Ok(TranslationMode::ChineseToEnglish),
            _ => {
                println!("Invalid input. Please choose 1 or 2.");
                continue;
            }
        }
    }
}

fn ask_user_to_continue() -> io::Result<bool> {
    loop {
        print!("Do you want to translate another text? (y/n): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => {
                println!("Invalid input. Please choose y or n.");
                continue;
            }
        }
    }
}
