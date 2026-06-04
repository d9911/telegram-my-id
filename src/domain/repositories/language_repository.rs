// Trait (интерфейс) для работы с языками пользователей
// В Rust trait - это как interface в TypeScript или protocol в Swift
// Это контракт, который должна выполнить любая реализация

use teloxide::types::UserId;

// async trait - означает, что методы асинхронные (как async функции в JS)
// Send + Sync - означает, что trait можно использовать в многопоточном коде
// 'static - означает, что trait не имеет lifetime параметров (живет все время работы программы)
pub trait LanguageRepository: Send + Sync {
    // Получить язык пользователя по его ID
    // Option<String> - это как string | null в TypeScript
    // Если пользователя нет, вернется None (null)
    async fn get_language(&self, user_id: UserId) -> Option<String>;
    
    // Сохранить язык пользователя
    // Result<(), E> - это как Promise<void> в TypeScript
    // Ok(()) означает успех без данных, Err(E) - ошибка
    async fn save_language(&self, user_id: UserId, lang: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    // Получить все сохраненные языки (для статистики)
    // HashMap - это как Map в TypeScript или dict в Python
    async fn get_all_languages(&self) -> std::collections::HashMap<UserId, String>;
}

// В Rust trait может иметь default implementation (как методы интерфейса в Java 8+):
// impl LanguageRepository for SomeStruct {
//     async fn get_language(&self, user_id: UserId) -> Option<String> {
//         // реализация
//     }
// }

