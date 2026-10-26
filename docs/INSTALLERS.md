# Telegram My ID: установка и локальная сборка

Версия берётся из `Cargo.toml`, идентификатор остаётся `org.d9911.telegrammyid`.
GUI `telegram-my-id` и headless `telegram-bot` входят в один пакет. Готовому приложению
не нужны исходники, Rust, Cargo, Python, Make или открытый терминал.

## Исходное состояние

| ОС / архитектура | Прежняя команда | Зависимости сборки | Прежний артефакт | Пробел |
| --- | --- | --- | --- | --- |
| macOS arm64 / Intel | `make build` | Rust, Xcode CLT | GUI `.app`, ZIP | DMG, включение CLI, проверка установленной копии |
| Windows x64 | отсутствовала | Rust MSVC, C++ Build Tools / Windows SDK | отсутствовал | EXE-ресурс, runtime DLL, установщик и удаление |
| Ubuntu arm64 / amd64 | Cargo вручную | Rust, C toolchain, GUI/OpenSSL headers | отдельный executable | DEB, runtime-зависимости, меню |

Таблица описывает прежние сценарии; проверенные архитектуры указаны ниже отдельно.

## Команды

| Команда | Назначение |
| --- | --- |
| `make help` | Команды и путь результата |
| `make doctor` | Диагностика инструментов без изменения системы |
| `make setup` | Нативный Rust target, rustfmt/Clippy; DMG-инструменты в проектной venv на Mac |
| `make dev` | Прежний режим разработки GUI; цель по умолчанию |
| `make cli` | Прежний headless запуск через Cargo |
| `make build` | Release GUI + CLI и установщик текущей ОС, ZIP на Mac/Windows |
| `make local` | Собрать текущий код и открыть установщик |
| `make run` | Открыть существующую release-копию без пересборки |
| `make start` | Собрать и открыть release-копию |
| `make check` | Существующие fmt, строгий Clippy и тесты |

Все действия доступны в PowerShell без GNU Make: `./package.ps1 help`,
`./package.ps1 setup`, `./package.ps1 local`, `./package.ps1 run`.
`-PythonExe` принимает путь к Python. Скрипты самостоятельны и делегируют
одному локальному `scripts/project.py`; соседние репозитории не нужны.

Сборщик получает точные пути текущих executable из JSON Cargo, учитывает
`CARGO_TARGET_DIR` и пробелы в пути. Ошибка останавливает последующие шаги.
Приложение, установщики и manifest продвигаются с откатом прежних результатов;
активные процессы не завершаются и проверяются повторно перед заменой.

## macOS

Для сборки: Rust не ниже `rust-version`, Python 3.11+, Xcode Command Line Tools.
DMG-инструменты закреплены в `scripts/requirements-package.txt` и устанавливаются
в `build/package-tools`; системный Python не меняется.

```sh
make setup
make doctor
make local
```

Результаты для arm64:

- `dist/TelegramMyID.app`;
- `dist/TelegramMyID-0.2.0-macos-arm64.dmg`;
- `dist/TelegramMyID-macos-arm64.zip`.

Перетащите приложение из DMG в Applications, подтвердите замену самостоятельно.
Перед заменой остановите бота и закройте GUI/CLI. CLI внутри приложения:
`TelegramMyID.app/Contents/MacOS/telegram-bot`.

Проверяются подписи исходной .app, копии внутри DMG и checksum образа. По умолчанию
подпись локальная ad-hoc. Для имеющегося Developer ID сохранена переменная
`TELEGRAM_MY_ID_SIGNING_IDENTITY`; notarization и публикация не выполняются.
arm64 не является universal. Intel требует отдельной нативной сборки и проверки.

## Windows x64 MSVC

Для сборки: Rust MSVC, Python 3.11+, Visual Studio C++ Build Tools и Windows SDK,
Inno Setup 6.3+. Запускайте Developer PowerShell с `cl.exe`, `rc.exe` и `dumpbin.exe`
в PATH. Вместо dumpbin поддерживается objdump. Путь Inno можно задать через `ISCC`.

```powershell
.\package.ps1 setup
.\package.ps1 doctor
.\package.ps1 local
```

Результаты: `dist/windows-x64/TelegramMyID.exe` и `telegram-bot.exe`,
`dist/TelegramMyID-windows-x64.zip`,
`dist/TelegramMyID-0.2.0-windows-x86_64-setup.exe`.
Windows-ресурс содержит существующую иконку GUI. Реальные импорты EXE/DLL обходятся
рекурсивно; отсутствующая несистемная библиотека прекращает упаковку. VC runtime
включается при наличии импорта; установленный Cargo не предполагается.

Установка для текущего пользователя, ярлык в меню и необязательный ярлык рабочего
стола, штатное удаление. Общий mutex GUI и headless процесса блокирует обновление
и удаление работающей копии. Автозапуск, службы и запуск бота установщиком не создаются.
Нативный Windows-пакет в этой итерации не собран и не проверен.

## Ubuntu 24.04

Инструменты сборки, использованные в проверочной Ubuntu:

```sh
sudo apt-get update
sudo apt-get install build-essential pkg-config python3 python3-venv dpkg-dev binutils xdg-utils libx11-dev libxi-dev libxtst-dev libxrandr-dev libxcursor-dev libxinerama-dev libxkbcommon-dev libxkbcommon-x11-dev libgl1-mesa-dev libegl1-mesa-dev libwayland-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libssl-dev libdbus-1-dev
make setup
make doctor
make build
sudo apt install ./dist/telegram-my-id_0.2.0_arm64.deb
```

