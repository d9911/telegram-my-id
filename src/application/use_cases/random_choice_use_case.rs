// Use Case: RandomChoice - выбор случайного значения из списка

use std::sync::Arc;

// Импортируем сервисы из domain
use crate::domain::services::RandomService;

// Структура use case
pub struct RandomChoiceUseCase {
    random_service: Arc<RandomService>,
}

impl RandomChoiceUseCase {
    // Конструктор
    pub fn new(random_service: Arc<RandomService>) -> Self {
        Self { random_service }
    }
    
    // Выполняет use case
    // text - текст со списком значений через запятую
    // -> Result<(String, Vec<String>), String> - возвращает выбранное значение и весь список, или ошибку
    pub fn execute(&self, text: &str) -> Result<(String, Vec<String>), String> {
        // Проверяем, что текст содержит запятые
        if !text.contains(',') {
            return Err("Список должен содержать значения через запятую".to_string());
        }
        
        // Разбиваем строку по запятым и очищаем от пробелов
        // split(',') - разбивает по запятой (как split(',') в JS)
        // map(|s| s.trim()) - убирает пробелы с начала и конца (как trim() в JS)
        // filter(|s| !s.is_empty()) - убирает пустые строки (как filter() в JS)
        // collect() - собирает в Vec<String>
        let items: Vec<String> = text
            .split(',')
            .map(|s| s.trim().to_string()) // to_string() - преобразует &str в String (owned)
            .filter(|s| !s.is_empty())
            .collect();
        
        // Проверяем, что список не пустой
        if items.is_empty() {
            return Err("Список не может быть пустым".to_string());
        }
        
        // Преобразуем Vec<String> в Vec<&str> для передачи в сервис
        // as_slice() - получаем срез массива
        // iter() - итератор по элементам
        // map(|s| s.as_str()) - преобразуем String в &str
        // collect() - собираем в Vec<&str>
        let items_refs: Vec<&str> = items.iter().map(|s| s.as_str()).collect();
        
        // Выбираем случайный элемент
        // choose_from_list возвращает Option<&str>
        if let Some(chosen) = self.random_service.choose_from_list(&items_refs) {
            // Если элемент выбран, возвращаем его и весь список
            Ok((chosen.to_string(), items))
        } else {
            // Это не должно произойти, так как мы проверили, что список не пустой
            Err("Не удалось выбрать элемент".to_string())
        }
    }
}

