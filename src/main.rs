mod locales;

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
}

type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

// Хранилище выбранных языков пользователей (в памяти)
type UserLanguages = Arc<RwLock<HashMap<UserId, String>>>;

#[tokio::main]
async fn main() {
    // Загружаем переменные окружения из .env файла
    dotenv().ok();
    
    pretty_env_logger::init();
    log::info!("Запуск бота...");

    let bot = Bot::from_env();
    let user_languages: UserLanguages = Arc::new(RwLock::new(HashMap::new()));

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
                    move |bot: Bot, msg: Message, cmd: Command| {
                        let user_languages = user_languages.clone();
                        async move {
                            handle_command(bot, msg, cmd, user_languages).await
                        }
                    }
                }),
        )
        .branch(
            Update::filter_message()
                .endpoint(handle_message),
        );

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

async fn handle_message(bot: Bot, msg: Message) -> HandlerResult {
    // Обрабатываем только текстовые сообщения (не команды, так как они обрабатываются выше)
    if let Some(text) = msg.text() {
        // Пропускаем команды, так как они уже обработаны выше
        if !text.starts_with('/') {
            // Генерация случайного символа (ASCII буква) - делаем это до await
            let random_char: char = {
                let mut rng = rand::thread_rng();
                rng.gen_range('A'..='Z')
            };

            let response = format!("{} + {}", text, random_char);
            bot.send_message(msg.chat.id, response).await?;
        }
    }
    Ok(())
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
