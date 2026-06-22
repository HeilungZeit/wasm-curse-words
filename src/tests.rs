use crate::WasmCurse;

pub mod tests {
    use super::*;

    #[test]
    fn test_check_curse_words() {
        let wasm_curse = WasmCurse::new(Some('*'), vec!["en".to_string()].into());
        assert_eq!(wasm_curse.check_curse_words("Hello, world!"), false);
        assert_eq!(wasm_curse.check_curse_words("This is a test."), false);
        assert_eq!(wasm_curse.check_curse_words("You're an asshole!"), true);
        assert_eq!(wasm_curse.check_curse_words("What the hell?"), true);

        let wasm_curse_ru = WasmCurse::new(Some('*'), vec!["ru".to_string()].into());
        assert_eq!(wasm_curse_ru.check_curse_words("Привет!"), false);
        assert_eq!(wasm_curse_ru.check_curse_words("Привет, пидр!"), true);
        assert_eq!(wasm_curse_ru.check_curse_words("пидарас"), true);
        assert_eq!(wasm_curse_ru.check_curse_words("хуесосина"), true);
    }

    #[test]
    fn test_replace_curse_words() {
        let wasm_curse = WasmCurse::new(
            Some('*'),
            vec!["en".to_string(), "ru".to_string(), "de".to_string()].into(),
        );
        assert_eq!(
            wasm_curse.replace_curse_words("Hello, world!"),
            "Hello, world!"
        );
        assert_eq!(
            wasm_curse.replace_curse_words("This is a test."),
            "This is a test."
        );
        assert_eq!(
            wasm_curse.replace_curse_words("You're a fucking asshole!"),
            "You're a f****** a******!"
        );
        assert_eq!(
            wasm_curse.replace_curse_words("What the hell?"),
            "What the h***?"
        );
        assert_eq!(
            wasm_curse.replace_curse_words("Блять давай... ах ты сука!"),
            "Б***ь давай... ах ты с**а!"
        );
        assert_eq!(
            wasm_curse.replace_curse_words("не выебывайся"),
            "не в********я"
        );
        assert_eq!(wasm_curse.replace_curse_words("Мать ебал?"), "Мать е**л?");
        assert_eq!(
            wasm_curse.replace_curse_words("kurwa jebana"),
            "k***a j****a"
        );

        assert_eq!(
            wasm_curse.replace_curse_words("Leck mich am Arsch!"),
            "L**k mich am A***h!"
        );
    }

    #[test]
    fn define_language() {
        let wasm_curse_lang_def_en = WasmCurse::new(Some('*'), vec!["en".to_string()].into());
        assert_eq!(
            wasm_curse_lang_def_en.define_language("What the hell is this?"),
            "en"
        );
        assert_eq!(
            wasm_curse_lang_def_en.define_language("Давай давай нападай!"),
            "ru"
        );
        assert_eq!(
            wasm_curse_lang_def_en.define_language("Die Sonne scheint mir aus den Händen"),
            "de"
        );
    }

    #[test]
    fn test_german_regex_patterns() {
        let wasm_curse_de = WasmCurse::new(Some('*'), vec!["de".to_string()].into());
        
        // Test \w* patterns - words ending with pattern
        assert_eq!(wasm_curse_de.check_curse_words("verficktnochmal"), true);
        assert_eq!(wasm_curse_de.check_curse_words("arschgesicht"), true);
        assert_eq!(wasm_curse_de.check_curse_words("hurensohn"), true);
        
        // Test \W* patterns - words with spaces/punctuation
        assert_eq!(wasm_curse_de.check_curse_words("leck mich am arsch"), true);
        assert_eq!(wasm_curse_de.check_curse_words("geht sterben"), true);
        
        // Test alternation patterns like schei(ss|ß)e
        assert_eq!(wasm_curse_de.check_curse_words("scheisse"), true);
        assert_eq!(wasm_curse_de.check_curse_words("scheiße"), true);
        
        // Test simple exact matches
        assert_eq!(wasm_curse_de.check_curse_words("arschloch"), true);
        assert_eq!(wasm_curse_de.check_curse_words("schlampe"), true);
        
        // Test non-profane words
        assert_eq!(wasm_curse_de.check_curse_words("Guten Tag"), false);
        assert_eq!(wasm_curse_de.check_curse_words("Wie geht es dir"), false);
    }

