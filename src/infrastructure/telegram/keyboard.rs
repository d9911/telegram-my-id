// Модуль для создания клавиатур Telegram
// Инкапсулирует логику создания inline-кнопок

use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

// Создает клавиатуру для выбора языка
// В Rust функции возвращают значения напрямую (не через return, если это последний expression)
// Это как arrow functions в JS: const create = () => value
pub fn create_language_keyboard() -> InlineKeyboardMarkup {
    // Vec - это динамический массив (как Array в TypeScript)
    // Vec<Vec<...>> - это массив массивов (двумерный массив)
    // mut означает, что переменная изменяемая (по умолчанию в Rust все immutable)
    let mut keyboard = Vec::new();
    
    // push - добавляет элемент в конец массива (как push() в JS)
    // vec![] - это макрос для создания массива (как [] в JS)
    keyboard.push(vec![
        // InlineKeyboardButton::callback - создает кнопку с callback
        // Первый параметр - текст кнопки, второй - callback data
        InlineKeyboardButton::callback("🇷🇺 Русский", "lang_ru"),
        InlineKeyboardButton::callback("🇺🇸 English", "lang_en"),
    ]);
    
    keyboard.push(vec![
        InlineKeyboardButton::callback("🇪🇸 Español", "lang_es"),
        InlineKeyboardButton::callback("🇰🇷 한국어", "lang_ko"),
    ]);
    
    // InlineKeyboardMarkup::new - создает клавиатуру из массива кнопок
    // В Rust :: означает статический метод (как . в TypeScript для статических методов)
    InlineKeyboardMarkup::new(keyboard)
}

