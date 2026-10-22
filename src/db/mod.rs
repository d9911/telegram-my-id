use std::collections::HashMap;
use std::fs;
use std::path::Path;
use teloxide::types::UserId;

// Константы используются в старых функциях load_languages/save_languages
// Оставляем для совместимости
#[allow(dead_code)]
const DB_DIR: &str = "src/db/data";
#[allow(dead_code)]
const DB_FILE: &str = "src/db/data/languages.json";

// Структура для сериализации UserId как числа
#[derive(serde::Serialize, serde::Deserialize)]
struct LanguageData {
    #[serde(with = "serde_user_id_map")]
    languages: HashMap<UserId, String>,
}

// Модуль для сериализации HashMap<UserId, String> как HashMap<u64, String>
mod serde_user_id_map {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::HashMap;
    use teloxide::types::UserId;

    pub fn serialize<S>(map: &HashMap<UserId, String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let converted: HashMap<u64, String> = map.iter().map(|(k, v)| (k.0, v.clone())).collect();
        converted.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<UserId, String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map: HashMap<u64, String> = HashMap::deserialize(deserializer)?;
        Ok(map.into_iter().map(|(k, v)| (UserId(k), v)).collect())
    }
}

// Старая функция, больше не используется (заменена на FileLanguageRepository)
// Оставляем для совместимости, но помечаем как неиспользуемую
#[allow(dead_code)]
pub fn load_languages() -> HashMap<UserId, String> {
    // Создаем директорию, если её нет
    if let Err(e) = fs::create_dir_all(DB_DIR) {
        log::warn!("Не удалось создать директорию {}: {}", DB_DIR, e);
    }

    // Загружаем данные из файла
    if !Path::new(DB_FILE).exists() {
        log::info!("Файл базы данных не найден, создаем новую базу");
        return HashMap::new();
    }

    match fs::read_to_string(DB_FILE) {
        Ok(content) => match serde_json::from_str::<LanguageData>(&content) {
            Ok(data) => {
                log::info!("Загружено {} языков из базы данных", data.languages.len());
                data.languages
            }
            Err(e) => {
                log::warn!("Ошибка парсинга JSON: {}, создаем новую базу", e);
                HashMap::new()
            }
        },
        Err(e) => {
            log::warn!("Ошибка чтения файла {}: {}, создаем новую базу", DB_FILE, e);
            HashMap::new()
        }
    }
}

// Старая функция, больше не используется (заменена на FileLanguageRepository)
// Оставляем для совместимости, но помечаем как неиспользуемую
#[allow(dead_code)]
pub fn save_languages(
    languages: &HashMap<UserId, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Создаем директорию, если её нет
    fs::create_dir_all(DB_DIR)?;

    let data = LanguageData {
        languages: languages.clone(),
    };

    let json = serde_json::to_string_pretty(&data)?;
    fs::write(DB_FILE, json)?;

    log::info!("Сохранено {} языков в базу данных", languages.len());
    Ok(())
}
