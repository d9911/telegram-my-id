// Handler для обработки callback query (нажатия на inline-кнопки)
// Обрабатывает выбор языка через кнопки

use std::sync::Arc;
use teloxide::prelude::*;

// Импортируем типы и use cases
use crate::application::use_cases::LanguageUseCase;
use crate::locales;
use crate::shared::HandlerResult;

// Структура handler'а
pub struct CallbackHandler {
    language_use_case: Arc<LanguageUseCase>,
}

impl CallbackHandler {
    // Конструктор
    pub fn new(language_use_case: Arc<LanguageUseCase>) -> Self {
        Self { language_use_case }
    }

    // Обрабатывает callback query
    pub async fn handle(&self, bot: Bot, q: CallbackQuery) -> HandlerResult {
        // Проверяем, что callback содержит данные
        if let Some(data) = q.data {
            // Извлекаем язык из callback data (формат: "lang_ru", "lang_en", и т.д.)
            let lang = data.strip_prefix("lang_").unwrap_or(&data);

            // Проверяем, что язык валидный
            if !["ru", "en", "es", "ko"].contains(&lang) {
                return Ok(()); // Если язык невалидный, просто выходим
            }

            let user_id = q.from.id;

            // Сохраняем язык через use case
            if let Err(e) = self
                .language_use_case
                .save_user_language(user_id, lang.to_string())
                .await
            {
                log::error!("Ошибка сохранения языка: {}", e);
                // Продолжаем выполнение, даже если не удалось сохранить
            }

            // Получаем локализацию для ответа
            let locale = locales::get_locale(lang);
            let response = format!(
                "{}\n\n{}",
                locale.get("language_set"),
                locale.get("language_confirmed")
            );

            // Отвечаем на callback query (показываем уведомление)
            bot.answer_callback_query(q.id)
                .text(locale.get("language_set_short"))
                .await?;

            // Обновляем сообщение с выбором языка
            if let Some(msg) = q.message {
                bot.edit_message_text(msg.chat().id, msg.id(), response)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await?;
            }
        }
        Ok(())
    }
}
