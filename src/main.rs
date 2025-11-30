mod locales;
mod db;

use rand::Rng;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, User, UserId};
use teloxide::utils::command::BotCommands;
use dotenvy::dotenv;
use tokio::sync::RwLock;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case", description = "Доступные команды:")]
enum Command {
    #[command(description = "Начать работу с ботом")]
    Start,
    #[command(description = "Показать информацию о пользователе")]
    InfoUser,
    #[command(description = "Показать помощь")]
    Help,
    #[command(description = "Выбрать язык интерфейса")]
    Language,
    #[command(description = "Тестирование функций бота")]
    Test,
    #[command(description = "Сгенерировать случайное число")]
    RandomNumber,
    #[command(description = "Выбрать случайное значение из списка")]
    RandomChoice,
}

type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

// Хранилище выбранных языков пользователей (в памяти)
type UserLanguages = Arc<RwLock<HashMap<UserId, String>>>;

// Хранилище ожидающих ответа пользователей
type WaitingForRange = Arc<RwLock<HashMap<UserId, bool>>>; // для команды random_number
type WaitingForChoice = Arc<RwLock<HashMap<UserId, bool>>>; // для команды random_choice

#[tokio::main]
async fn main() {
    // Загружаем переменные окружения из .env файла
    dotenv().ok();
    
    pretty_env_logger::init();
    log::info!("Запуск бота...");

    let bot = Bot::from_env();
    
    // Проверяем подключение к Telegram API
    log::info!("Проверка подключения к Telegram API...");
    match bot.get_me().await {
        Ok(me) => {
            let username = format!("@{}", me.username());
            log::info!("✅ Бот успешно подключен: {} (ID: {})", username, me.id);
        }
        Err(e) => {
            log::error!("❌ Ошибка подключения к Telegram API: {}", e);
            log::error!("Возможные причины:");
            log::error!("  1. Неправильный токен бота (проверьте .env файл)");
            log::error!("  2. Проблемы с интернет-соединением");
            log::error!("  3. Telegram API заблокирован в вашем регионе");
            log::error!("  4. Firewall блокирует подключение");
            log::error!("Попробуйте:");
            log::error!("  - Проверить токен в .env файле");
            log::error!("  - Проверить интернет-соединение");
            log::error!("  - Использовать VPN, если Telegram заблокирован");
            std::process::exit(1);
        }
    }
    
    // Загружаем сохраненные языки из файла
    let loaded_languages = db::load_languages();
    let user_languages: UserLanguages = Arc::new(RwLock::new(loaded_languages));
    let waiting_for_range: WaitingForRange = Arc::new(RwLock::new(HashMap::new()));
    let waiting_for_choice: WaitingForChoice = Arc::new(RwLock::new(HashMap::new()));

    let handler = dptree::entry()
        .branch(
            Update::filter_callback_query()
                .endpoint({
                    let user_languages = user_languages.clone();
                    move |bot: Bot, q: CallbackQuery| {
                        let user_languages = user_languages.clone();
                        async move {
                            handle_language_callback(bot, q, user_languages).await
                        }
                    }
                }),
        )
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint({
                    let user_languages = user_languages.clone();
                    let waiting_for_range = waiting_for_range.clone();
                    let waiting_for_choice = waiting_for_choice.clone();
                    move |bot: Bot, msg: Message, cmd: Command| {
                        let user_languages = user_languages.clone();
                        let waiting_for_range = waiting_for_range.clone();
                        let waiting_for_choice = waiting_for_choice.clone();
                        async move {
                            handle_command(bot, msg, cmd, user_languages, waiting_for_range, waiting_for_choice).await
                        }
                    }
                }),
        )
        .branch(
            Update::filter_message()
                .endpoint({
                    let waiting_for_range = waiting_for_range.clone();
                    let waiting_for_choice = waiting_for_choice.clone();
                    let user_languages = user_languages.clone();
                    move |bot: Bot, msg: Message| {
                        let waiting_for_range = waiting_for_range.clone();
                        let waiting_for_choice = waiting_for_choice.clone();
                        let user_languages = user_languages.clone();
                        async move {
                            handle_message(bot, msg, waiting_for_range, waiting_for_choice, user_languages).await
                        }
                    }
                }),
        );

    log::info!("Бот готов к работе. Ожидание сообщений...");
    log::warn!("⚠️  Если вы видите ошибки 'TimedOut' - это НЕ критично!");
    log::warn!("Бот автоматически повторяет попытки подключения.");
    log::warn!("Возможные причины таймаутов:");
    log::warn!("  • Медленное интернет-соединение");
    log::warn!("  • Telegram API заблокирован (используйте VPN)");
    log::warn!("  • Временные проблемы на стороне Telegram");
    log::warn!("Бот продолжит работать, просто попытки подключения могут занимать больше времени.");
    
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn handle_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    user_languages: UserLanguages,
    waiting_for_range: WaitingForRange,
    waiting_for_choice: WaitingForChoice,
) -> HandlerResult {
    let lang = get_user_language(&user_languages, msg.from()).await;

    match cmd {
        Command::Start => {
            // Определяем язык пользователя из его профиля для первого приветствия
            let user_lang = if let Some(user) = msg.from() {
                if let Some(lang_code) = &user.language_code {
                    locales::detect_language(Some(lang_code))
                } else {
                    "en"
                }
            } else {
                "en"
            };
            
            // Используем язык пользователя для приветствия, но сохраняем выбранный язык для будущих команд
            let welcome_locale = locales::get_locale(user_lang);
            let keyboard = create_language_keyboard();
            
            let welcome_text = format!(
                "{}\n\n{}\n\n{}",
                welcome_locale.get("welcome_message"),
                welcome_locale.get("welcome_description"),
                welcome_locale.get("select_language")
            );
            
            bot.send_message(msg.chat.id, welcome_text)
                .reply_markup(keyboard)
                .await?;
        }
        Command::InfoUser => {
            if let Some(user) = msg.from() {
                let info = format_user_info(user, &lang);
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
            let test_info = format_test_info(msg.from(), &lang, &user_languages).await;
            bot.send_message(msg.chat.id, test_info)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await?;
        }
        Command::RandomNumber => {
            let locale = locales::get_locale(&lang);
            if let Some(user) = msg.from() {
                // Помечаем пользователя как ожидающего ответа
                {
                    let mut waiting = waiting_for_range.write().await;
                    waiting.insert(user.id, true);
                }
                bot.send_message(msg.chat.id, locale.get("random_number_prompt")).await?;
            }
        }
        Command::RandomChoice => {
            let locale = locales::get_locale(&lang);
            if let Some(user) = msg.from() {
                // Помечаем пользователя как ожидающего список
                {
                    let mut waiting = waiting_for_choice.write().await;
                    waiting.insert(user.id, true);
                }
            }
            bot.send_message(msg.chat.id, locale.get("random_choice_prompt")).await?;
        }
    }
    Ok(())
}

async fn handle_language_callback(
    bot: Bot,
    q: CallbackQuery,
    user_languages: UserLanguages,
) -> HandlerResult {
    if let Some(data) = q.data {
        // Извлекаем язык из callback data (формат: "lang_ru", "lang_en", и т.д.)
        let lang = if data.starts_with("lang_") {
            &data[5..] // Пропускаем "lang_"
        } else {
            data.as_str()
        };

        // Проверяем что язык валидный
        if !["ru", "en", "es", "ko"].contains(&lang) {
            return Ok(());
        }

        let user_id = q.from.id;

        // Сохраняем выбранный язык
        {
            let mut langs = user_languages.write().await;
            langs.insert(user_id, lang.to_string());
            
            // Сохраняем в файл
            if let Err(e) = db::save_languages(&langs) {
                log::error!("Ошибка сохранения языков в файл: {}", e);
            }
        }

        let locale = locales::get_locale(lang);
        let response = format!(
            "{}\n\n{}",
            locale.get("language_set"),
            locale.get("language_confirmed")
        );

        bot.answer_callback_query(q.id)
            .text(locale.get("language_set_short"))
            .await?;

        if let Some(msg) = q.message {
            bot.edit_message_text(msg.chat.id, msg.id, response)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await?;
        }
    }
    Ok(())
}

fn create_language_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Vec::new();
    keyboard.push(vec![
        InlineKeyboardButton::callback("🇷🇺 Русский", "lang_ru"),
        InlineKeyboardButton::callback("🇺🇸 English", "lang_en"),
    ]);
    keyboard.push(vec![
        InlineKeyboardButton::callback("🇪🇸 Español", "lang_es"),
        InlineKeyboardButton::callback("🇰🇷 한국어", "lang_ko"),
    ]);
    InlineKeyboardMarkup::new(keyboard)
}

