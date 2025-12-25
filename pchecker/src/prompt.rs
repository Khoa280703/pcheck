// Interactive prompt module for language selection

use crate::lang::Language;

#[allow(dead_code)]
pub fn select_language_silent() -> Language {
    // For non-interactive mode (future: --lang flag support)
    Language::Vietnamese
}
