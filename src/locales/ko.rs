use super::Translations;

pub fn get_translations() -> Translations {
    let mut translations = Translations::new();
    
    translations.insert("user_info_title", "👤 사용자 정보:");
    translations.insert("id", "🆔 ID");
    translations.insert("bot", "🤖 봇");
    translations.insert("first_name", "👋 이름");
    translations.insert("last_name", "👤 성");
    translations.insert("username", "✉️ 사용자 이름");
    translations.insert("language", "🌐 언어");
    translations.insert("yes", "예");
    translations.insert("no", "아니오");
    translations.insert("error_user_info", "❌ 사용자 정보를 가져올 수 없습니다.");
    translations.insert("help_title", "사용 가능한 명령:");
    translations.insert("info_user_desc", "사용자 정보 표시");
    translations.insert("help_desc", "도움말 표시");
    translations.insert("language_set", "✅ 언어가 한국어로 변경되었습니다");
    translations.insert("language_set_short", "언어 변경됨");
    translations.insert("language_confirmed", "이제 모든 메시지가 한국어로 표시됩니다.\n\n자신에 대한 정보를 얻으려면 /info_user를 사용하세요.");
    translations.insert("select_language", "인터페이스 언어 선택:");
    translations.insert("language_desc", "인터페이스 언어 선택");
    translations.insert("welcome_message", "👋 환영합니다!");
    translations.insert("welcome_description", "저는 Telegram 사용자 정보를 가져오는 봇입니다.\n\n📋 사용 가능한 명령:\n• /info_user - 자신에 대한 정보 가져오기\n• /help - 모든 명령 목록\n• /language - 언어 변경");
    translations.insert("start_desc", "봇과 함께 작업 시작");
    
    translations
}