    #[test]
    fn test_russian_regex_patterns() {
        let wasm_curse_ru = WasmCurse::new(Some('*'), vec!["ru".to_string()].into());
        
        // Test \w* and \w+ patterns
        assert_eq!(wasm_curse_ru.check_curse_words("ебать"), true);
        assert_eq!(wasm_curse_ru.check_curse_words("ебало"), true);
        assert_eq!(wasm_curse_ru.check_curse_words("гавно"), true);
        assert_eq!(wasm_curse_ru.check_curse_words("гавноед"), true);
        assert_eq!(wasm_curse_ru.check_curse_words("ахуеть"), true);
        
        // Test + patterns like бля+
        assert_eq!(wasm_curse_ru.check_curse_words("бляяя"), true);
        assert_eq!(wasm_curse_ru.check_curse_words("блять"), true);
        
        // Test phrases with spaces
        assert_eq!(wasm_curse_ru.check_curse_words("пиздюлей навешать"), true);
        assert_eq!(wasm_curse_ru.check_curse_words("какого хуя"), true);
        assert_eq!(wasm_curse_ru.check_curse_words("пизды дать"), true);
        
        // Test exact matches
        assert_eq!(wasm_curse_ru.check_curse_words("сука"), true);
        assert_eq!(wasm_curse_ru.check_curse_words("пидр"), true);
        
        // Test non-profane words
        assert_eq!(wasm_curse_ru.check_curse_words("Привет как дела"), false);
        assert_eq!(wasm_curse_ru.check_curse_words("хороший день"), false);
    }

    #[test]
    fn test_polish_regex_patterns() {
        let wasm_curse_pl = WasmCurse::new(Some('*'), vec!["pl".to_string()].into());
        
        // Test [ey] pattern: popierdolon[ey]
        assert_eq!(wasm_curse_pl.check_curse_words("popierdolone to jest"), true);
        assert_eq!(wasm_curse_pl.check_curse_words("popierdolony on"), true);
        
        // Test exact matches
        assert_eq!(wasm_curse_pl.check_curse_words("jebana jest"), true);
        assert_eq!(wasm_curse_pl.check_curse_words("pojebana"), true);
        assert_eq!(wasm_curse_pl.check_curse_words("zajebana"), true);
        
        // Test non-profane words
        assert_eq!(wasm_curse_pl.check_curse_words("Dzień dobry"), false);
        assert_eq!(wasm_curse_pl.check_curse_words("Jak się masz"), false);
    }

    #[test]
    fn test_custom_replace_char() {
        let wasm_curse_ru = WasmCurse::new(Some('#'), vec!["ru".to_string()].into());
        assert_eq!(wasm_curse_ru.replace_curse_words("сука"), "с##а");

        let wasm_curse_en = WasmCurse::new(Some('#'), vec!["en".to_string()].into());
        assert_eq!(
            wasm_curse_en.replace_curse_words("What the hell?"),
            "What the h###?"
        );
    }

    #[test]
    fn test_default_replace_char() {
        let wasm_curse = WasmCurse::new(None, None);
        assert_eq!(
            wasm_curse.replace_curse_words("What the hell?"),
            "What the h***?"
        );
    }

    #[test]
    fn test_empty_and_clean_input() {
        let wasm_curse = WasmCurse::new(Some('*'), vec!["en".to_string(), "ru".to_string()].into());

        // Empty input must not panic and must be treated as non-profane.
        assert_eq!(wasm_curse.define_language(""), "en");
        assert_eq!(wasm_curse.check_curse_words(""), false);
        assert_eq!(wasm_curse.replace_curse_words(""), "");

        // Clean multibyte text stays untouched.
        assert_eq!(wasm_curse.check_curse_words("Привет, как дела?"), false);
        assert_eq!(
            wasm_curse.replace_curse_words("Привет, как дела?"),
            "Привет, как дела?"
        );
    }

    #[test]
    fn test_language_isolation() {
        // Only English is enabled, so the Russian dictionary must never be used.
        let wasm_curse_en = WasmCurse::new(Some('*'), vec!["en".to_string()].into());
        assert_eq!(wasm_curse_en.check_curse_words("пидарас"), false);
        assert_eq!(wasm_curse_en.replace_curse_words("пидарас"), "пидарас");
    }

    #[test]
    fn test_multiple_curse_words_replace() {
        let wasm_curse_ru = WasmCurse::new(Some('*'), vec!["ru".to_string()].into());
        assert_eq!(
            wasm_curse_ru.replace_curse_words("сука и пидарас"),
            "с**а и п*****с"
        );
    }

    #[test]
    fn test_polish_replace() {
        let wasm_curse_pl = WasmCurse::new(Some('*'), vec!["pl".to_string()].into());
        assert_eq!(wasm_curse_pl.replace_curse_words("kurwa"), "k***a");
    }
}
