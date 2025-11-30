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
    translations.insert("test_desc", "봇 기능 테스트");
    translations.insert("test_title", "봇 기능 테스트");
    translations.insert("test_status", "시스템 상태");
    translations.insert("test_bot_online", "봇이 실행 중입니다");
    translations.insert("test_translations", "번역이 로드되었습니다");
    translations.insert("test_formatting", "포맷팅이 작동합니다");
    translations.insert("test_current_lang", "현재 언어");
    translations.insert("test_selected_lang", "선택된 언어");
    translations.insert("test_translations_examples", "번역 예제");
    translations.insert("test_formatting_demo", "포맷팅 데모");
    translations.insert("test_user_info", "귀하의 정보");
    translations.insert("test_statistics", "통계");
    translations.insert("test_users_count", "언어가 저장된 사용자 수");
    translations.insert("test_footer", "모든 시스템이 정상적으로 작동합니다! ✅");
    translations.insert("random_number_desc", "무작위 숫자 생성");
    translations.insert("random_number_prompt", "🎲 숫자 범위를 형식으로 입력하세요:\n<code>부터 까지</code>\n\n예시:\n• <code>1 100</code>\n• <code>1-100</code>\n• <code>1,100</code>");
    translations.insert("random_number_result", "무작위 숫자");
    translations.insert("random_number_selected", "결과");
    translations.insert("random_number_range", "범위");
    translations.insert("random_number_error_format", "❌ 잘못된 형식입니다. 사용: <code>부터 까지</code>\n\n예시: <code>1 100</code>");
    translations.insert("random_number_error_range", "❌ 최소값은 최대값보다 작거나 같아야 합니다");
    translations.insert("random_choice_desc", "목록에서 무작위 값 선택");
    translations.insert("random_choice_prompt", "🎯 쉼표로 구분된 값 목록을 보내세요.\n\n예시:\n<code>사과, 바나나, 오렌지, 배</code>");
    translations.insert("random_choice_result", "선택 결과");
    translations.insert("random_choice_selected", "선택됨");
    translations.insert("random_choice_options", "옵션");
    translations.insert("random_choice_error_empty", "❌ 목록이 비어있을 수 없습니다. 쉼표로 구분된 값을 보내세요.");
    translations.insert("random_choice_error_format", "❌ 쉼표로 구분된 값 목록을 보내세요.\n\n예시: <code>사과, 바나나, 오렌지</code>");
    
    translations
}

