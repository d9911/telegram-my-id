pub mod ru;
pub mod en;
pub mod es;
pub mod ko;

use std::collections::HashMap;

pub type Translations = HashMap<&'static str, &'static str>;

pub struct Locale {
    pub translations: Translations,
}

impl Locale {
    pub fn get(&self, key: &str) -> &'static str {
        // Все ключи должны существовать в переводах, поэтому используем unwrap
        // Если ключ не найден, это ошибка программиста
        self.translations.get(key).copied().unwrap_or_else(|| {
            eprintln!("Warning: Translation key '{}' not found", key);
            "" // Возвращаем пустую строку как fallback
        })
    }
}

pub fn get_locale(lang: &str) -> Locale {
    match lang {
        "ru" => Locale {
            translations: ru::get_translations(),
        },
        "en" => Locale {
            translations: en::get_translations(),
        },
        "es" => Locale {
            translations: es::get_translations(),
        },
        "ko" => Locale {
            translations: ko::get_translations(),
        },
        _ => Locale {
            translations: en::get_translations(), // По умолчанию английский
        },
    }
}

pub fn detect_language(user_lang: Option<&str>) -> &'static str {
    match user_lang {
        Some("ru") | Some("ru-RU") => "ru",
        Some("en") | Some("en-US") | Some("en-GB") => "en",
        Some("es") | Some("es-ES") | Some("es-MX") => "es",
        Some("ko") | Some("ko-KR") => "ko",
        _ => "en", // По умолчанию английский
    }
}

