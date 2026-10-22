// Handler для обработки обычных сообщений (не команд)
// Обрабатывает ответы пользователей на запросы бота

use std::collections::HashMap;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::UserId;
use tokio::sync::RwLock;

// Импортируем типы и use cases
use crate::application::use_cases::*;
use crate::locales;
use crate::shared::HandlerResult;

// Типы для хранения состояния ожидающих пользователей
type WaitingForRange = Arc<RwLock<HashMap<UserId, bool>>>;
type WaitingForChoice = Arc<RwLock<HashMap<UserId, bool>>>;

// Структура handler'а
pub struct MessageHandler {
    // Use cases для обработки сообщений
    random_number_use_case: Arc<RandomNumberUseCase>,
    random_choice_use_case: Arc<RandomChoiceUseCase>,
    language_use_case: Arc<LanguageUseCase>,

    // Состояние ожидающих пользователей
    waiting_for_range: WaitingForRange,
    waiting_for_choice: WaitingForChoice,
}

impl MessageHandler {
    // Конструктор
    pub fn new(
        random_number_use_case: Arc<RandomNumberUseCase>,
        random_choice_use_case: Arc<RandomChoiceUseCase>,
        language_use_case: Arc<LanguageUseCase>,
        waiting_for_range: WaitingForRange,
        waiting_for_choice: WaitingForChoice,
    ) -> Self {
        Self {
            random_number_use_case,
            random_choice_use_case,
            language_use_case,
            waiting_for_range,
            waiting_for_choice,
        }
    }

    // Обрабатывает сообщение
    pub async fn handle(&self, bot: Bot, msg: Message) -> HandlerResult {
        // Проверяем, что сообщение содержит текст
        if let Some(text) = msg.text() {
            // Пропускаем команды (они обрабатываются в command_handler)
            if text.starts_with('/') {
                return Ok(());
            }

            if let Some(user) = msg.from.as_ref() {
                let user_id = user.id;

                // Получаем язык пользователя
                let user_lang_code = user.language_code.as_deref();
                let lang = self
                    .language_use_case
                    .get_user_language(user_id, user_lang_code)
                    .await;
                let locale = locales::get_locale(&lang);

                // Проверяем, ожидает ли пользователь ввода диапазона для random_number
                {
                    let waiting = self.waiting_for_range.read().await;
                    if waiting.get(&user_id).copied().unwrap_or(false) {
                        // Пользователь ожидает ввода диапазона
                        drop(waiting); // Освобождаем блокировку чтения

                        // Парсим диапазон через use case
                        if let Some((min, max)) = self.random_number_use_case.parse_range(text) {
                            // Генерируем случайное число через use case
                            match self.random_number_use_case.execute(min, max) {
                                Ok(random_num) => {
                                    // Убираем пользователя из ожидающих
                                    {
                                        let mut waiting = self.waiting_for_range.write().await;
                                        waiting.remove(&user_id);
                                    }

                                    // Формируем ответ
                                    let response = format!(
                                        "🎲 <b>{}</b>\n\n📊 {}: <code>{}</code>\n📏 {}: <code>{}</code> - <code>{}</code>",
                                        locale.get("random_number_result"),
                                        locale.get("random_number_selected"),
                                        random_num,
                                        locale.get("random_number_range"),
                                        min,
                                        max
                                    );

                                    bot.send_message(msg.chat.id, response)
                                        .parse_mode(teloxide::types::ParseMode::Html)
                                        .await?;
                                    return Ok(());
                                }
                                Err(_e) => {
                                    // Если ошибка валидации, отправляем сообщение об ошибке
                                    bot.send_message(
                                        msg.chat.id,
                                        locale.get("random_number_error_range"),
                                    )
                                    .parse_mode(teloxide::types::ParseMode::Html)
                                    .await?;
                                    return Ok(());
                                }
                            }
                        } else {
                            // Неверный формат диапазона
                            bot.send_message(msg.chat.id, locale.get("random_number_error_format"))
                                .parse_mode(teloxide::types::ParseMode::Html)
                                .await?;
                            return Ok(());
                        }
                    }
                }

                // Проверяем, ожидает ли пользователь список для random_choice
                {
                    let waiting = self.waiting_for_choice.read().await;
                    if waiting.get(&user_id).copied().unwrap_or(false) {
                        // Пользователь ожидает список
                        drop(waiting);

                        // Проверяем, содержит ли текст запятые
                        if text.contains(',') {
                            // Используем use case для выбора случайного значения
                            match self.random_choice_use_case.execute(text) {
                                Ok((chosen, items)) => {
                                    // Убираем пользователя из ожидающих
                                    {
                                        let mut waiting = self.waiting_for_choice.write().await;
                                        waiting.remove(&user_id);
                                    }

                                    // Находим индекс выбранного элемента
                                    let chosen_index =
                                        items.iter().position(|item| item == &chosen).unwrap_or(0);

                                    // Формируем список вариантов с выделением выбранного
                                    let options_list = items
                                        .iter()
                                        .enumerate()
                                        .map(|(i, item)| {
                                            if i == chosen_index {
                                                format!("<b>→ {}. {}</b>", i + 1, item)
                                            } else {
                                                format!("   {}. {}", i + 1, item)
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                        .join("\n");

                                    // Формируем ответ
                                    let response = format!(
                                        "🎯 <b>{}</b>\n\n✅ {}: <b>{}</b>\n\n📋 {}:\n{}",
                                        locale.get("random_choice_result"),
                                        locale.get("random_choice_selected"),
                                        chosen.trim(),
                                        locale.get("random_choice_options"),
                                        options_list
                                    );

                                    bot.send_message(msg.chat.id, response)
                                        .parse_mode(teloxide::types::ParseMode::Html)
                                        .await?;
                                    return Ok(());
                                }
                                Err(_e) => {
                                    // Если список пустой или ошибка
                                    bot.send_message(
                                        msg.chat.id,
                                        locale.get("random_choice_error_empty"),
                                    )
                                    .parse_mode(teloxide::types::ParseMode::Html)
                                    .await?;
                                    return Ok(());
                                }
                            }
                        } else {
                            // Пользователь ожидает список, но отправил не список
                            bot.send_message(msg.chat.id, locale.get("random_choice_error_format"))
                                .parse_mode(teloxide::types::ParseMode::Html)
                                .await?;
                            return Ok(());
                        }
                    }
                }
            }

            // Обычная обработка сообщений (старая логика - добавляем случайную букву)
            // В будущем это можно вынести в отдельный use case
            use rand::RngExt;
            let response = {
                let random_char: char = {
                    let mut rng = rand::rng();
                    rng.random_range('A'..='Z')
                };
                format!("{} + {}", text, random_char)
            };
            bot.send_message(msg.chat.id, response).await?;
        }
        Ok(())
    }
}
