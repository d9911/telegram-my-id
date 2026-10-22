#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
use telegram_bot::desktop::{self, appearance};
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Telegram My ID")
            .with_app_id("org.d9911.telegrammyid")
            .with_inner_size([1120.0, 820.0])
            .with_min_inner_size([760.0, 640.0])
            .with_icon(appearance::icon()),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Telegram My ID",
        options,
        Box::new(|cc| Ok(Box::new(desktop::App::new(cc)))),
    )
}
