// Реализация LanguageRepository для файлового хранилища
// Это как class FileLanguageRepository implements LanguageRepository в TypeScript

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use teloxide::types::UserId;
use tokio::sync::RwLock;

// Импортируем trait из domain (интерфейс)
use crate::domain::repositories::LanguageRepository;

// Константы для путей к файлам
// const - это как const в TypeScript (compile-time константа)
// &str - это строковый литерал (статическая строка)
const DB_DIR: &str = "src/db/data";

// Структура для сериализации UserId как числа
// #[derive] - автоматически генерирует код для сериализации
// serde::Serialize - позволяет сериализовать в JSON
// serde::Deserialize - позволяет десериализовать из JSON
#[derive(serde::Serialize, serde::Deserialize)]
struct LanguageData {
    // #[serde(with = "...")] - кастомная сериализация для HashMap<UserId, String>
    #[serde(with = "serde_user_id_map")]
    languages: HashMap<UserId, String>,
}

// Модуль для сериализации HashMap<UserId, String> как HashMap<u64, String>
// UserId в teloxide - это структура, но в JSON мы храним как число
mod serde_user_id_map {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::HashMap;
    use teloxide::types::UserId;

    // Функция сериализации (преобразование в JSON)
    // S: Serializer - это generic параметр (как <S> в TypeScript)
    pub fn serialize<S>(map: &HashMap<UserId, String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Преобразуем HashMap<UserId, String> в HashMap<u64, String>
        // iter() - итератор по элементам (как Object.entries() в JS)
        // map() - преобразует каждый элемент (как map() в JS)
        // collect() - собирает в коллекцию (как Array.from() в JS)
        let converted: HashMap<u64, String> = map
            .iter()
            .map(|(k, v)| (k.0, v.clone())) // k.0 - доступ к первому полю структуры UserId
            .collect();
        converted.serialize(serializer)
    }

    // Функция десериализации (преобразование из JSON)
    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<UserId, String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Десериализуем как HashMap<u64, String>
        let map: HashMap<u64, String> = HashMap::deserialize(deserializer)?;

        // Преобразуем обратно в HashMap<UserId, String>
        Ok(map
            .into_iter() // into_iter() - владеющий итератор (как for...of в JS)
            .map(|(k, v)| (UserId(k), v)) // UserId(k) - создаем UserId из числа
            .collect())
    }
}

// Реализация LanguageRepository для файлового хранилища
// impl Trait for Struct - это как class Struct implements Trait в TypeScript
pub struct FileLanguageRepository {
    // Arc<RwLock<HashMap>> - потокобезопасное хранилище в памяти
    // Arc - умный указатель с подсчетом ссылок (как shared_ptr в C++)
    // RwLock - читатель-писатель блокировка (много читателей или один писатель)
    // HashMap - хеш-таблица (как Map в TypeScript)
    languages: Arc<RwLock<HashMap<UserId, String>>>,
    path: PathBuf,
    save_lock: tokio::sync::Mutex<()>,
}

impl FileLanguageRepository {
    // Создает новый репозиторий и загружает данные из файла
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Self::at(Path::new(DB_DIR))
    }

    pub fn at(directory: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let path = directory.join("languages.json");
        // Создаем директорию, если её нет
        // create_dir_all - создает все директории в пути (как mkdir -p в bash)
        fs::create_dir_all(directory)?;

        // Загружаем данные из файла
        let languages = if path.exists() {
            // Файл существует, читаем его
            let content = fs::read_to_string(&path)?; // ? - если ошибка, возвращаем её
            // Do not overwrite a malformed existing database with an empty map.
            serde_json::from_str::<LanguageData>(&content)?.languages
        } else {
            log::info!("Файл базы данных не найден, создаем новую базу");
            HashMap::new()
        };

        Ok(Self {
            languages: Arc::new(RwLock::new(languages)),
            path,
            save_lock: tokio::sync::Mutex::new(()),
        })
    }

    // Сохраняет данные в файл
    // &self - ссылка на сам объект (не владеем, только читаем)
    // &mut self - изменяемая ссылка (можем изменять объект)
    async fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // read() - получаем блокировку для чтения (много читателей одновременно)
        // .await - ждем получения блокировки (асинхронная операция)
        let languages = self.languages.read().await;

        // Создаем структуру для сериализации
        let data = LanguageData {
            languages: languages.clone(), // clone() - копируем HashMap
        };

        // to_string_pretty() - сериализует в красивый JSON (с отступами)
        let json = serde_json::to_string_pretty(&data)?;

        // write() - записываем в файл
        let temporary = self.path.with_extension("json.tmp");
        fs::write(&temporary, json)?;
        fs::rename(temporary, &self.path)?;

        log::info!("Сохранено {} языков в базу данных", languages.len());
        Ok(())
    }
}

// Реализуем trait LanguageRepository для FileLanguageRepository
impl LanguageRepository for FileLanguageRepository {
    // Получить язык пользователя
    async fn get_language(&self, user_id: UserId) -> Option<String> {
        // read() - получаем блокировку для чтения
        // .await - ждем получения блокировки
        let languages = self.languages.read().await;

        // get() - получаем значение по ключу (как map.get(key) в JS)
        // cloned() - клонируем Option<String> (если Some, клонируем String)
        languages.get(&user_id).cloned()
    }

    // Сохранить язык пользователя
    async fn save_language(
        &self,
        user_id: UserId,
        lang: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let _save_guard = self.save_lock.lock().await;
        // write() - получаем блокировку для записи (только один писатель)
        {
            let mut languages = self.languages.write().await;
            // insert() - вставляем или обновляем значение (как map.set() в JS)
            languages.insert(user_id, lang);
        } // Блокировка освобождается здесь (когда languages выходит из scope)

        // Сохраняем в файл
        self.save_to_file().await?;
        Ok(())
    }

    // Получить все языки
    async fn get_all_languages(&self) -> HashMap<UserId, String> {
        let languages = self.languages.read().await;
        languages.clone() // Клонируем HashMap для возврата
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn scratch(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("telegram-storage-{}-{name}", std::process::id()))
    }
    #[tokio::test]
    async fn existing_language_format_round_trips_at_the_selected_path() {
        let path = scratch("roundtrip");
        fs::create_dir_all(&path).expect("test directory");
        fs::write(
            path.join("languages.json"),
            r#"{"languages":{"100000001":"ru"}}"#,
        )
        .expect("fixture");
        let repo = FileLanguageRepository::at(&path).expect("load legacy JSON");
        assert_eq!(
            repo.get_language(UserId(100000001)).await.as_deref(),
            Some("ru")
        );
        repo.save_language(UserId(100000002), "ko".into())
            .await
            .expect("save language");
        let reopened = FileLanguageRepository::at(&path).expect("reopen");
        assert_eq!(
            reopened.get_language(UserId(100000002)).await.as_deref(),
            Some("ko")
        );
        assert_eq!(reopened.get_all_languages().await.len(), 2);
        fs::remove_dir_all(path).expect("test cleanup");
    }
    #[test]
    fn malformed_existing_data_is_preserved_and_reported() {
        let path = scratch("invalid");
        fs::create_dir_all(&path).expect("test directory");
        let file = path.join("languages.json");
        fs::write(&file, "broken fixture").expect("fixture");
        assert!(FileLanguageRepository::at(&path).is_err());
        assert_eq!(
            fs::read_to_string(file).expect("unchanged data"),
            "broken fixture"
        );
        fs::remove_dir_all(path).expect("test cleanup");
    }
}
