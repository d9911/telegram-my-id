// Application Layer - use cases и handlers
// Это как controllers в TypeScript/PHP, но с разделением на use cases
// Координирует работу между domain и infrastructure

pub mod handlers;
pub mod use_cases;

// Реэкспортируем (используем конкретные импорты в коде)
// pub use handlers::*;
// pub use use_cases::*;

