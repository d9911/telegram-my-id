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
    translations.insert(
        "error_user_info",
        "❌ No se pudo obtener la información del usuario.",
    );
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
    translations.insert("test_desc", "Probar funciones del bot");
    translations.insert("test_title", "Prueba de Funciones del Bot");
    translations.insert("test_status", "Estado del Sistema");
    translations.insert("test_bot_online", "El bot está funcionando");
    translations.insert("test_translations", "Traducciones cargadas");
    translations.insert("test_formatting", "El formato funciona");
    translations.insert("test_current_lang", "Idioma Actual");
    translations.insert("test_selected_lang", "Idioma seleccionado");
    translations.insert("test_translations_examples", "Ejemplos de Traducción");
    translations.insert("test_formatting_demo", "Demostración de Formato");
    translations.insert("test_user_info", "Tu Información");
    translations.insert("test_statistics", "Estadísticas");
    translations.insert("test_users_count", "Usuarios con idioma guardado");
    translations.insert(
        "test_footer",
        "¡Todos los sistemas funcionan normalmente! ✅",
    );
    translations.insert("random_number_desc", "Generar número aleatorio");
    translations.insert("random_number_prompt", "🎲 Ingresa el rango de números en formato:\n\n<pre><code>desde hasta</code></pre>\n\n📝 Ejemplos:\n<pre><code>1 100</code></pre>\n<pre><code>1-100</code></pre>\n<pre><code>1,100</code></pre>");
    translations.insert("random_number_result", "Número Aleatorio");
    translations.insert("random_number_selected", "Resultado");
    translations.insert("random_number_range", "Rango");
    translations.insert(
        "random_number_error_format",
        "❌ Formato inválido.\n\nUsa: <code>desde hasta</code>\n\n📝 Ejemplo: <code>1 100</code>",
    );
    translations.insert(
        "random_number_error_range",
        "❌ El valor mínimo debe ser menor o igual al máximo",
    );
    translations.insert("random_choice_desc", "Elegir valor aleatorio de la lista");
    translations.insert("random_choice_prompt", "🎯 Envía una lista de valores separados por comas.\n\n📝 Ejemplo:\n<pre><code>manzana, plátano, naranja, pera</code></pre>");
    translations.insert("random_choice_result", "Resultado de la Elección");
    translations.insert("random_choice_selected", "Elegido");
    translations.insert("random_choice_options", "Opciones");
    translations.insert(
        "random_choice_error_empty",
        "❌ La lista no puede estar vacía.\n\nEnvía valores separados por comas.",
    );
    translations.insert("random_choice_error_format", "❌ Envía una lista de valores separados por comas.\n\n📝 Ejemplo: <code>manzana, plátano, naranja</code>");

    translations
}
