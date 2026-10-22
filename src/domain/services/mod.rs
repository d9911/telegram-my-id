// Services - бизнес-логика приложения
// Это как service classes в TypeScript/PHP
// Содержат логику, которая не относится к конкретной сущности

pub mod locale_service;
pub mod random_service;
pub mod user_service;

// Реэкспортируем
pub use locale_service::*;
pub use random_service::*;
pub use user_service::*;
