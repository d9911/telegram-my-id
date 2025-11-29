Запуск (Unix/Linux/macOS)
```
# Установите токен в переменную окружения
export TELOXIDE_TOKEN="YOUR_BOT_TOKEN_HERE"

# Запустите бота
cargo run
```
Запуск (Windows PowerShell)
```
# Установите токен в переменную окружения
$env:TELOXIDE_TOKEN="YOUR_BOT_TOKEN_HERE"

# Запустите бота
cargo run
```

Расширение: Inline-режим (как @userinfobot)

Чтобы бот работал в inline-режиме (показывал информацию при пересылке сообщений), добавьте в main.rs:
```
// Добавьте в enum Command
#[command(description = "Показать информацию о пользователе")]
InfoUser,

// Добавьте ветку обработчика
.branch(Update::filter_inline_query().endpoint(handle_inline))

// Обработчик inline-запросов
async fn handle_inline(bot: Bot, q: InlineQuery) -> HandlerResult {
    let user = &q.from;
    let info = format_user_info(user);

    let results = vec![InlineQueryResultArticle::new(
        "0",
        "Информация о пользователе",
        InputMessageContent::Text(InputMessageContentText::new(info)),
    )];

    bot.answer_inline_query(&q.id, results).await?;
    Ok(())
}
```


👋 Hi! I'm a bot for getting Telegram user information.

🌟 Features:
• Get user information (ID, name, username, etc.)
• Support for 4 languages: Russian, English, Spanish, Korean
• Easy ID copying
• Automatic language detection from profile

📋 Commands:
/start - Start working with the bot
/info_user - Get information about yourself
/language - Select interface language
/help - List all commands

Start with /start command!