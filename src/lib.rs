use rustrict::CensorStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn check_curse_words(text: &str) -> bool {
    let censored_text = text.is_inappropriate();
    censored_text
}

#[wasm_bindgen]
pub fn replace_curse_words(text: &str) -> String {
    let censored_text = text.censor();
    censored_text
}
