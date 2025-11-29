use super::Translations;

pub fn get_translations() -> Translations {
    let mut translations = Translations::new();
    
    translations.insert("user_info_title", "👤 Информация о пользователе:");
    translations.insert("id", "🆔 ID");
    translations.insert("bot", "🤖 Бот");
    translations.insert("first_name", "👋 Имя");
    translations.insert("last_name", "👤 Фамилия");
    translations.insert("username", "✉️ Юзернейм");
    translations.insert("language", "🌐 Язык");
    translations.insert("yes", "Да");
    translations.insert("no", "Нет");
    translations.insert("error_user_info", "❌ Не удалось получить информацию о пользователе.");
    translations.insert("help_title", "Доступные команды:");
    translations.insert("info_user_desc", "Показать информацию о пользователе");
    translations.insert("help_desc", "Показать помощь");
    translations.insert("language_set", "✅ Язык изменен на русский");
    translations.insert("language_set_short", "Язык изменен");
    translations.insert("language_confirmed", "Теперь все сообщения будут на русском языке.\n\nИспользуйте /info_user для получения информации о себе.");
    translations.insert("select_language", "Выберите язык интерфейса:");
    translations.insert("language_desc", "Выбрать язык интерфейса");
    translations.insert("welcome_message", "👋 Добро пожаловать!");
    translations.insert("welcome_description", "Я бот для получения информации о пользователях Telegram.\n\n📋 Доступные команды:\n• /info_user - получить информацию о себе\n• /help - список всех команд\n• /language - изменить язык");
    translations.insert("start_desc", "Начать работу с ботом");
    
    translations
}

