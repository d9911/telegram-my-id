use super::Translations;

pub fn get_translations() -> Translations {
    let mut translations = Translations::new();
    
    translations.insert("user_info_title", "👤 User Information:");
    translations.insert("id", "🆔 ID");
    translations.insert("bot", "🤖 Bot");
    translations.insert("first_name", "👋 First Name");
    translations.insert("last_name", "👤 Last Name");
    translations.insert("username", "✉️ Username");
    translations.insert("language", "🌐 Language");
    translations.insert("yes", "Yes");
    translations.insert("no", "No");
    translations.insert("error_user_info", "❌ Failed to get user information.");
    translations.insert("help_title", "Available commands:");
    translations.insert("info_user_desc", "Show user information");
    translations.insert("help_desc", "Show help");
    translations.insert("language_set", "✅ Language changed to English");
    translations.insert("language_set_short", "Language changed");
    translations.insert("language_confirmed", "All messages will now be in English.\n\nUse /info_user to get information about yourself.");
    translations.insert("select_language", "Select interface language:");
    translations.insert("language_desc", "Select interface language");
    translations.insert("welcome_message", "👋 Welcome!");
    translations.insert("welcome_description", "I'm a bot for getting Telegram user information.\n\n📋 Available commands:\n• /info_user - get information about yourself\n• /help - list all commands\n• /language - change language");
    translations.insert("start_desc", "Start working with the bot");
    
    translations
}

