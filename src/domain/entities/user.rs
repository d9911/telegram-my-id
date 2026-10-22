// Entity: User - сущность пользователя
// Пока используем User из teloxide, но в будущем можно создать свою обертку

// В Rust можно создать wrapper для User, чтобы изолировать domain от infrastructure:
// pub struct DomainUser {
//     pub id: UserId,
//     pub first_name: String,
//     pub last_name: Option<String>,
//     pub username: Option<String>,
//     pub language_code: Option<String>,
//     pub is_bot: bool,
// }
//
// impl From<teloxide::types::User> for DomainUser {
//     fn from(user: teloxide::types::User) -> Self {
//         DomainUser {
//             id: user.id,
//             first_name: user.first_name,
//             last_name: user.last_name,
//             username: user.username,
//             language_code: user.language_code,
//             is_bot: user.is_bot,
//         }
//     }
// }

// Пока используем напрямую teloxide::types::User
// В будущем можно добавить методы для работы с пользователем здесь
