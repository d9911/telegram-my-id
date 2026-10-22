// Service для работы с информацией о пользователях
// Содержит логику форматирования и обработки данных пользователя

use teloxide::types::User;

// Структура сервиса для работы с пользователями
pub struct UserService {
    // В будущем здесь можно добавить настройки форматирования
}

impl UserService {
    pub fn new() -> Self {
        Self {}
    }

    // Форматирует информацию о пользователе в строку
    // &User - это ссылка на User (не владеем объектом, только читаем)
    // В Rust & означает "borrow" (заимствование), как const reference в C++
    // &str - это строковый срез (как string в TypeScript)
    // String - это owned string (владеем строкой, можем изменять)
    pub fn format_user_info(&self, user: &User, locale_getter: impl Fn(&str) -> &str) -> String {
        // Fn(&str) -> &str - это closure (функция), которая принимает &str и возвращает &str
        // В TypeScript это было бы: (key: string) => string
        // impl Fn - это как function type в TypeScript

        // format! - это макрос для форматирования строк (как template literals в JS)
        // {} - это placeholder для значения (как ${} в template literals)
        let mut info = format!(
            "{}\n\n{}:\n<code>{}</code>\n\n{}: {}\n{}: {}",
            locale_getter("user_info_title"),
            locale_getter("id"),
            user.id,
            locale_getter("bot"),
            if user.is_bot {
                locale_getter("yes")
            } else {
                locale_getter("no")
            },
            locale_getter("first_name"),
            user.first_name,
        );

        // if let - это как if (value !== null) в TypeScript
        // Option<T> - это как T | null в TypeScript
        // Some(value) означает, что значение есть
        // None означает, что значения нет (null)
        if let Some(last_name) = &user.last_name {
            // push_str - добавляет строку к существующей (как += в JS)
            info.push_str(&format!("\n{}: {}", locale_getter("last_name"), last_name));
        }

        if let Some(username) = &user.username {
            info.push_str(&format!("\n{}: @{}", locale_getter("username"), username));
        }

        if let Some(lang_code) = &user.language_code {
            info.push_str(&format!("\n{}: {}", locale_getter("language"), lang_code));
        }

        info
    }
}

impl Default for UserService {
    fn default() -> Self {
        Self::new()
    }
}