На amd64 используйте нативно собранный `_amd64.deb`. Проверочная среда — Ubuntu
24.04 arm64 с X11/Mesa/Xvfb. Другие версии Ubuntu, amd64 и полноценный Wayland-сеанс
требуют отдельной проверки.

В меню устанавливаются `.desktop` и иконка. GUI запускается через
`/usr/bin/telegram-my-id`, CLI — `/usr/bin/telegram-bot`; бинарники и публичные
ресурсы лежат в `/opt/telegram-my-id`. ELF-зависимости вычисляет `dpkg-shlibdeps`,
динамически загружаемые GUI-библиотеки и desktop portal перечислены дополнительно.
Для системных диалогов нужен работающий desktop portal. Работающая установленная
копия блокирует обновление и удаление, пользовательские каталоги не удаляются.

## Настройки, токены и база языков

Установка и открытие GUI не запускают бота. Подключение начинается только по
кнопке пользователя. Настройки сохраняют пути, токен в них не сериализуется.
Упаковка включает только исполняемые файлы, assets, публичные README и лицензии;
`.env`, `settings.json` и `src/db/data` в whitelist не входят.

Пути GUI сохранены:

- macOS: `~/Library/Application Support/TelegramMyID/settings.json`;
- Windows: `%APPDATA%\TelegramMyID\settings.json`;
- Ubuntu: `${XDG_DATA_HOME:-~/.local/share}/telegram-my-id/settings.json`.

По умолчанию данные GUI находятся в подкаталоге `data`. Выбранный пользователем
каталог может находиться в другом месте; обновление/удаление его не затрагивает.
Существующую базу можно подключить в настройках без автоматического переноса.

Headless читает `TELOXIDE_TOKEN` из окружения/локального `.env`, необязательный
`TELOXIDE_PROXY` и явный `TELEGRAM_MY_ID_DATA_DIR`. Если переменная каталога отсутствует,
существующая `src/db/data` в текущем каталоге сохраняет прежнюю совместимость.
Вне checkout CLI использует пользовательский `application_dir()/data`, а не каталог
установки. Не запускайте headless с настоящим токеном одновременно с GUI.

Для проверки CLI без сети запускайте из пустого каталога без `.env`, с пустым
`TELOXIDE_TOKEN`: ожидается ошибка формата и ненулевой код без создания базы.
Проверки polling выполняются существующим тестом с локальным API; реальные сообщения
Telegram для проверки установщика не нужны.

## Источники требований

- Локально: `Cargo.toml`, `Cargo.lock`, `src/settings.rs`, `src/runtime.rs`, `src/bin/telegram-bot.rs`, `packaging/app.json`, `scripts/project.py`.
- [Inno Setup AppMutex](https://jrsoftware.org/ishelp/topic_setup_appmutex.htm).
- [eframe: нативные зависимости GUI](https://github.com/emilk/egui/tree/main/crates/eframe).


## Выполненная проверка 26 октября 2026

- macOS arm64: fmt, строгий Clippy, 17 существующих тестов; GUI + CLI, .app/DMG/ZIP собраны. Подписи исходной и DMG-копии, checksum проверены.
- Копия из read-only DMG вне checkout открыта обычным пользователем через LaunchServices: окно, иконка, ресурсы, версия и статус «Остановлен» подтверждены. Бот не запускался.
- Установленный headless EXE запущен с системным PATH и пустым токеном из пустого каталога: корректная ошибка, файлов данных не создано.
- Ubuntu 24.04 arm64: fmt, строгий Clippy и те же 17 тестов; проект и явный `CARGO_TARGET_DIR` содержали пробелы. Реальный DEB собран и установлен.
- GUI проверен обычным пользователем с сохранёнными синтетическими путями и базой языков. Проверочный локальный прокси не получил соединений при открытии; bot.lock не появился, файлы не менялись. Настоящие токены и Telegram-сообщения не использовались.
- Активная копия блокирует обновление и удаление. После закрытия оба действия прошли; settings.json, выбранный каталог, пробная база и внешний пробный .env сохранились побайтно.
- `.desktop` прошёл desktop-file-validate; `ldd` обоих бинарников не содержит отсутствующих библиотек, OpenSSL учтён в DEB-зависимостях.
- Все три сборщика дополнительно прошли практическую инъекцию ошибки продвижения файлов: приложение, установщики и manifest откатились. Поздняя output-symlink отклоняется до первой замены. Новых тестов в репозиториях не добавлено.
- Windows, Intel Mac, Ubuntu amd64, полноценный Wayland и Citrix не проверены.

Публичная инструкция `docs/INSTALLERS.md` также включается в пакет с сохранением относительного пути; пользовательские документы и данные не включаются.

Дополнительная macOS-проверка завершена: полноценный DMG построен из отдельной копии проекта с пробелами и явным `CARGO_TARGET_DIR` с пробелами; executable и публичная инструкция получены из этой сборки.


## Дополнение: фоновый режим

Добавлена сохраняемая настройка продолжения работы после закрытия окна. Повторное открытие управляет отдельным worker. Старое описание закрытия относится к выключенному фоновому режиму. Перед обновлением остановите бота, а затем закройте окно. Точные команды и свежие проверки: [BACKGROUND.md](BACKGROUND.md).
