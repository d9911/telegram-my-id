// Repositories - интерфейсы (traits) для работы с данными
// Это как интерфейсы в TypeScript или абстрактные классы в PHP
// Domain не знает, как именно хранятся данные (файл, БД, память)

pub mod language_repository;

// Реэкспортируем
pub use language_repository::*;
