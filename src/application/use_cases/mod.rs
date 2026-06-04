// Use Cases - конкретные сценарии использования приложения
// Это как service methods в TypeScript/PHP
// Каждый use case решает одну бизнес-задачу

pub mod start_use_case;
pub mod info_user_use_case;
pub mod random_number_use_case;
pub mod random_choice_use_case;
pub mod test_use_case;
pub mod language_use_case;

// Реэкспортируем
pub use start_use_case::*;
pub use info_user_use_case::*;
pub use random_number_use_case::*;
pub use random_choice_use_case::*;
pub use test_use_case::*;
pub use language_use_case::*;

