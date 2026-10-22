// Telegram адаптеры - работа с Telegram Bot API
// Инкапсулирует всю работу с teloxide библиотекой

pub mod bot_client;
pub mod keyboard;

// Реэкспортируем
pub use bot_client::*;
pub use keyboard::*;
