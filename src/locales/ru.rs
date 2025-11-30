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
    translations.insert("test_desc", "Тестирование функций бота");
    translations.insert("test_title", "Тест функций бота");
    translations.insert("test_status", "Статус системы");
    translations.insert("test_bot_online", "Бот работает");
    translations.insert("test_translations", "Переводы загружены");
    translations.insert("test_formatting", "Форматирование работает");
    translations.insert("test_current_lang", "Текущий язык");
    translations.insert("test_selected_lang", "Выбранный язык");
    translations.insert("test_translations_examples", "Примеры переводов");
    translations.insert("test_formatting_demo", "Демонстрация форматирования");
    translations.insert("test_user_info", "Информация о вас");
    translations.insert("test_statistics", "Статистика");
    translations.insert("test_users_count", "Пользователей с сохраненным языком");
    translations.insert("test_footer", "Все системы работают нормально! ✅");
    translations.insert("random_number_desc", "Сгенерировать случайное число");
    translations.insert("random_number_prompt", "🎲 Введите диапазон чисел в формате:\n\n<pre><code>от до</code></pre>\n\n📝 Примеры:\n<pre><code>1 100</code></pre>\n<pre><code>1-100</code></pre>\n<pre><code>1,100</code></pre>");
    translations.insert("random_number_result", "Случайное число");
    translations.insert("random_number_selected", "Результат");
    translations.insert("random_number_range", "Диапазон");
    translations.insert("random_number_error_format", "❌ Неверный формат.\n\nИспользуйте: <code>от до</code>\n\n📝 Пример: <code>1 100</code>");
    translations.insert("random_number_error_range", "❌ Минимальное значение должно быть меньше или равно максимальному");
    translations.insert("random_choice_desc", "Выбрать случайное значение из списка");
    translations.insert("random_choice_prompt", "🎯 Отправьте список значений через запятую.\n\n📝 Пример:\n<pre><code>яблоко, банан, апельсин, груша</code></pre>");
    translations.insert("random_choice_result", "Результат выбора");
    translations.insert("random_choice_selected", "Выбрано");
    translations.insert("random_choice_options", "Варианты");
    translations.insert("random_choice_error_empty", "❌ Список не может быть пустым.\n\nОтправьте значения через запятую.");
    translations.insert("random_choice_error_format", "❌ Отправьте список значений через запятую.\n\n📝 Пример: <code>яблоко, банан, апельсин</code>");
    
    translations
}

