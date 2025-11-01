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
        let lang = self.define_language(text);
        
        if (self.languages_to_check.contains(&String::from("ru"))
            || self.languages_to_check.contains(&String::from("de"))
            || self.languages_to_check.contains(&String::from("pl")))
            && lang != "en"
        {
            let dictionary = match lang.as_str() {
                "ru" => utils::get_ru_dictionary(),
                "de" => utils::get_de_dictionary(),
                "pl" => utils::get_pl_dictionary(),
                _ => return false,
            };

            let text_lower = text.to_lowercase();
            
            let regex_patterns: Vec<_> = dictionary
                .iter()
                .filter(|&&word| utils::is_regex_pattern(word))
                .collect();
            
            let compiled_regexes: Vec<Regex> = regex_patterns
                .iter()
                .filter_map(|&&pattern| Regex::new(pattern).ok())
                .collect();
            
            for re in &compiled_regexes {
                if re.is_match(&text_lower) {
                    return true;
                }
            }
            
            let all_words = utils::remove_all_symbols(text_lower);
            for word in &all_words {
                if dictionary.contains(word.as_str()) {
                    return true;
                }
                
                for re in &compiled_regexes {
                    if re.is_match(word) {
                        return true;
                    }
                }
            }
            
            return false;
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

        let has_non_english = self.languages_to_check.iter()
            .any(|lang| lang == "ru" || lang == "de" || lang == "pl");

        if has_non_english && language != "en" {
            let dictionary = match language.as_str() {
                "ru" => utils::get_ru_dictionary(),
                "de" => utils::get_de_dictionary(),
                "pl" => utils::get_pl_dictionary(),
                _ => return text.to_string(),
            };

            let word_re = Regex::new(r"(\w+)(\W*)").unwrap();
            
            let (simple_words, regex_patterns): (Vec<_>, Vec<_>) = dictionary
                .iter()
                .partition(|&&word| !utils::is_regex_pattern(word));
            
            let compiled_regexes: Vec<Regex> = regex_patterns
                .iter()
                .filter_map(|&&pattern| Regex::new(pattern).ok())
                .collect();

            let replaced_words: Vec<String> = text.split(' ')
                .map(|word| {
                    let lower_word = word.to_lowercase();

                    if let Some(captures) = word_re.captures(&lower_word) {
                        let word_part = captures.get(1).map_or("", |m| m.as_str());
                        let punctuation_part = captures.get(2).map_or("", |m| m.as_str());

                        // Fast exact match with simple words
                        let mut is_profane = simple_words.iter().any(|&&w| w == word_part);
                        
                        // Check regex patterns if not found
                        if !is_profane {
                            is_profane = compiled_regexes.iter()
                                .any(|re| re.is_match(word_part) || re.is_match(&lower_word));
                        }

                        if is_profane {
                            let mut censored: String = word
                                .chars()
                                .enumerate()
                                .map(|(i, c)| {
                                    if i > 0 && i < word_part.len() - 1 {
                                        '*'
                                    } else {
                                        c
                                    }
                                })
                                .collect();
                            censored.push_str(punctuation_part);
                            return censored;
                        }
                    }
                    word.to_string()
                })
                .collect();
            
            return replaced_words.join(" ");
        }

        let censored = Censor::from_str(text)
            .with_censor_replacement(self.replace_char)
            .censor();

        censored
    }
}
