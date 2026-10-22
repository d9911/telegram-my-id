// Обертка над teloxide::Bot для изоляции domain от infrastructure
// В будущем можно добавить методы-хелперы для отправки сообщений

use teloxide::{Bot, requests::Requester};

// Структура-обертка для Bot
// В Rust можно создать wrapper для добавления дополнительной функциональности
pub struct BotClient {
    // bot - приватное поле (не pub)
    // В Rust по умолчанию все приватное, нужно явно указывать pub
    bot: Bot,
}

impl BotClient {
    // Создает новый BotClient из Bot
    // В Rust принято использовать new() для конструкторов
    pub fn new(bot: Bot) -> Self {
        Self { bot }
    }

    // Получает внутренний Bot (для прямого доступа, если нужно)
    // &self - это как this в TypeScript (ссылка на сам объект)
    // &mut self - это как this, но с возможностью изменять объект
    pub fn inner(&self) -> &Bot {
        &self.bot
    }

    // Проверяет подключение к Telegram API
    // async fn - асинхронная функция (как async function в JS)
    // Result<T, E> - это как Promise<T> в JS, но с явной обработкой ошибок
    pub async fn check_connection(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // get_me() - получает информацию о боте (асинхронный метод)
        // .await - ждет завершения асинхронной операции (как await в JS)
        // ? - оператор для обработки ошибок (если ошибка, возвращает её)
        // Это как try-catch, но более компактно
        self.bot.get_me().await?;
        Ok(())
    }
}

// В Rust можно реализовать From trait для автоматического преобразования
// impl From<Bot> for BotClient означает, что можно написать: BotClient::from(bot)
impl From<Bot> for BotClient {
    fn from(bot: Bot) -> Self {
        Self::new(bot)
    }
}
