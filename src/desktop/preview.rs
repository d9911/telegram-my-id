//! Preview runs the existing formatting services using explicitly synthetic data.
use crate::{domain::services::UserService, locales};
use teloxide::types::User;
pub const COMMANDS: [(&str, &str); 7] = [
    ("/start", "Приветствие и выбор языка"),
    ("/info_user", "ID и информация о пользователе"),
    ("/language", "Смена языка интерфейса"),
    ("/help", "Список команд"),
    ("/random_number", "Случайное число в диапазоне"),
    ("/random_choice", "Выбор значения из списка"),
    ("/test", "Диагностика функций бота"),
];
pub const LANGUAGES: [(&str, &str); 4] = [
    ("ru", "Русский"),
    ("en", "English"),
    ("es", "Español"),
    ("ko", "한국어"),
];
pub fn text(command: usize, lang: &str) -> String {
    let locale = locales::get_locale(lang);
    let result = match command {
        0 => format!(
            "{}\n\n{}\n\n{}",
            locale.get("welcome_message"),
            locale.get("welcome_description"),
            locale.get("select_language")
        ),
        1 => {
            let user:User=serde_json::from_value(serde_json::json!({"id":100000001,"is_bot":false,"first_name":"Alex","last_name":"Example","username":"alex_demo","language_code":lang})).expect("static preview fixture must be valid");
            UserService::new().format_user_info(&user, |key| locale.get(key))
        }
        2 => locale.get("select_language").into(),
        3 => format!(
            "{}\n\n{}",
            locale.get("help_title"),
            COMMANDS
                .iter()
                .map(|(cmd, _)| *cmd)
                .collect::<Vec<_>>()
                .join("\n")
        ),
        4 => locale.get("random_number_prompt").into(),
        5 => locale.get("random_choice_prompt").into(),
        _ => format!(
            "{}\n\n{}\n{}\n{}",
            locale.get("test_title"),
            locale.get("test_status"),
            locale.get("test_translations"),
            locale.get("test_formatting")
        ),
    };
    result
        .replace("<pre>", "")
        .replace("</pre>", "")
        .replace("<code>", "")
        .replace("</code>", "")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_existing_command_has_a_preview_in_all_four_languages() {
        for (lang, _) in LANGUAGES {
            for command in 0..COMMANDS.len() {
                assert!(!text(command, lang).is_empty());
            }
        }
    }
    #[test]
    fn user_preview_contains_only_the_fixed_demo_identity() {
        for (lang, _) in LANGUAGES {
            let result = text(1, lang);
            assert!(result.contains("100000001"));
            assert!(result.contains("alex_demo"));
            assert!(!result.contains("<code>"));
        }
    }
}
