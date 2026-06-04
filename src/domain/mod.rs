// Domain Layer - ядро приложения, бизнес-логика
// Не зависит от внешних библиотек (Telegram, база данных и т.д.)
// Аналогично domain models в DDD (Domain-Driven Design)

// В Rust mod объявляет модуль, pub делает его публичным (как export в JS)
pub mod entities;
pub mod repositories;
pub mod services;

// Реэкспортируем для удобства (используем конкретные импорты в коде)
// pub use entities::*;
// pub use repositories::*;
// pub use services::*;

