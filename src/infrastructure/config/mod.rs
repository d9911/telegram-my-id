// Модуль конфигурации приложения
// Загружает настройки из переменных окружения

use dotenvy::dotenv;

// Инициализирует конфигурацию приложения
// В Rust функции могут возвращать Result для обработки ошибок
// Это как try-catch в JS, но более явное
pub fn init_config() -> Result<(), Box<dyn std::error::Error>> {
    // dotenv() - загружает переменные из .env файла
    // .ok() - преобразует Result в Option (игнорирует ошибку)
    // В TypeScript это было бы: try { loadEnv(); } catch { /* ignore */ }
    dotenv().ok();

    // Ok(()) - успешный результат без данных (как return в JS)
    Ok(())
}
