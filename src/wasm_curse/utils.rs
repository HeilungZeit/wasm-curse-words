use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

/// A dictionary split into exact-match words and pre-compiled regex patterns.
/// Both are built once and reused to avoid recompiling regexes on every call.
pub struct CompiledDictionary {
    pub simple_words: HashSet<&'static str>,
    pub regexes: Vec<Regex>,
}

fn compile_dictionary(raw: &'static str) -> CompiledDictionary {
    let mut simple_words = HashSet::new();
    let mut regexes = Vec::new();

    for word in raw.lines() {
        if word.is_empty() {
            continue;
        }

        if is_regex_pattern(word) {
            if let Ok(re) = Regex::new(word) {
                regexes.push(re);
            }
        } else {
            simple_words.insert(word);
        }
    }

    CompiledDictionary {
        simple_words,
        regexes,
    }
}

static RU_DICTIONARY: Lazy<CompiledDictionary> = Lazy::new(|| {
    compile_dictionary(include_str!("../curseDictionaries/filter_profanity_russian.txt"))
});

static DE_DICTIONARY: Lazy<CompiledDictionary> = Lazy::new(|| {
    compile_dictionary(include_str!("../curseDictionaries/filter_profanity_german.txt"))
});

static PL_DICTIONARY: Lazy<CompiledDictionary> = Lazy::new(|| {
    compile_dictionary(include_str!("../curseDictionaries/filter_profanity_polish.txt"))
});

/// Cached splitter that separates a token into its word part and trailing punctuation.
pub static WORD_SPLIT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w+)(\W*)").unwrap());

/// Returns the cached, compiled dictionary for a supported language code.
pub fn get_dictionary(lang: &str) -> Option<&'static CompiledDictionary> {
    match lang {
        "ru" => Some(&RU_DICTIONARY),
        "de" => Some(&DE_DICTIONARY),
        "pl" => Some(&PL_DICTIONARY),
        _ => None,
    }
}

pub fn remove_all_symbols(text: &str) -> Vec<String> {
    text.split_whitespace()
        .filter_map(|word| {
            let clean: String = word.chars().filter(|c| c.is_alphabetic()).collect();
            if clean.is_empty() {
                None
            } else {
                Some(clean)
            }
        })
        .collect()
}

pub fn is_regex_pattern(word: &str) -> bool {
    word.contains(r"\w")
        || word.contains(r"\W")
        || word.contains(r"\d")
        || word.contains(r"\D")
        || word.contains(r"\s")
        || word.contains(r"\S")
        || word.contains('(')
        || word.contains(')')
        || word.contains('[')
        || word.contains(']')
        || word.contains('{')
        || word.contains('}')
        || word.contains('?')
        || word.contains('+')
        || word.contains('*')
        || word.contains('|')
        || word.contains('^')
        || word.contains('$')
        || word.contains('.')
        || word.contains(' ')
}
