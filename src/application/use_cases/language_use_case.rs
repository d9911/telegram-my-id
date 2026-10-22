// Use Case: Language - работа с языками пользователей

use std::sync::Arc;
use teloxide::types::UserId;

// Импортируем репозиторий и сервисы
// Используем конкретный тип вместо trait object, так как async traits не dyn compatible
use crate::domain::services::LocaleService;
use crate::infrastructure::persistence::FileLanguageRepository;
// Импортируем trait, чтобы использовать его методы
use crate::domain::repositories::LanguageRepository;

// Структура use case
pub struct LanguageUseCase {
    // Используем конкретный тип вместо dyn LanguageRepository
    // В Rust async traits не dyn compatible без специальных макросов (async-trait crate)
    language_repository: Arc<FileLanguageRepository>,
    locale_service: Arc<LocaleService>,
}

impl LanguageUseCase {
    // Конструктор
    pub fn new(
        language_repository: Arc<FileLanguageRepository>,
        locale_service: Arc<LocaleService>,
    ) -> Self {
        Self {
            language_repository,
            locale_service,
        }
    }

    // Получает язык пользователя
    // Сначала проверяет сохраненный язык, потом язык из профиля Telegram
    pub async fn get_user_language(&self, user_id: UserId, user_lang_code: Option<&str>) -> String {
        // Проверяем сохраненный язык
        if let Some(lang) = self.language_repository.get_language(user_id).await {
            return lang;
        }

        // Если сохраненного языка нет, используем язык из профиля
        if let Some(lang_code) = user_lang_code {
            return self
                .locale_service
                .detect_language(Some(lang_code))
                .to_string();
        }

        // По умолчанию английский
        "en".to_string()
    }

    // Сохраняет язык пользователя
    pub async fn save_user_language(
        &self,
        user_id: UserId,
        lang: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Валидация языка
        if !["ru", "en", "es", "ko"].contains(&lang.as_str()) {
            return Err("Неверный код языка".into()); // into() - автоматическое преобразование типов
        }

        // Сохраняем через репозиторий
        self.language_repository.save_language(user_id, lang).await
    }
}
