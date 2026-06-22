use lingua::Language::{English, German, Polish, Russian};
use lingua::{LanguageDetector, LanguageDetectorBuilder};
use once_cell::sync::Lazy;
use rustrict::{Censor, Type};
use wasm_bindgen::prelude::*;

mod statics;
mod utils;

/// Building the language detector loads heavy language models, so construct it
/// once and reuse it across all calls instead of rebuilding it every time.
static DETECTOR: Lazy<LanguageDetector> = Lazy::new(|| {
    LanguageDetectorBuilder::from_languages(&[English, German, Russian, Polish]).build()
});

#[wasm_bindgen]
pub struct WasmCurse {
    replace_char: char,
    languages_to_check: Vec<String>,
}

#[wasm_bindgen]
impl WasmCurse {
    #[wasm_bindgen(constructor)]
    pub fn new(replace_char: Option<char>, languages_to_check: Option<Vec<String>>) -> WasmCurse {
        WasmCurse {
            replace_char: replace_char.unwrap_or('*'),
            languages_to_check: languages_to_check.unwrap_or(vec!["en".to_string()]),
        }
    }

    #[wasm_bindgen(js_name = checkCurseWords)]
    pub fn check_curse_words(&self, text: &str) -> bool {
        let lang = self.define_language(text);

        if self.wants_non_english() && lang != "en" {
            let dictionary = match utils::get_dictionary(&lang) {
                Some(dictionary) => dictionary,
                None => return false,
            };

            let text_lower = text.to_lowercase();

            for re in &dictionary.regexes {
                if re.is_match(&text_lower) {
                    return true;
                }
            }

            let words = utils::remove_all_symbols(&text_lower);
            return words
                .iter()
                .any(|word| dictionary.simple_words.contains(word.as_str()));
        }

        Censor::from_str(text)
            .with_censor_replacement(self.replace_char)
            .analyze()
            .is(Type::INAPPROPRIATE)
    }

    #[wasm_bindgen(js_name = defineLanguage)]
    pub fn define_language(&self, text: &str) -> String {
        let detected = match DETECTOR.detect_language_of(text) {
            Some(language) => language.to_string(),
            None => return "en".to_string(),
        };

        statics::LANGS
            .get(detected.as_str())
            .map_or("en", |code| *code)
            .to_string()
    }

    #[wasm_bindgen(js_name = replaceCurseWords)]
    pub fn replace_curse_words(&self, text: &str) -> String {
        let language = self.define_language(text);

        if self.wants_non_english() && language != "en" {
            let dictionary = match utils::get_dictionary(&language) {
                Some(dictionary) => dictionary,
                None => return text.to_string(),
            };

            let replaced_words: Vec<String> = text
                .split(' ')
                .map(|word| self.censor_word(word, dictionary))
                .collect();

            return replaced_words.join(" ");
        }

        Censor::from_str(text)
            .with_censor_replacement(self.replace_char)
            .censor()
    }
}

impl WasmCurse {
    fn wants_non_english(&self) -> bool {
        self.languages_to_check
            .iter()
            .any(|lang| lang == "ru" || lang == "de" || lang == "pl")
    }

    /// Masks the interior of a profane word while keeping its first and last
    /// characters as well as any trailing punctuation intact.
    fn censor_word(&self, word: &str, dictionary: &utils::CompiledDictionary) -> String {
        let lower_word = word.to_lowercase();

        let captures = match utils::WORD_SPLIT_RE.captures(&lower_word) {
            Some(captures) => captures,
            None => return word.to_string(),
        };

        let word_part = captures.get(1).map_or("", |m| m.as_str());

        let is_profane = dictionary.simple_words.contains(word_part)
            || dictionary
                .regexes
                .iter()
                .any(|re| re.is_match(word_part) || re.is_match(&lower_word));

        if !is_profane {
            return word.to_string();
        }

        // Use the character count (not byte length) so multibyte alphabets such
        // as Cyrillic are masked at the correct boundaries.
        let word_part_len = word_part.chars().count();

        word.chars()
            .enumerate()
            .map(|(i, c)| {
                if i > 0 && i + 1 < word_part_len {
                    self.replace_char
                } else {
                    c
                }
            })
            .collect()
    }
}
