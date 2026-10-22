pub mod appearance;
mod controller;
mod preview;
use crate::{
    runtime::{BotEvent, RunConfig},
    settings::{self, Settings},
};
use appearance::*;
use controller::{Controller, Phase};
use eframe::egui::{self, Align, Layout, RichText, Stroke};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Overview,
    Commands,
    Settings,
}
#[derive(Clone, Copy, PartialEq)]
enum TokenSource {
    File,
    Temporary,
}
struct LogEntry {
    time: String,
    text: String,
    error: bool,
}
pub struct App {
    controller: Controller,
    settings: Settings,
    token_source: TokenSource,
    temporary_token: String,
    tab: Tab,
    brand: egui::TextureHandle,
    logs: VecDeque<LogEntry>,
    error: Option<String>,
    command: usize,
    language: usize,
    started: Option<Instant>,
    closing: bool,
    help: bool,
}
impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        apply(&cc.egui_ctx);
        let (settings, error) = match Settings::load() {
            Ok(s) => (s, None),
            Err(e) => (Settings::default(), Some(e)),
        };
        let image = eframe::icon_data::from_png_bytes(include_bytes!(
            "../../assets/icons/telegram-my-id-512.png"
        ))
        .expect("bundled illustration");
        let brand = cc.egui_ctx.load_texture(
            "telegram-my-id-brand",
            egui::ColorImage::from_rgba_unmultiplied(
                [image.width as usize, image.height as usize],
                &image.rgba,
            ),
            egui::TextureOptions::LINEAR,
        );
        let mut app = Self {
            controller: Controller::default(),
            settings,
            token_source: TokenSource::File,
            temporary_token: String::new(),
            tab: Tab::Overview,
            brand,
            logs: VecDeque::new(),
            error,
            command: 1,
            language: 0,
            started: None,
            closing: false,
            help: false,
        };
        app.log(
            "Приложение открыто. Бот остановлен; подключение начнётся по кнопке.",
            false,
        );
        app
    }
    fn log(&mut self, text: impl Into<String>, error: bool) {
        self.logs.push_back(LogEntry {
            time: chrono::Local::now().format("%H:%M:%S").to_string(),
            text: text.into(),
            error,
        });
        while self.logs.len() > 150 {
            self.logs.pop_front();
        }
    }
    fn start(&mut self, ctx: &egui::Context) {
        let token = match self.token_source {
            TokenSource::File => self
                .settings
                .env_file
                .as_ref()
                .ok_or_else(|| "Выберите файл .env во вкладке «Настройки».".to_owned())
                .and_then(|p| settings::read_credentials(p)),
            TokenSource::Temporary => {
                settings::validate_token(&self.temporary_token).map(|()| settings::Credentials {
                    token: self.temporary_token.clone(),
                    proxy: None,
                })
            }
        };
        let result = token.and_then(|credentials| {
            self.controller.start(
                RunConfig {
                    token: credentials.token,
                    proxy: credentials.proxy,
                    data_dir: self.settings.data_dir.clone(),
                },
                ctx.clone(),
            )
        });
        match result {
            Ok(()) => {
                self.error = None;
                self.started = Some(Instant::now());
                self.log("Запуск. Проверяю токен и подключение к Telegram…", false);
            }
            Err(error) => {
                self.log(error.clone(), true);
                self.error = Some(error);
                self.tab = Tab::Settings;
            }
        }
    }
    fn stop(&mut self) {
        self.controller.stop();
        self.log("Запрошена остановка. Завершаю текущие обработчики…", false);
    }
    fn poll(&mut self) {
        for event in self.controller.poll() {
            match event {
                BotEvent::Connecting => {}
                BotEvent::Connected { username, id } => self.log(
                    format!("Telegram подтвердил @{username} · ID {id}. Запущен long polling."),
                    false,
                ),
                BotEvent::Stopping => {}
                BotEvent::Stopped => {
                    self.log("Бот остановлен.", false);
                    self.started = None;
                }
                BotEvent::Error(error) => {
                    self.error = Some(error.clone());
                    self.log(error, true);
                }
            }
        }
    }
    fn status(&self) -> &str {
        match self.controller.phase {
            Phase::Idle => "Остановлен",
            Phase::Connecting => "Подключение…",
            Phase::Running if self.controller.had_errors => "Запущен · есть ошибки",
            Phase::Running => "Запущен",
            Phase::Stopping => "Остановка…",
        }
    }
    fn run_button(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let active = self.controller.active();
        let text = if active {
            "Остановить бота"
        } else {
            "Запустить бота"
        };
        if ui
            .add_enabled(
                self.controller.phase != Phase::Stopping,
                egui::Button::new(RichText::new(text).color(egui::Color32::WHITE))
                    .fill(INK)
                    .stroke(Stroke::NONE)
                    .min_size(egui::vec2(180.0, BUTTON_HEIGHT)),
            )
            .clicked()
        {
            if active {
                self.stop();
            } else {
                self.start(ctx);
            }
        }
    }
    fn header(&mut self, root: &mut egui::Ui, ctx: &egui::Context) {
        egui::Panel::top("header")
            .frame(
                egui::Frame::new()
                    .fill(CANVAS)
                    .inner_margin(egui::Margin::symmetric(28, 12)),
            )
            .show(root, |ui| {
                ui.horizontal(|ui| {
                    ui.image((self.brand.id(), egui::vec2(40.0, 40.0)));
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Telegram My ID").font(display(20.0)));
                        ui.label(
                            RichText::new("Ваш бот. В одном окне.")
                                .size(12.0)
                                .color(MUTED),
                        );
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("Справка").clicked() {
                            self.help = true;
                        }
                        for (tab, label) in [
                            (Tab::Settings, "Настройки"),
                            (Tab::Commands, "Команды"),
                            (Tab::Overview, "Обзор"),
                        ] {
                            if ui.selectable_label(self.tab == tab, label).clicked() {
                                self.tab = tab;
                            }
                        }
                    });
                });
                ui.add_space(4.0);
            });
        egui::Panel::bottom("status")
            .frame(
                egui::Frame::new()
                    .fill(CANVAS)
                    .inner_margin(egui::Margin::symmetric(28, 10)),
            )
            .show(root, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(self.status()).size(12.0).color(
                        if self.controller.had_errors {
                            ERROR
                        } else {
                            MUTED
                        },
                    ));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!(
                                "v{} · ⌘/Ctrl+Enter — запуск · Esc — стоп",
                                env!("CARGO_PKG_VERSION")
                            ))
                            .size(12.0)
                            .color(MUTED),
                        );
                    });
                });
            });
        if self.help {
            egui::Window::new("Как пользоваться").open(&mut self.help).collapsible(false).resizable(false).show(ctx,|ui| {
                ui.set_max_width(470.0);
                ui.label("1. В настройках выберите свой .env с TELOXIDE_TOKEN или введите токен на время сессии.\n2. Выберите папку данных и при желании сохраните пути.\n3. Нажмите «Запустить бота». Откройте его в Telegram и отправьте /start.");
                ui.add_space(12.0);
                ui.label("Запуск выполняется только вручную. Не запускайте этот же токен одновременно в другом приложении или CLI. При закрытии окна бот останавливается.");
                ui.add_space(12.0);
                ui.label("«Команды» — локальный предпросмотр с вымышленным пользователем. Он не отправляет сообщения. Журнал хранит последние 150 событий только в памяти.");
                ui.add_space(12.0);
                ui.label("Токен не записывается в настройки приложения. Папка .app не содержит .env или базу данных. Для прекращения работы используйте кнопку остановки или Esc.");
            });
        }
    }
    fn overview(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let wide = ui.available_width() >= 880.0;
        ui.horizontal_top(|ui| {
            let width=if wide {ui.available_width()-360.0}else{ui.available_width()};
            ui.allocate_ui_with_layout(egui::vec2(width,0.0),Layout::top_down(Align::Min),|ui| {
                ui.set_min_width(width);
                ui.add_space(8.0);
                ui.label(RichText::new("Бот под рукой.").font(display(40.0)));
                ui.label(RichText::new("Узнать Telegram ID, сменить язык,\nвыбрать число или вариант из списка.").size(18.0).color(BODY));
                ui.add_space(16.0);
                ui.horizontal_wrapped(|ui| {
                    self.run_button(ui,ctx);
                    if ui.add(egui::Button::new("Настроить подключение").min_size(egui::vec2(0.0,BUTTON_HEIGHT))).clicked(){self.tab=Tab::Settings;}
                });
            });
            if wide {
                egui::Frame::new().fill(PEACH).corner_radius(24).inner_margin(16).show(ui,|ui| {
                    ui.set_width(300.0);
                    ui.vertical_centered(|ui|{ui.image((self.brand.id(),egui::vec2(175.0,175.0)));ui.label(RichText::new("Telegram My ID").size(14.0).color(INK));});
                });
            }
        });
        ui.add_space(24.0);
        if ui.available_width() >= 880.0 {
            let left = (ui.available_width() - 24.0) * 0.40;
            ui.horizontal_top(|ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(left, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| self.connection_card(ui),
                );
                ui.add_space(12.0);
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), 0.0),
                    Layout::top_down(Align::Min),
                    |ui| self.journal(ui),
                );
            });
        } else {
            self.connection_card(ui);
            ui.add_space(16.0);
            self.journal(ui);
        }
        ui.add_space(20.0);
        let cards = [
            (
                LAVENDER,
                "Ваш ID",
                "/info_user",
                "Личный ID и данные профиля",
            ),
            (
                OCHRE,
                "Четыре языка",
                "RU · EN · ES · KO",
                "Язык выбирается в Telegram",
            ),
            (
                PINK,
                "Случайный выбор",
                "Число или вариант",
                "Два инструмента для выбора",
            ),
        ];
        if ui.available_width() > 880.0 {
            ui.columns(3, |columns| {
                for (column, (color, title, value, note)) in columns.iter_mut().zip(cards) {
                    Self::feature(column, color, title, value, note);
                }
            });
        } else {
            for (color, title, value, note) in cards {
                Self::feature(ui, color, title, value, note);
                ui.add_space(8.0);
            }
        }
    }
    fn feature(ui: &mut egui::Ui, color: egui::Color32, title: &str, value: &str, note: &str) {
        egui::Frame::new()
            .fill(color)
            .corner_radius(RADIUS)
            .inner_margin(PAD)
            .show(ui, |ui| {
                ui.set_min_width((ui.available_width() - 1.0).max(0.0));
                ui.label(RichText::new(title).size(14.0).color(INK));
                ui.label(RichText::new(value).font(display(21.0)).color(INK));
                ui.label(RichText::new(note).size(12.0).color(INK));
            });
    }
    fn connection_card(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(TEAL)
            .corner_radius(RADIUS)
            .inner_margin(24)
            .show(ui, |ui| {
                ui.set_min_width((ui.available_width() - 1.0).max(0.0));
                ui.label(RichText::new("Состояние бота").size(14.0).color(MINT));
                ui.label(
                    RichText::new(self.status())
                        .font(display(27.0))
                        .color(CREAM),
                );
                ui.add_space(8.0);
                if let Some((username, id)) = &self.controller.identity {
                    ui.label(RichText::new(format!("@{username}\nID {id}")).color(CREAM));
                } else {
                    ui.label(
                        RichText::new("Имя и ID бота появятся после\nподключения к Telegram.")
                            .color(CREAM),
                    );
                }
                ui.add_space(12.0);
                let source = match self.token_source {
                    TokenSource::File => self
                        .settings
                        .env_file
                        .as_ref()
                        .and_then(|p| p.file_name())
                        .map(|s| format!("Источник токена: {}", s.to_string_lossy()))
                        .unwrap_or_else(|| "Токен ещё не выбран".into()),
                    TokenSource::Temporary => "Токен на время сессии".into(),
                };
                ui.label(RichText::new(source).size(12.0).color(CREAM));
                if let Some(started) = self.started {
                    ui.label(
                        RichText::new(format!(
                            "Время с запуска: {:.0} с",
                            started.elapsed().as_secs_f64()
                        ))
                        .size(12.0)
                        .color(CREAM),
                    );
                }
                ui.add_space(8.0);
            });
    }
    fn journal(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(SURFACE)
            .corner_radius(RADIUS)
            .inner_margin(PAD)
            .show(ui, |ui| {
                ui.set_min_width((ui.available_width() - 1.0).max(0.0));
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Журнал событий").font(display(20.0)));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.small_button("Очистить").clicked() {
                            self.logs.clear();
                        }
                    });
                });
                egui::ScrollArea::vertical()
                    .id_salt("event-log")
                    .auto_shrink([false, false])
                    .max_height(140.0)
                    .min_scrolled_height(120.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        if self.logs.is_empty() {
                            ui.label(RichText::new("Пока нет событий").color(MUTED));
                        }
                        for entry in &self.logs {
                            ui.horizontal_top(|ui| {
                                ui.label(
                                    RichText::new(&entry.time)
                                        .monospace()
                                        .size(12.0)
                                        .color(MUTED),
                                );
                                ui.label(
                                    RichText::new(&entry.text).size(13.0).color(if entry.error {
                                        ERROR
                                    } else {
                                        BODY
                                    }),
                                );
                            });
                        }
                    });
            });
    }
    fn commands(&mut self, ui: &mut egui::Ui) {
        ui.heading("Что умеет бот");
        ui.label(
            RichText::new(
                "Локальный предпросмотр · демонстрационные данные, без подключения к Telegram",
            )
            .color(MUTED),
        );
        ui.add_space(16.0);
        ui.horizontal_wrapped(|ui| {
            for (index, (code, label)) in preview::LANGUAGES.iter().enumerate() {
                if ui
                    .selectable_label(
                        self.language == index,
                        format!("{} · {label}", code.to_uppercase()),
                    )
                    .clicked()
                {
                    self.language = index;
                }
            }
        });
        ui.add_space(16.0);
        if ui.available_width() >= 880.0 {
            ui.horizontal_top(|ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(300.0, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| self.command_list(ui),
                );
                ui.add_space(12.0);
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), 0.0),
                    Layout::top_down(Align::Min),
                    |ui| self.command_preview(ui),
                );
            });
        } else {
            self.command_list(ui);
            ui.add_space(20.0);
            self.command_preview(ui);
        }
    }
    fn command_list(&mut self, ui: &mut egui::Ui) {
        for (index, (command, description)) in preview::COMMANDS.iter().enumerate() {
            let selected = self.command == index;
            if ui
                .add_sized(
                    [ui.available_width(), 42.0],
                    egui::Button::new(RichText::new(*command).monospace())
                        .fill(if selected { LAVENDER } else { CANVAS })
                        .stroke(Stroke::new(1.0, if selected { LAVENDER } else { BORDER })),
                )
                .on_hover_text(*description)
                .clicked()
            {
                self.command = index;
            }
        }
    }
    fn command_preview(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(LAVENDER)
            .corner_radius(24)
            .inner_margin(24)
            .show(ui, |ui| {
                ui.set_min_width((ui.available_width() - 1.0).max(0.0));
                ui.label(
                    RichText::new(preview::COMMANDS[self.command].0)
                        .monospace()
                        .font(display(20.0)),
                );
                ui.label(
                    RichText::new(preview::COMMANDS[self.command].1)
                        .size(13.0)
                        .color(BODY),
                );
                ui.add_space(16.0);
                egui::Frame::new()
                    .fill(CREAM)
                    .corner_radius(16)
                    .inner_margin(20)
                    .show(ui, |ui| {
                        ui.set_min_width((ui.available_width() - 1.0).max(0.0));
                        ui.label(
                            RichText::new(preview::text(
                                self.command,
                                preview::LANGUAGES[self.language].0,
                            ))
                            .color(INK),
                        );
                    });
                ui.add_space(12.0);
                ui.label(
                    RichText::new("Alex Example · ID 100000001 — вымышленный пользователь.")
                        .size(12.0)
                        .color(INK),
                );
            });
    }
    fn configuration(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Настройки подключения");
        ui.label(
            RichText::new(
                "Подключите свой бот от @BotFather. Токен не сохраняется в настройках приложения.",
            )
            .color(MUTED),
        );
        ui.add_space(20.0);
        egui::Frame::new().fill(SURFACE).corner_radius(RADIUS).inner_margin(24).show(ui,|ui| {
            ui.set_min_width((ui.available_width()-1.0).max(0.0));
            ui.add_enabled_ui(!self.controller.active(),|ui| {
                ui.label(RichText::new("1. Источник токена").font(display(20.0)));
                ui.horizontal_wrapped(|ui| {let changed=ui.selectable_value(&mut self.token_source,TokenSource::File,"Файл .env").changed();
                    let changed=ui.selectable_value(&mut self.token_source,TokenSource::Temporary,"Ввести на время сессии").changed() || changed;
                    if changed {self.error=None;}});
                match self.token_source {
                    TokenSource::File=>{
                        ui.label(self.settings.env_file.as_ref().map(|p|p.display().to_string()).unwrap_or_else(||"Файл не выбран".into()));
                        if ui.button("Выбрать .env…").clicked()&& let Some(path)=rfd::FileDialog::new().set_title("Выберите .env с TELOXIDE_TOKEN").pick_file(){self.settings.env_file=Some(path);self.error=None;}
                        ui.label(RichText::new("Содержимое не отображается. В диалоге macOS ⌘⇧. показывает скрытые файлы.").size(12.0).color(MUTED));
                    },
                    TokenSource::Temporary=>{
                        ui.add_sized([ui.available_width(),44.0],egui::TextEdit::singleline(&mut self.temporary_token).password(true).hint_text("123456789:токен от @BotFather").margin(egui::vec2(12.0,10.0)));
                        ui.label(RichText::new("Только в памяти до закрытия приложения. Вставьте токен целиком без пробелов.").size(12.0).color(MUTED));
                    },
                }
                ui.add_space(20.0);
                ui.label(RichText::new("2. Папка данных").font(display(20.0)));
                ui.label(self.settings.data_dir.display().to_string());
                if ui.button("Выбрать папку…").clicked()&& let Some(path)=rfd::FileDialog::new().set_title("Папка с languages.json").pick_folder(){self.settings.data_dir=path;}
                ui.label(RichText::new("Здесь хранится languages.json с выбранными языками пользователей. Чтобы использовать прежнюю базу, выберите её папку src/db/data; перенос не выполняется автоматически.").size(12.0).color(MUTED));
                ui.add_space(16.0);
                if ui.button("Сохранить пути").clicked(){match self.settings.save(){Ok(())=>{self.error=None;self.log("Пути настроек сохранены. Токен в настройки не записывался.",false);},Err(e)=>self.error=Some(e)}}
            });
            if self.controller.active(){ui.label(RichText::new("Остановите бота, чтобы изменить подключение.").color(MUTED));}
        });
        ui.add_space(16.0);
        ui.horizontal(|ui| self.run_button(ui, ctx));
        if let Some(error) = &self.error {
            ui.add_space(8.0);
            ui.colored_label(ERROR, error);
        }
    }
}
impl eframe::App for App {
    fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = root.ctx().clone();
        self.poll();
        if ctx.input(|i| i.viewport().close_requested()) && self.controller.active() {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.closing = true;
            if self.controller.phase != Phase::Stopping {
                self.stop();
            }
        }
        if self.closing && !self.controller.active() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if !self.controller.active()
            && ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::Enter))
        {
            self.start(&ctx);
        }
        if self.controller.active()
            && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
        {
            self.stop();
        }
        self.header(root, &ctx);
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(CANVAS)
                    .inner_margin(egui::Margin::symmetric(28, 20)),
            )
            .show(root, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("workspace")
                    .show(ui, |ui| match self.tab {
                        Tab::Overview => self.overview(ui, &ctx),
                        Tab::Commands => self.commands(ui),
                        Tab::Settings => self.configuration(ui, &ctx),
                    });
            });
        if self.controller.active() {
            ctx.request_repaint_after(Duration::from_millis(50));
        }
    }
}
