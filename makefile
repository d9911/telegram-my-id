.DEFAULT_GOAL := dev
CARGO ?= cargo
.PHONY: dev cli start build run check test lint fmt fmt-check icons help
help:
	@echo "make: GUI development | make build: .app and ZIP | make run: open existing app | make cli: headless bot | make check: formatting, Clippy and tests"
dev:
	$(CARGO) run --locked --bin telegram-my-id
cli:
	$(CARGO) run --locked --bin telegram-bot
start: build
build:
	sh scripts/build-macos.sh
run start:
	@test -x dist/TelegramMyID.app/Contents/MacOS/telegram-my-id || { echo "Run make build first." >&2; exit 1; }
	open dist/TelegramMyID.app
check: fmt-check lint test
test:
	$(CARGO) test --locked
lint:
	$(CARGO) clippy --locked --all-targets --all-features -- -D warnings
fmt:
	$(CARGO) fmt --all
fmt-check:
	$(CARGO) fmt --all -- --check
icons:
	node scripts/generate-icons.mjs
