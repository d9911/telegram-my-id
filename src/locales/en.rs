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
    translations.insert("test_desc", "Test bot functions");
    translations.insert("test_title", "Bot Functions Test");
    translations.insert("test_status", "System Status");
    translations.insert("test_bot_online", "Bot is running");
    translations.insert("test_translations", "Translations loaded");
    translations.insert("test_formatting", "Formatting works");
    translations.insert("test_current_lang", "Current Language");
    translations.insert("test_selected_lang", "Selected language");
    translations.insert("test_translations_examples", "Translation Examples");
    translations.insert("test_formatting_demo", "Formatting Demo");
    translations.insert("test_user_info", "Your Information");
    translations.insert("test_statistics", "Statistics");
    translations.insert("test_users_count", "Users with saved language");
    translations.insert("test_footer", "All systems are working normally! ✅");
    
    translations
}

