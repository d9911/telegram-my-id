// Общие типы, используемые во всем приложении
// Аналогично type definitions в TypeScript

use std::error::Error;

// Тип результата для обработчиков команд
// В Rust Result<T, E> - это как Promise в JS, но для синхронного кода
// Box<dyn Error> - это как Error в TypeScript, но в Rust это trait object
// Send + Sync - означает, что ошибка может быть передана между потоками (как в async/await)
pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

// В Rust type alias - это как type в TypeScript:
// type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

