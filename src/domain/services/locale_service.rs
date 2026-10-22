// Service для работы с локализацией
// Инкапсулирует логику определения языка пользователя

// В Rust структура (struct) - это как class в TypeScript/PHP, но без методов по умолчанию
// Arc - это Atomic Reference Counter, умный указатель для многопоточности
// Аналогично shared_ptr в C++ или просто ссылка в JS (но с подсчетом ссылок)
pub struct LocaleService {
    // В Rust поля структуры приватные по умолчанию
    // pub делает их публичными (как public в TypeScript)
}

// impl - блок реализации методов для структуры (как методы класса)
impl LocaleService {
    // new() - конструктор (паттерн в Rust)
    // Self - это как this.constructor в JS, но для типа
    pub fn new() -> Self {
        Self {}
    }

    // Определяет язык пользователя на основе его языка в Telegram
    // &str - это строковый срез (как string в TypeScript, но без владения)
    // Option<&str> - это как string | null в TypeScript
    pub fn detect_language(&self, user_lang: Option<&str>) -> &'static str {
        // match - это как switch в JS, но более мощный (pattern matching)
        match user_lang {
            // Some(value) - это как проверка на null: if (value !== null)
            // | означает "или" (как case fallthrough в switch)
            Some("ru") | Some("ru-RU") => "ru",
            Some("en") | Some("en-US") | Some("en-GB") => "en",
            Some("es") | Some("es-ES") | Some("es-MX") => "es",
            Some("ko") | Some("ko-KR") => "ko",
            // _ - это как default в switch (catch-all pattern)
            _ => "en", // По умолчанию английский
        }
    }
}

// Default trait - позволяет использовать LocaleService::default()
// Это как default parameters в конструкторе
impl Default for LocaleService {
    fn default() -> Self {
        Self::new()
    }
}
