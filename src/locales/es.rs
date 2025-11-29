use super::Translations;

pub fn get_translations() -> Translations {
    let mut translations = Translations::new();
    
    translations.insert("user_info_title", "👤 Información del usuario:");
    translations.insert("id", "🆔 ID");
    translations.insert("bot", "🤖 Bot");
    translations.insert("first_name", "👋 Nombre");
    translations.insert("last_name", "👤 Apellido");
    translations.insert("username", "✉️ Nombre de usuario");
    translations.insert("language", "🌐 Idioma");
    translations.insert("yes", "Sí");
    translations.insert("no", "No");
    translations.insert("error_user_info", "❌ No se pudo obtener la información del usuario.");
    translations.insert("help_title", "Comandos disponibles:");
    translations.insert("info_user_desc", "Mostrar información del usuario");
    translations.insert("help_desc", "Mostrar ayuda");
    translations.insert("language_set", "✅ Idioma cambiado a español");
    translations.insert("language_set_short", "Idioma cambiado");
    translations.insert("language_confirmed", "Todos los mensajes serán ahora en español.\n\nUsa /info_user para obtener información sobre ti.");
    translations.insert("select_language", "Selecciona el idioma de la interfaz:");
    translations.insert("language_desc", "Seleccionar idioma de la interfaz");
    translations.insert("welcome_message", "👋 ¡Bienvenido!");
    translations.insert("welcome_description", "Soy un bot para obtener información de usuarios de Telegram.\n\n📋 Comandos disponibles:\n• /info_user - obtener información sobre ti\n• /help - lista de todos los comandos\n• /language - cambiar idioma");
    translations.insert("start_desc", "Comenzar a trabajar con el bot");
    
    translations
}

