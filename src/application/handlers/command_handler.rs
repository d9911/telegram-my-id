// Handler для обработки команд бота
// Координирует работу use cases и отправку ответов

use std::collections::HashMap;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::types::UserId;
use tokio::sync::RwLock;

// Импортируем типы и use cases
use crate::domain::entities::Command;
use crate::shared::HandlerResult;
use crate::application::use_cases::*;
use crate::infrastructure::telegram::create_language_keyboard;
use crate::locales;

// Типы для хранения состояния ожидающих пользователей
// HashMap - это как Map в TypeScript или dict в Python
// UserId - ID пользователя Telegram
// bool - флаг, ожидает ли пользователь ответа
type WaitingForRange = Arc<RwLock<HashMap<UserId, bool>>>;
type WaitingForChoice = Arc<RwLock<HashMap<UserId, bool>>>;

// Структура handler'а
// Содержит все use cases и зависимости
pub struct CommandHandler {
    // Use cases для разных команд
    start_use_case: Arc<StartUseCase>,
    info_user_use_case: Arc<InfoUserUseCase>,
    #[allow(dead_code)] // Используется через методы, но компилятор не всегда это видит
    random_number_use_case: Arc<RandomNumberUseCase>,
    #[allow(dead_code)] // Используется через методы, но компилятор не всегда это видит
    random_choice_use_case: Arc<RandomChoiceUseCase>,
    test_use_case: Arc<TestUseCase>,
    language_use_case: Arc<LanguageUseCase>,
    
    // Состояние для ожидающих пользователей
    waiting_for_range: WaitingForRange,
    waiting_for_choice: WaitingForChoice,
}

impl CommandHandler {
    // Конструктор с dependency injection
    // Принимает все зависимости через параметры
    pub fn new(
        start_use_case: Arc<StartUseCase>,
        info_user_use_case: Arc<InfoUserUseCase>,
        random_number_use_case: Arc<RandomNumberUseCase>,
        random_choice_use_case: Arc<RandomChoiceUseCase>,
        test_use_case: Arc<TestUseCase>,
        language_use_case: Arc<LanguageUseCase>,
    ) -> Self {
        Self {
            start_use_case,
            info_user_use_case,
            random_number_use_case,
            random_choice_use_case,
            test_use_case,
            language_use_case,
            waiting_for_range: Arc::new(RwLock::new(HashMap::new())),
            waiting_for_choice: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    // Обрабатывает команду
    // bot - клиент Telegram Bot API
    // msg - сообщение от пользователя
    // cmd - команда для обработки
    pub async fn handle(&self, bot: Bot, msg: Message, cmd: Command) -> HandlerResult {
        // Получаем язык пользователя через use case
        let user_id = msg.from().map(|u| u.id);
        let user_lang_code = msg.from().and_then(|u| u.language_code.as_deref());
        let lang = if let Some(id) = user_id {
            self.language_use_case.get_user_language(id, user_lang_code).await
        } else {
            "en".to_string()
        };
        
        // Обрабатываем команду в зависимости от типа
        // match - это как switch в JS, но более мощный
        match cmd {
            Command::Start => {
                // Используем use case для определения языка
                let (user_lang, _) = self.start_use_case.execute(msg.from());
                
                // Получаем локализацию
                let welcome_locale = locales::get_locale(&user_lang);
                let keyboard = create_language_keyboard();
                
                // Формируем приветственное сообщение
                let welcome_text = format!(
                    "{}\n\n{}\n\n{}",
                    welcome_locale.get("welcome_message"),
                    welcome_locale.get("welcome_description"),
                    welcome_locale.get("select_language")
                );
                
                // Отправляем сообщение
                bot.send_message(msg.chat.id, welcome_text)
                    .reply_markup(keyboard)
                    .await?;
            }
            Command::InfoUser => {
                if let Some(user) = msg.from() {
                    // Используем use case для форматирования информации
                    let locale = locales::get_locale(&lang);
                    let info = self.info_user_use_case.execute(
                        user,
                        &lang,
                        |key| locale.get(key), // closure для получения переводов
                    );
                    
                    bot.send_message(msg.chat.id, info)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                } else {
                    let locale = locales::get_locale(&lang);
                    bot.send_message(msg.chat.id, locale.get("error_user_info"))
                        .await?;
                }
            }
            Command::Help => {
                let locale = locales::get_locale(&lang);
                let help_text = format!("{}\n\n{}", locale.get("help_title"), Command::descriptions());
                bot.send_message(msg.chat.id, help_text).await?;
            }
            Command::Language => {
                let locale = locales::get_locale(&lang);
                let keyboard = create_language_keyboard();
                bot.send_message(msg.chat.id, locale.get("select_language"))
                    .reply_markup(keyboard)
                    .await?;
            }
            Command::Test => {
                // Используем use case для тестирования
                let locale = locales::get_locale(&lang);
                let test_info = self.test_use_case.execute(
                    msg.from(),
                    &lang,
                    |key| locale.get(key),
                ).await;
                
                bot.send_message(msg.chat.id, test_info)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await?;
            }
            Command::RandomNumber => {
                let locale = locales::get_locale(&lang);
                if let Some(user) = msg.from() {
                    // Помечаем пользователя как ожидающего диапазон
                    {
                        let mut waiting = self.waiting_for_range.write().await;
                        waiting.insert(user.id, true);
                    }
                    bot.send_message(msg.chat.id, locale.get("random_number_prompt"))
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
            }
            Command::RandomChoice => {
                let locale = locales::get_locale(&lang);
                if let Some(user) = msg.from() {
                    // Помечаем пользователя как ожидающего список
                    {
                        let mut waiting = self.waiting_for_choice.write().await;
                        waiting.insert(user.id, true);
                    }
                }
                bot.send_message(msg.chat.id, locale.get("random_choice_prompt"))
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await?;
            }
        }
        Ok(())
    }
    
    // Получает ссылки на состояния ожидающих пользователей
    // Нужно для message_handler
    pub fn get_waiting_states(&self) -> (WaitingForRange, WaitingForChoice) {
        (self.waiting_for_range.clone(), self.waiting_for_choice.clone())
    }
}

