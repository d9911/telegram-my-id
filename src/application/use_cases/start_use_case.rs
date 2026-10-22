// Use Case: Start - обработка команды /start
// Определяет язык пользователя и возвращает приветственное сообщение

use std::sync::Arc;
use teloxide::types::User;

// Импортируем сервисы из domain
use crate::domain::services::LocaleService;

// Структура use case
// В Rust структуры могут содержать зависимости (dependency injection)
pub struct StartUseCase {
    // Arc - умный указатель для многопоточности (как shared reference)
    // LocaleService - сервис для работы с локализацией
    locale_service: Arc<LocaleService>,
}

impl StartUseCase {
    // Конструктор с dependency injection
    // В Rust принято передавать зависимости через конструктор
    pub fn new(locale_service: Arc<LocaleService>) -> Self {
        Self { locale_service }
    }

    // Выполняет use case
    // &self - ссылка на сам объект (не владеем, только читаем)
    // Option<&User> - пользователь может быть None (null)
    // -> (String, String) - возвращает кортеж из двух строк
    // В Rust кортежи - это как [string, string] в TypeScript, но типизированные
    pub fn execute(&self, user: Option<&User>) -> (String, String) {
        // Определяем язык пользователя
        let user_lang = if let Some(user) = user {
            // Если пользователь есть, проверяем его язык
            if let Some(lang_code) = &user.language_code {
                // detect_language - определяет язык из кода (ru-RU -> ru)
                self.locale_service.detect_language(Some(lang_code))
            } else {
                "en" // По умолчанию английский
            }
        } else {
            "en" // Если пользователя нет, английский
        };

        // Возвращаем код языка и ключ для приветственного сообщения
        // В Rust можно возвращать кортеж напрямую (без return)
        (user_lang.to_string(), "welcome_message".to_string())
    }
}
