// Persistence адаптеры - реализация репозиториев для работы с данными
// Это как DAO (Data Access Object) в Java или Repository implementations в TypeScript

pub mod language_repository_impl;

// Реэкспортируем
pub use language_repository_impl::*;
