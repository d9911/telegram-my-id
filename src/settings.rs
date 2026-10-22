//! Paths are persisted; bot credentials are deliberately never serialized.
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn application_dir() -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    if cfg!(target_os = "macos") {
        home.join("Library/Application Support/TelegramMyID")
    } else if cfg!(target_os = "windows") {
        std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .unwrap_or(home)
            .join("TelegramMyID")
    } else {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".local/share"))
            .join("telegram-my-id")
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub env_file: Option<PathBuf>,
    pub data_dir: PathBuf,
}
impl Default for Settings {
    fn default() -> Self {
        let local = std::env::current_dir()
            .ok()
            .map(|p| p.join(".env"))
            .filter(|p| p.is_file());
        Self {
            env_file: local,
            data_dir: application_dir().join("data"),
        }
    }
}
impl Settings {
    pub fn load() -> Result<Self, String> {
        let path = application_dir().join("settings.json");
        if !path.exists() {
            return Ok(Self::default());
        }
        let bytes =
            fs::read(path).map_err(|_| "Не удалось прочитать настройки приложения".to_owned())?;
        serde_json::from_slice(&bytes).map_err(|_| {
            "Файл настроек повреждён. Выберите пути заново и сохраните настройки.".into()
        })
    }
    pub fn save(&self) -> Result<(), String> {
        let dir = application_dir();
        fs::create_dir_all(&dir).map_err(|e| format!("Не удалось создать папку настроек: {e}"))?;
        let bytes = serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?;
        let temporary = dir.join(format!("settings-{}.tmp", std::process::id()));
        fs::write(&temporary, bytes).map_err(|e| e.to_string())?;
        fs::rename(temporary, dir.join("settings.json")).map_err(|e| e.to_string())
    }
}

pub fn validate_token(token: &str) -> Result<(), String> {
    let Some((id, secret)) = token.split_once(':') else {
        return Err(
            "Токен должен иметь формат 123456789:секретная_часть. Возьмите его у @BotFather."
                .into(),
        );
    };
    if id.is_empty()
        || id.len() > 20
        || !id.bytes().all(|c| c.is_ascii_digit())
        || !(20..=256).contains(&secret.len())
        || !secret
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-')
    {
        return Err(
            "Проверьте формат токена от @BotFather. Пробелы и переносы строк недопустимы.".into(),
        );
    }
    Ok(())
}

pub struct Credentials {
    pub token: String,
    pub proxy: Option<String>,
}
pub fn read_credentials(path: &Path) -> Result<Credentials, String> {
    let entries = dotenvy::from_path_iter(path)
        .map_err(|_| "Не удалось открыть выбранный .env".to_owned())?;
    let mut token = None;
    let mut proxy = None;
    for item in entries {
        let (key, value) = item.map_err(|_| {
            "Ошибка формата .env. Проверьте TELOXIDE_TOKEN и TELOXIDE_PROXY.".to_owned()
        })?;
        match key.as_str() {
            "TELOXIDE_TOKEN" if token.is_none() => token = Some(value),
            "TELOXIDE_PROXY" if proxy.is_none() && !value.is_empty() => proxy = Some(value),
            _ => {}
        }
    }
    let token = token.ok_or_else(|| "В выбранном .env нет TELOXIDE_TOKEN".to_owned())?;
    validate_token(&token)?;
    Ok(Credentials { token, proxy })
}
pub fn read_token(path: &Path) -> Result<String, String> {
    read_credentials(path).map(|value| value.token)
}

pub fn redact(text: &str, token: &str) -> String {
    let text = if token.is_empty() {
        text.to_owned()
    } else {
        text.replace(token, "[токен скрыт]")
    };
    // Keep only a short diagnostic; request URLs can contain credentials.
    if text.contains("/bot") || text.contains("api.telegram.org") {
        "Ошибка запроса Telegram API. Проверьте соединение и токен.".into()
    } else {
        text.chars().take(400).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_credentials_are_rejected_without_echoing_them() {
        for token in [
            "",
            "secret",
            "123:short",
            "abc:abcdefghijklmnopqrstuvwxyz",
            "123:abcdefghijklmnopqrst\n",
        ] {
            let error = validate_token(token).expect_err("invalid");
            if !token.is_empty() {
                assert!(!error.contains(token));
            }
        }
        assert!(validate_token("123456789:abcdefghijklmnopqrstuvwxyz_0123456789").is_ok());
    }
    #[test]
    fn preferences_never_contain_credentials() {
        let settings = Settings {
            env_file: Some(PathBuf::from("/example/.env")),
            data_dir: PathBuf::from("/example/data"),
        };
        let json = serde_json::to_value(settings).expect("settings JSON");
        assert_eq!(json.as_object().expect("object").len(), 2);
        assert!(json.get("token").is_none());
    }
    #[test]
    fn logs_remove_token_and_request_urls() {
        let token = "123:abcdefghijklmnopqrstuvwxyz";
        assert!(!redact(&format!("failed {token}"), token).contains(token));
        assert!(
            !redact(&format!("https://api.telegram.org/bot{token}/getMe"), token).contains("https")
        );
    }
    #[test]
    fn selected_env_preserves_proxy_without_mutating_the_environment() {
        let path = std::env::temp_dir().join(format!("telegram-config-{}.env", std::process::id()));
        let before = std::env::var_os("TELOXIDE_TOKEN");
        std::fs::write(&path,"TELOXIDE_TOKEN=123456789:abcdefghijklmnopqrstuvwxyz0123456789\nTELOXIDE_PROXY=http://127.0.0.1:8080\n").expect("test config");
        let config = read_credentials(&path).expect("valid selected config");
        assert_eq!(config.proxy.as_deref(), Some("http://127.0.0.1:8080"));
        assert!(config.token.starts_with("123456789:"));
        assert!(
            std::env::var_os("TELOXIDE_TOKEN") == before,
            "must not mutate process credentials"
        );
        std::fs::remove_file(path).expect("test cleanup");
    }
}
