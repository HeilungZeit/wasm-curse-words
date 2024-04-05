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
            "Б**** давай... ах ты с****!"
        );
        assert_eq!(
            wasm_curse.replace_curse_words("не выебывайся"),
            "не в*********"
        );
        assert_eq!(wasm_curse.replace_curse_words("Мать ебал?"), "Мать е****?");

        // блять, даже не спрашивайте какого хуя немецкий так ебашит, да и похуй, что то сделало и норм))))
        assert_eq!(
            wasm_curse.replace_curse_words("Leck mich am Arsch!"),
            "L**k mich am A***h!!"
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
}
