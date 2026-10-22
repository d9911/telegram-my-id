// Service для генерации случайных чисел и выбора случайных значений
// Инкапсулирует всю логику работы со случайностью

use rand::RngExt;

// Структура сервиса для работы со случайностью
pub struct RandomService {
    // В будущем здесь можно добавить настройки генератора
    // Например, seed для воспроизводимых результатов
}

impl RandomService {
    // Конструктор
    pub fn new() -> Self {
        Self {}
    }

    // Генерирует случайное число в диапазоне [min, max] (включительно)
    // i64 - это 64-битное целое число со знаком (как number в TypeScript, но точно 64 бита)
    // В Rust есть разные типы чисел: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128
    // i - signed (со знаком), u - unsigned (без знака)
    pub fn generate_number(&self, min: i64, max: i64) -> i64 {
        // rng() - генератор случайных чисел для текущего потока
        // В Rust это thread-local (как ThreadLocal в Java)
        let mut rng = rand::rng();

        // random_range(min..=max) - генерирует число в диапазоне [min, max]
        // ..= означает включительно с обеих сторон (как [min, max] в математике)
        // .. означает [min, max) - не включая max
        rng.random_range(min..=max)
    }

    // Выбирает случайный элемент из списка
    // &[T] - это срез массива (как T[] в TypeScript, но без владения)
    // Option<&str> - возвращает None если список пустой, Some(value) если есть элемент
    // 'a - lifetime параметр (время жизни ссылки)
    // В Rust lifetime указывает, как долго ссылка действительна
    pub fn choose_from_list<'a>(&self, items: &'a [&'a str]) -> Option<&'a str> {
        if items.is_empty() {
            // Если список пустой, возвращаем None (null)
            return None;
        }

        let mut rng = rand::rng();
        // Генерируем случайный индекс от 0 до items.len() (не включая len)
        let index = rng.random_range(0..items.len());

        // items[index] - получаем элемент по индексу
        // В Rust это безопасно - компилятор проверяет границы массива
        Some(items[index])
    }

    // Генерирует случайный символ в диапазоне
    // char - это Unicode символ (4 байта), не просто ASCII
    // Пока не используется, но может пригодиться в будущем
    #[allow(dead_code)]
    pub fn generate_char(&self, min: char, max: char) -> char {
        let mut rng = rand::rng();
        rng.random_range(min..=max)
    }
}

impl Default for RandomService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::RandomService;

    #[test]
    fn singleton_ranges_preserve_their_only_value() {
        let service = RandomService::new();
        for value in [i64::MIN, -1, 0, 1, i64::MAX] {
            assert_eq!(service.generate_number(value, value), value);
        }
    }

    #[test]
    fn generated_numbers_stay_inside_the_requested_range() {
        let service = RandomService::new();
        for _ in 0..128 {
            assert!((-10..=10).contains(&service.generate_number(-10, 10)));
        }
        // Полный диапазон i64 должен поддерживаться без переполнения.
        let _ = service.generate_number(i64::MIN, i64::MAX);
    }

    #[test]
    fn choosing_from_an_empty_list_returns_none() {
        assert_eq!(RandomService::new().choose_from_list(&[]), None);
    }

    #[test]
    fn choosing_from_a_singleton_returns_its_value() {
        assert_eq!(
            RandomService::new().choose_from_list(&["единственный"]),
            Some("единственный")
        );
    }

    #[test]
    fn generated_letters_stay_in_the_requested_alphabet() {
        let service = RandomService::new();
        for _ in 0..128 {
            assert!(service.generate_char('A', 'Z').is_ascii_uppercase());
        }
    }
}
