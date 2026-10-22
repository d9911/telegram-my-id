// Use Case: InfoUser - получение и форматирование информации о пользователе

use std::sync::Arc;
use teloxide::types::User;

// Импортируем сервисы из domain
use crate::domain::services::{LocaleService, UserService};

// Структура use case
pub struct InfoUserUseCase {
    // Зависимости (dependency injection)
    user_service: Arc<UserService>,
    #[allow(dead_code)] // Поле может использоваться в будущем для расширения функциональности
    locale_service: Arc<LocaleService>,
}

impl InfoUserUseCase {
    // Конструктор
    pub fn new(user_service: Arc<UserService>, locale_service: Arc<LocaleService>) -> Self {
        Self {
            user_service,
            locale_service,
        }
    }

    // Выполняет use case
    // user - пользователь, для которого нужно получить информацию
    // _lang - язык интерфейса (не используется напрямую, но может быть полезен в будущем)
    // locale_getter - функция для получения перевода по ключу
    // Fn(&str) -> &str - это closure (функция), которая принимает ключ и возвращает перевод
    // В TypeScript это было бы: (key: string) => string
    pub fn execute<F>(&self, user: &User, _lang: &str, locale_getter: F) -> String
    where
        F: Fn(&str) -> &str, // F - это generic параметр, который должен быть функцией
    {
        // Вызываем сервис для форматирования информации
        // В Rust методы вызываются через точку (как в JS)
        self.user_service.format_user_info(user, locale_getter)
    }
}
