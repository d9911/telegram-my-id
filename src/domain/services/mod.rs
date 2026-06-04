// Services - бизнес-логика приложения
// Это как service classes в TypeScript/PHP
// Содержат логику, которая не относится к конкретной сущности

pub mod user_service;
pub mod random_service;
pub mod locale_service;

// Реэкспортируем
pub use user_service::*;
pub use random_service::*;
pub use locale_service::*;

