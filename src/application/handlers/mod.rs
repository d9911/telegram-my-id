// Handlers - обработчики команд, сообщений и callback'ов
// Это как controllers в TypeScript/PHP
// Координируют работу use cases и отправку ответов через infrastructure

pub mod callback_handler;
pub mod command_handler;
pub mod message_handler;

// Реэкспортируем
pub use callback_handler::*;
pub use command_handler::*;
pub use message_handler::*;
