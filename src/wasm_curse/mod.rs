use std::collections::HashSet;

use lingua::Language::{English, German, Polish, Russian};
use lingua::LanguageDetectorBuilder;
use regex::Regex;
use rustrict::{Censor, Type};
use wasm_bindgen::prelude::*;

mod statics;
mod utils;

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
        if (self.languages_to_check.contains(&String::from("ru"))
            || self.languages_to_check.contains(&String::from("de")))
            && self.define_language(text) != "en"
        {
            let dictionary = match self.define_language(text).as_str() {
                "ru" => utils::get_ru_dictionary(),
                "de" => utils::get_de_dictionary(),
                _ => vec![],
            };

            let text = text.to_lowercase();

            let all_words: String = utils::remove_all_symbols(text).join(" ");

            let words_in_text: Vec<&str> = all_words.split_whitespace().collect();

            let mut is_inappropriate = false;

            for word in words_in_text {
                for &dictionary_word in &dictionary {
                    if dictionary_word.starts_with("\\w*") {
                        let re = Regex::new(&dictionary_word[3..]).unwrap();
                        if re.is_match(word) {
                            is_inappropriate = true;
                            break;
                        }
                    } else if word == dictionary_word {
                        is_inappropriate = true;
                        break;
                    }
                }
                if is_inappropriate {
                    break;
                }
            }
            return is_inappropriate;
        }

        let analysis = Censor::from_str(text)
            .with_censor_replacement(self.replace_char)
            .analyze();

        analysis.is(Type::INAPPROPRIATE)
    }

    #[wasm_bindgen(js_name = defineLanguage)]
    pub fn define_language(&self, text: &str) -> String {
        let languages = vec![English, German, Russian, Polish];
        let detector = LanguageDetectorBuilder::from_languages(&languages).build();
        let detected_language = detector.detect_language_of(text);
        let lang = detected_language.unwrap().to_string();

        if !statics::LANGS.contains_key(lang.as_str()) {
            panic!("Language not found in statics: {}", lang);
        }

        statics::LANGS[lang.as_str()].to_string()
    }

    #[wasm_bindgen(js_name = replaceCurseWords)]
    pub fn replace_curse_words(&self, text: &str) -> String {
        let language = self.define_language(text);

        let languages_to_check = self
            .languages_to_check
            .iter()
            .map(|s| s.to_string())
            .collect::<HashSet<_>>();
        let non_english_languages = ["ru", "de", "pl"]
            .iter()
            .map(|s| s.to_string())
            .collect::<HashSet<_>>();

        if non_english_languages
            .intersection(&languages_to_check)
            .count()
            > 0
            && language != "en"
        {
            let dictionary = match language.as_str() {
                "ru" => utils::get_ru_dictionary()
                    .into_iter()
                    .collect::<HashSet<_>>(),
                "de" => utils::get_de_dictionary()
                    .into_iter()
                    .collect::<HashSet<_>>(),
                "pl" => utils::get_pl_dictionary()
                    .into_iter()
                    .collect::<HashSet<_>>(),
                _ => HashSet::new(),
            };

            let mut replaced_words = Vec::new();
            let re = Regex::new(r"(\w+)(\W*)").unwrap();

            for word in text.split(" ").map(String::from).collect::<Vec<String>>() {
                let mut replaced = word.to_string();
                let lower_word = word.to_lowercase();

                if let Some(captures) = re.captures(&lower_word) {
                    let word_part = captures.get(1).map_or("", |m| m.as_str());
                    let punctuation_part = captures.get(2).map_or("", |m| m.as_str());

                    for &dictionary_word in &dictionary {
                        let lower_dictionary_word = dictionary_word.to_lowercase();
                        if lower_dictionary_word.starts_with("\\w*")
                            || lower_dictionary_word.ends_with("\\w*")
                        {
                            let re = Regex::new(&lower_dictionary_word).unwrap();
                            if re.is_match(word_part) {
                                replaced = word
                                    .chars()
                                    .enumerate()
                                    .map(|(i, c)| {
                                        if i > 0 && i < word_part.len() - 1 {
                                            '*'
                                        } else {
                                            c
                                        }
                                    })
                                    .collect::<String>();
                                replaced.push_str(punctuation_part);
                            }
                        } else if word_part == lower_dictionary_word {
                            replaced = word
                                .chars()
                                .enumerate()
                                .map(|(i, c)| {
                                    if i > 0 && i < word_part.len() - 1 {
                                        '*'
                                    } else {
                                        c
                                    }
                                })
                                .collect::<String>();
                            replaced.push_str(punctuation_part);
                        }
                    }
                }
                replaced_words.push(replaced);
            }
            return replaced_words.join(" ");
        }

        let censored = Censor::from_str(text)
            .with_censor_replacement(self.replace_char)
            .censor();

        censored
    }
}
