use once_cell::sync::Lazy;
use std::collections::HashSet;

// Cache dictionaries as HashSet for O(1) lookup instead of O(n)
static RU_DICTIONARY: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let russian_words = include_str!("../curseDictionaries/filter_profanity_russian.txt");
    russian_words.lines().collect()
});

static DE_DICTIONARY: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let german_words = include_str!("../curseDictionaries/filter_profanity_german.txt");
    german_words.lines().collect()
});

static PL_DICTIONARY: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let polish_words = include_str!("../curseDictionaries/filter_profanity_polish.txt");
    polish_words.lines().collect()
});

pub fn get_ru_dictionary() -> &'static HashSet<&'static str> {
    &RU_DICTIONARY
}

pub fn get_de_dictionary() -> &'static HashSet<&'static str> {
    &DE_DICTIONARY
}

pub fn get_pl_dictionary() -> &'static HashSet<&'static str> {
    &PL_DICTIONARY
}

pub fn remove_all_symbols(text: String) -> Vec<String> {
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
