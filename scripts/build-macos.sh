#!/bin/sh
set -eu
project_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$project_dir"
if [ "$(uname -s)" != Darwin ]; then echo "This packaging script requires macOS." >&2; exit 1; fi
if pgrep -x telegram-my-id >/dev/null; then
    echo "Закройте Telegram My ID перед пересборкой. Запуск готовой сборки: make run." >&2
    exit 1
else
    check=$?
    if [ "$check" -ne 1 ]; then echo "Не удалось проверить запущенные процессы." >&2; exit 1; fi
fi
export MACOSX_DEPLOYMENT_TARGET=11.0
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$project_dir/target}"
cargo build --release --locked --bin telegram-my-id
mkdir -p dist
stage=$(mktemp -d "$project_dir/dist/.telegram-package.XXXXXX")
trap 'rm -rf "$stage"' EXIT HUP INT TERM
app="$stage/TelegramMyID.app"
resources="$app/Contents/Resources"
mkdir -p "$app/Contents/MacOS" "$resources/assets/icons" "$resources/assets/fonts"
cp "$CARGO_TARGET_DIR/release/telegram-my-id" "$app/Contents/MacOS/telegram-my-id"
cp packaging/macos/Info.plist "$app/Contents/Info.plist"
cp assets/icons/TelegramMyID.icns "$resources/TelegramMyID.icns"
cp assets/icons/telegram-my-id-128.png "$resources/assets/icons/telegram-my-id-128.png"
cp assets/fonts/*-OFL.txt "$resources/assets/fonts/"
cp README*.md LICENSE "$resources/"
if [ -d assets/screenshots ]; then cp -R assets/screenshots "$resources/assets/screenshots"; fi
printf 'APPL????' > "$app/Contents/PkgInfo"
plutil -lint "$app/Contents/Info.plist"
codesign --force --sign "${TELEGRAM_MY_ID_SIGNING_IDENTITY:--}" "$app"
codesign --verify --strict "$app"
if [ -d dist/TelegramMyID.app ]; then mv dist/TelegramMyID.app "$stage/previous.app"; fi
mv "$app" dist/TelegramMyID.app
architecture=$(uname -m)
ditto -c -k --norsrc --keepParent dist/TelegramMyID.app "dist/TelegramMyID-macos-$architecture.zip"
echo "Готово: $project_dir/dist/TelegramMyID.app"
