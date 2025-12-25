// Interactive prompt module for language selection

use std::io::{self, Write};
use crate::lang::Language;

pub fn select_language() -> Language {
    // Default to Vietnamese prompt for the language selection screen
    println!();
    println!("============================================================");
    println!("🤖 PCHECKER v0.1.0");
    println!("============================================================");
    println!();
    println!("Chọn ngôn ngữ / Select language:");
    println!();
    println!("  [1] Tiếng Việt");
    println!("  [2] English");
    println!();

    loop {
        print!("Your choice [1-2]: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim() {
                    "1" | "vi" | "VI" | "vietnamese" => return Language::Vietnamese,
                    "2" | "en" | "EN" | "english" => return Language::English,
                    _ => {
                        println!("⚠️  Invalid choice. Please select 1 or 2.");
                        continue;
                    }
                }
            }
            Err(_) => {
                // On error, default to Vietnamese
                return Language::Vietnamese;
            }
        }
    }
}

pub fn select_language_silent() -> Language {
    // For non-interactive mode (future: --lang flag support)
    Language::Vietnamese
}
