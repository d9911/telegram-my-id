// Infrastructure Layer - адаптеры для внешних систем
// Это как adapters в TypeScript/PHP - связывают наше приложение с внешним миром
// Telegram API, база данных, файловая система и т.д.

pub mod config;
pub mod persistence;
pub mod telegram;

// Реэкспортируем (используем конкретные импорты в коде)
// pub use telegram::*;
// pub use persistence::*;
// pub use config::*;
