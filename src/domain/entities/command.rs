// Entity: Command - перечисление всех команд бота
// В Rust enum - это как union types в TypeScript, но более мощный

// Импортируем макрос BotCommands из teloxide для генерации команд
// Это как декоратор в TypeScript/Python
use teloxide::utils::command::BotCommands;

// #[derive] - это как декораторы в TypeScript, автоматически генерирует код
// BotCommands - генерирует методы для работы с командами Telegram
// Clone - позволяет копировать enum (как Object.assign в JS)
#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case", description = "Доступные команды:")]
// rename_rule = "snake_case" означает, что Start становится /start (lowercase)
pub enum Command {
    #[command(description = "Начать работу с ботом")]
    Start,
    #[command(description = "Показать информацию о пользователе")]
    InfoUser,
    #[command(description = "Показать помощь")]
    Help,
    #[command(description = "Выбрать язык интерфейса")]
    Language,
    #[command(description = "Тестирование функций бота")]
    Test,
    #[command(description = "Сгенерировать случайное число")]
    RandomNumber,
    #[command(description = "Выбрать случайное значение из списка")]
    RandomChoice,
}

// В Rust enum может содержать данные (как discriminated unions в TypeScript):
// enum Result<T, E> {
//     Ok(T),    // содержит значение типа T
//     Err(E),   // содержит ошибку типа E
// }
// Но наш Command - простой enum без данных (как const enum в TypeScript)