async fn get_user_language(
    user_languages: &UserLanguages,
    user: Option<&User>,
) -> String {
    if let Some(user) = user {
        let user_id = user.id;
        
        // Проверяем сохраненный язык
        {
            let langs = user_languages.read().await;
            if let Some(lang) = langs.get(&user_id) {
                return lang.clone();
            }
        }
        
        // Используем язык из профиля пользователя
        if let Some(lang_code) = &user.language_code {
            return locales::detect_language(Some(lang_code)).to_string();
        }
    }
    
    "en".to_string() // По умолчанию английский
}

async fn handle_message(
    bot: Bot,
    msg: Message,
    waiting_for_range: WaitingForRange,
    waiting_for_choice: WaitingForChoice,
    user_languages: UserLanguages,
) -> HandlerResult {
    if let Some(text) = msg.text() {
        // Пропускаем команды
        if text.starts_with('/') {
            return Ok(());
        }

        if let Some(user) = msg.from() {
            let user_id = user.id;
            let lang = get_user_language(&user_languages, Some(user)).await;
            let locale = locales::get_locale(&lang);

            // Проверяем, ожидает ли пользователь ввода диапазона для random_number
            {
                let waiting = waiting_for_range.read().await;
                if waiting.get(&user_id).copied().unwrap_or(false) {
                    // Пользователь ожидает ввода диапазона
                    drop(waiting);
                    
                    // Парсим диапазон (формат: "1-100" или "1 100" или "1,100")
                    let range_text = text.replace(',', " ").replace('-', " ");
                    let parts: Vec<&str> = range_text.split_whitespace().collect();
                    
                    if parts.len() >= 2 {
                        if let (Ok(min), Ok(max)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                            if min <= max {
                                // Генерируем случайное число до await
                                let random_num = {
                                    let mut rng = rand::thread_rng();
                                    rng.gen_range(min..=max)
                                };
                                
                                // Убираем пользователя из ожидающих
                                {
                                    let mut waiting = waiting_for_range.write().await;
                                    waiting.remove(&user_id);
                                }
                                
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
                            } else {
                                bot.send_message(msg.chat.id, locale.get("random_number_error_range")).await?;
                                return Ok(());
                            }
                        }
                    }
                    
                    bot.send_message(msg.chat.id, locale.get("random_number_error_format")).await?;
                    return Ok(());
                }
            }

            // Проверяем, ожидает ли пользователь список для random_choice
            {
                let waiting = waiting_for_choice.read().await;
                if waiting.get(&user_id).copied().unwrap_or(false) {
                    // Пользователь ожидает список для random_choice
                    drop(waiting);
                    
                    // Проверяем, содержит ли текст запятые
                    if text.contains(',') {
                        let items: Vec<&str> = text.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
                        
                        if items.is_empty() {
                            bot.send_message(msg.chat.id, locale.get("random_choice_error_empty")).await?;
                            return Ok(());
                        }
                        
                        // Генерируем случайный индекс до await
                        let chosen_index = {
                            let mut rng = rand::thread_rng();
                            rng.gen_range(0..items.len())
                        };
                        let chosen = items[chosen_index];
                        
                        // Убираем пользователя из ожидающих
                        {
                            let mut waiting = waiting_for_choice.write().await;
                            waiting.remove(&user_id);
                        }
                        
                        let options_list = items.iter().enumerate()
                            .map(|(i, item)| {
                                if i == chosen_index {
                                    format!("<b>→ {}. {}</b>", i + 1, item)
                                } else {
                                    format!("   {}. {}", i + 1, item)
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("\n");
                        
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
                    } else {
                        // Пользователь ожидает список, но отправил не список
                        bot.send_message(msg.chat.id, locale.get("random_choice_error_format")).await?;
                        return Ok(());
                    }
                }
            }
        }

        // Обычная обработка сообщений (старая логика)
        // Генерируем случайный символ до await
        let response = {
            let random_char: char = {
                let mut rng = rand::thread_rng();
                rng.gen_range('A'..='Z')
            };
            format!("{} + {}", text, random_char)
        };
        bot.send_message(msg.chat.id, response).await?;
    }
    Ok(())
}

async fn format_test_info(
    user: Option<&User>,
    current_lang: &str,
    user_languages: &UserLanguages,
) -> String {
    let locale = locales::get_locale(current_lang);
    
    let mut test_info = format!(
        "🧪 <b>{}</b>\n\n",
        locale.get("test_title")
    );
    
    // Статус бота
    test_info.push_str(&format!("✅ <b>{}</b>\n", locale.get("test_status")));
    test_info.push_str(&format!("  • {}: ✅\n", locale.get("test_bot_online")));
    test_info.push_str(&format!("  • {}: ✅\n", locale.get("test_translations")));
    test_info.push_str(&format!("  • {}: ✅\n\n", locale.get("test_formatting")));
    
    // Текущий язык
    test_info.push_str(&format!("🌐 <b>{}</b>\n", locale.get("test_current_lang")));
    test_info.push_str(&format!("  {}: <code>{}</code>\n\n", locale.get("test_selected_lang"), current_lang));
    
    // Примеры переводов на всех языках
    test_info.push_str(&format!("📝 <b>{}</b>\n", locale.get("test_translations_examples")));
    for lang_code in ["ru", "en", "es", "ko"] {
        let test_locale = locales::get_locale(lang_code);
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
            test_locale.get("welcome_message")
        ));
    }
    test_info.push_str("\n");
    
    // Тест форматирования ID
    test_info.push_str(&format!("📋 <b>{}</b>\n", locale.get("test_formatting_demo")));
    test_info.push_str(&format!("  {}:\n", locale.get("id")));
    test_info.push_str(&format!("  <code>123456789</code>\n\n"));
    
    // Информация о пользователе (если доступна)
    if let Some(user) = user {
        test_info.push_str(&format!("👤 <b>{}</b>\n", locale.get("test_user_info")));
        test_info.push_str(&format!("  {}: <code>{}</code>\n", locale.get("id"), user.id));
        test_info.push_str(&format!("  {}: {}\n", locale.get("first_name"), user.first_name));
        if let Some(username) = &user.username {
            test_info.push_str(&format!("  {}: @{}\n", locale.get("username"), username));
        }
        test_info.push_str("\n");
    }
    
    // Статистика языков
    {
        let langs = user_languages.read().await;
        test_info.push_str(&format!("📊 <b>{}</b>\n", locale.get("test_statistics")));
        test_info.push_str(&format!("  {}: {}\n", locale.get("test_users_count"), langs.len()));
    }
    
    test_info.push_str(&format!("\n💡 <i>{}</i>", locale.get("test_footer")));
    
    test_info
}

fn format_user_info(user: &User, lang: &str) -> String {
    let locale = locales::get_locale(lang);
    
    // Делаем ID легко копируемым - используем моноширинный шрифт на отдельной строке
    // В Telegram можно легко скопировать текст в тегах <code>
    let mut info = format!(
        "{}\n\n{}:\n<code>{}</code>\n\n{}: {}\n{}: {}",
        locale.get("user_info_title"),
        locale.get("id"),
        user.id,
        locale.get("bot"),
        if user.is_bot {
            locale.get("yes")
        } else {
            locale.get("no")
        },
        locale.get("first_name"),
        user.first_name,
    );

    if let Some(last_name) = &user.last_name {
        info.push_str(&format!("\n{}: {}", locale.get("last_name"), last_name));
    }

    if let Some(username) = &user.username {
        info.push_str(&format!("\n{}: @{}", locale.get("username"), username));
    }

    if let Some(lang_code) = &user.language_code {
        info.push_str(&format!("\n{}: {}", locale.get("language"), lang_code));
    }

    info
}
