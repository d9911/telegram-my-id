// Use Case: RandomNumber - генерация случайного числа в диапазоне

use std::sync::Arc;

// Импортируем сервисы из domain
use crate::domain::services::RandomService;

// Структура use case
pub struct RandomNumberUseCase {
    random_service: Arc<RandomService>,
}

impl RandomNumberUseCase {
    // Конструктор
    pub fn new(random_service: Arc<RandomService>) -> Self {
        Self { random_service }
    }
    
    // Выполняет use case
    // min, max - границы диапазона
    // Result<i64, String> - возвращает число или ошибку (как Promise<number> | Promise<Error>)
    pub fn execute(&self, min: i64, max: i64) -> Result<i64, String> {
        // Валидация входных данных
        if min > max {
            // Err - возвращаем ошибку (как throw new Error() в JS)
            return Err("Минимальное значение должно быть меньше или равно максимальному".to_string());
        }
        
        // Генерируем случайное число
        let random_num = self.random_service.generate_number(min, max);
        
        // Ok - возвращаем успешный результат (как return value в JS)
        Ok(random_num)
    }
    
    // Парсит диапазон из строки
    // &str - строковый срез (как string в TypeScript)
    // Option<(i64, i64)> - возвращает кортеж (min, max) или None (null)
    pub fn parse_range(&self, text: &str) -> Option<(i64, i64)> {
        // Заменяем запятые и дефисы на пробелы для унификации формата
        // replace() - заменяет подстроки (как replace() в JS)
        let range_text = text.replace(',', " ").replace('-', " ");
        
        // split_whitespace() - разбивает строку по пробелам (как split(' ') в JS)
        // collect() - собирает в Vec (как Array.from() в JS)
        let parts: Vec<&str> = range_text.split_whitespace().collect();
        
        // Проверяем, что есть минимум 2 части
        if parts.len() >= 2 {
            // parse() - парсит строку в число (как parseInt() в JS)
            // Ok(value) - успешный парсинг, Err(_) - ошибка
            if let (Ok(min), Ok(max)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                // Если оба числа успешно распарсены, возвращаем кортеж
                return Some((min, max));
            }
        }
        
        // Если не удалось распарсить, возвращаем None (null)
        None
    }
}

