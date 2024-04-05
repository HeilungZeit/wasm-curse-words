use phf::phf_map;

pub static LANGS: phf::Map<&'static str, &'static str> = phf_map! {
    "Russian" => "ru",
    "English" => "en",
    "German" => "de",
    "ru" => "russian",
    "de" => "german",
};
