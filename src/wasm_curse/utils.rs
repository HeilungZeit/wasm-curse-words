// нельзя сделать динмаческие пути из-за того, что метод include_str! обрабатывается на этапе компиляции
pub fn get_ru_dictionary() -> Vec<&'static str> {
    let russian_words = include_str!("../curseDictionaries/filter_profanity_russian.txt");
    russian_words.lines().collect()
}

pub fn get_de_dictionary() -> Vec<&'static str> {
    let german_words = include_str!("../curseDictionaries/filter_profanity_german.txt");
    german_words.lines().collect()
}

pub fn get_pl_dictionary() -> Vec<&'static str> {
    let polish_words = include_str!("../curseDictionaries/filter_profanity_polish.txt");
    polish_words.lines().collect()
}

pub fn remove_all_symbols(text: String) -> Vec<String> {
    text.split_whitespace()
        .map(|word| {
            let mut word = word.to_string();
            word.retain(|c| c.is_alphabetic());
            word
        })
        .collect::<Vec<String>>()
}
