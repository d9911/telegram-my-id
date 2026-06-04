// Entities - сущности домена (бизнес-объекты)
// Это как модели в TypeScript/PHP, но без зависимостей от фреймворков

pub mod command;
pub mod user;

// Реэкспортируем для удобства
pub use command::*;
// user пока не экспортируется напрямую, используется через teloxide::types::User
// pub use user::*;

