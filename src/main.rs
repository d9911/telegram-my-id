use rand::Rng;
use std::error::Error;
use teloxide::prelude::*;
use teloxide::types::User;
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Доступные команды:")]
enum Command {
    #[command(description = "Показать информацию о пользователе")]
    InfoUser,
    #[command(description = "Показать помощь")]
    Help,
}

type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Запуск бота...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(handle_command),
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

async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> HandlerResult {
    match cmd {
        Command::InfoUser => {
            if let Some(user) = msg.from() {
                let info = format_user_info(user);
                bot.send_message(msg.chat.id, info).await?;
            } else {
                bot.send_message(
                    msg.chat.id,
                    "❌ Не удалось получить информацию о пользователе.",
                )
                .await?;
            }
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
    }
    Ok(())
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

fn format_user_info(user: &User) -> String {
    let mut info = format!(
        "👤 Информация о пользователе:\n\n\
🆔 ID: `{}`\n\
🤖 Бот: {}\n\
👋 Имя: {}\n",
        user.id,
        if user.is_bot { "Да" } else { "Нет" },
        user.first_name,
    );

    if let Some(last_name) = &user.last_name {
        info.push_str(&format!("👤 Фамилия: {}\n", last_name));
    }

    if let Some(username) = &user.username {
        info.push_str(&format!("✉️ Юзернейм: @{}\n", username));
    }

    if let Some(lang_code) = &user.language_code {
        info.push_str(&format!("🌐 Язык: {}\n", lang_code));
    }

    info
}
