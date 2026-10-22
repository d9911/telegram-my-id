// Use Case: Test - тестирование функций бота

use std::sync::Arc;
use teloxide::types::User;

// Импортируем репозиторий из infrastructure
// Используем конкретный тип вместо trait object, так как async traits не dyn compatible
use crate::infrastructure::persistence::FileLanguageRepository;
// Импортируем trait, чтобы использовать его методы
use crate::domain::repositories::LanguageRepository;

// Структура use case
pub struct TestUseCase {
    // Репозиторий для получения статистики
    // Используем конкретный тип вместо dyn LanguageRepository
    // В Rust async traits не dyn compatible без специальных макросов
    language_repository: Arc<FileLanguageRepository>,
}

impl TestUseCase {
    // Конструктор
    // FileLanguageRepository - конкретная реализация репозитория
    // В Rust async traits не dyn compatible, поэтому используем конкретный тип
    pub fn new(language_repository: Arc<FileLanguageRepository>) -> Self {
        Self {
            language_repository,
        }
    }

    // Выполняет use case
    // locale_getter - функция для получения переводов
    pub async fn execute<F>(
        &self,
        user: Option<&User>,
        current_lang: &str,
        locale_getter: F,
    ) -> String
    where
        F: Fn(&str) -> &str,
    {
        // format! - макрос для форматирования строк (как template literals в JS)
        let mut test_info = format!("🧪 <b>{}</b>\n\n", locale_getter("test_title"));

        // Статус бота
        test_info.push_str(&format!("✅ <b>{}</b>\n", locale_getter("test_status")));
        test_info.push_str(&format!("  • {}: ✅\n", locale_getter("test_bot_online")));
        test_info.push_str(&format!("  • {}: ✅\n", locale_getter("test_translations")));
        test_info.push_str(&format!("  • {}: ✅\n\n", locale_getter("test_formatting")));

        // Текущий язык
        test_info.push_str(&format!(
            "🌐 <b>{}</b>\n",
            locale_getter("test_current_lang")
        ));
        test_info.push_str(&format!(
            "  {}: <code>{}</code>\n\n",
            locale_getter("test_selected_lang"),
            current_lang
        ));

        // Примеры переводов на всех языках
        test_info.push_str(&format!(
            "📝 <b>{}</b>\n",
            locale_getter("test_translations_examples")
        ));
        for lang_code in ["ru", "en", "es", "ko"] {
            // Для каждого языка получаем перевод
            let lang_name = match lang_code {
                "ru" => "🇷🇺 Русский",
                "en" => "🇺🇸 English",
                "es" => "🇪🇸 Español",
                "ko" => "🇰🇷 한국어",
                _ => "",
            };
            test_info.push_str(&format!(
                "  {}: \"{}\"\n",
                lang_name,
                locale_getter("welcome_message")
            ));
        }
        test_info.push('\n');

        // Тест форматирования ID
        test_info.push_str(&format!(
            "📋 <b>{}</b>\n",
            locale_getter("test_formatting_demo")
        ));
        test_info.push_str(&format!("  {}:\n", locale_getter("id")));
        test_info.push_str("  <code>123456789</code>\n\n");

        // Информация о пользователе (если доступна)
        if let Some(user) = user {
            test_info.push_str(&format!("👤 <b>{}</b>\n", locale_getter("test_user_info")));
            test_info.push_str(&format!(
                "  {}: <code>{}</code>\n",
                locale_getter("id"),
                user.id
            ));
            test_info.push_str(&format!(
                "  {}: {}\n",
                locale_getter("first_name"),
                user.first_name
            ));
            if let Some(username) = &user.username {
                test_info.push_str(&format!("  {}: @{}\n", locale_getter("username"), username));
            }
            test_info.push('\n');
        }

        // Статистика языков
        let languages = self.language_repository.get_all_languages().await;
        test_info.push_str(&format!("📊 <b>{}</b>\n", locale_getter("test_statistics")));
        test_info.push_str(&format!(
            "  {}: {}\n",
            locale_getter("test_users_count"),
            languages.len()
        ));

        test_info.push_str(&format!("\n💡 <i>{}</i>", locale_getter("test_footer")));

        test_info
    }
}
