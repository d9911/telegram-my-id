use eframe::egui::{self, Color32, FontId, Stroke, TextStyle};
// Primitive brand palette from the supplied Clay design reference.
pub const CREAM: Color32 = Color32::from_rgb(255, 250, 240);
pub const CARD: Color32 = Color32::from_rgb(245, 240, 224);
pub const TEAL: Color32 = Color32::from_rgb(26, 58, 58);
pub const LAVENDER: Color32 = Color32::from_rgb(184, 164, 237);
pub const PEACH: Color32 = Color32::from_rgb(255, 176, 132);
pub const OCHRE: Color32 = Color32::from_rgb(232, 185, 74);
pub const PINK: Color32 = Color32::from_rgb(255, 77, 139);
pub const MINT: Color32 = Color32::from_rgb(164, 212, 197);
// Semantic aliases; components use these rather than new color literals.
pub const CANVAS: Color32 = CREAM;
pub const SURFACE: Color32 = CARD;
pub const INK: Color32 = Color32::from_rgb(10, 10, 10);
pub const BODY: Color32 = Color32::from_rgb(58, 58, 58);
pub const MUTED: Color32 = Color32::from_rgb(106, 106, 106);
pub const BORDER: Color32 = Color32::from_rgb(221, 215, 201);
pub const ERROR: Color32 = Color32::from_rgb(169, 31, 49);
pub const RADIUS: u8 = 16;
pub const PAD: i8 = 20;
pub const BUTTON_HEIGHT: f32 = 44.0;

pub fn display(size: f32) -> FontId {
    FontId::new(size, egui::FontFamily::Name("display".into()))
}
pub fn apply(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let inter = egui::FontData::from_static(include_bytes!("../../assets/fonts/Inter.ttf"));
    let medium = inter.clone().tweak(egui::FontTweak {
        coords: egui::epaint::text::VariationCoords::new([(b"wght", 500.0)]),
        ..Default::default()
    });
    fonts
        .font_data
        .insert("Inter".into(), std::sync::Arc::new(inter));
    fonts
        .font_data
        .insert("Inter Medium".into(), std::sync::Arc::new(medium));
    let korean = egui::FontData::from_static(include_bytes!("../../assets/fonts/NotoSansKR.ttf"))
        .tweak(egui::FontTweak {
            coords: egui::epaint::text::VariationCoords::new([(b"wght", 400.0)]),
            ..Default::default()
        });
    fonts
        .font_data
        .insert("Noto Sans KR".into(), std::sync::Arc::new(korean));
    fonts.font_data.insert(
        "Noto Emoji".into(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/NotoEmoji.ttf"
        ))),
    );
    let fallback = fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default();
    fallback.insert(0, "Noto Sans KR".into());
    fallback.insert(0, "Noto Emoji".into());
    fallback.insert(0, "Inter".into());
    let mut display_fonts = fallback.clone();
    display_fonts[0] = "Inter Medium".into();
    fonts
        .families
        .insert(egui::FontFamily::Name("display".into()), display_fonts);
    ctx.set_fonts(fonts);
    ctx.set_theme(egui::Theme::Light);
    let mut visuals = egui::Visuals::light();
    visuals.panel_fill = CANVAS;
    visuals.window_fill = CANVAS;
    visuals.extreme_bg_color = CREAM;
    visuals.text_edit_bg_color = Some(CREAM);
    visuals.override_text_color = Some(INK);
    visuals.weak_text_color = Some(MUTED);
    visuals.selection.bg_fill = LAVENDER;
    visuals.selection.stroke = Stroke::new(1.0, INK);
    visuals.hyperlink_color = TEAL;
    visuals.error_fg_color = ERROR;
    visuals.window_corner_radius = RADIUS.into();
    for widget in [
        &mut visuals.widgets.inactive,
        &mut visuals.widgets.hovered,
        &mut visuals.widgets.active,
        &mut visuals.widgets.open,
        &mut visuals.widgets.noninteractive,
    ] {
        widget.corner_radius = 12.into();
        widget.bg_fill = CREAM;
        widget.weak_bg_fill = CREAM;
        widget.bg_stroke = Stroke::new(1.0, BORDER);
        widget.fg_stroke = Stroke::new(1.0, INK);
        widget.expansion = 0.0;
    }
    visuals.widgets.active.bg_fill = CARD;
    visuals.widgets.active.weak_bg_fill = CARD;
    visuals.widgets.active.bg_stroke = Stroke::new(1.5, INK);
    ctx.set_visuals(visuals);
    ctx.global_style_mut(|style| {
        style.wrap_mode = Some(egui::TextWrapMode::Wrap);
        style.spacing.item_spacing = egui::vec2(12.0, 8.0);
        style.spacing.button_padding = egui::vec2(16.0, 10.0);
        style.spacing.interact_size = egui::vec2(44.0, 40.0);
        for (style_name, size) in [
            (TextStyle::Body, 15.0),
            (TextStyle::Button, 14.0),
            (TextStyle::Small, 12.0),
            (TextStyle::Heading, 30.0),
        ] {
            style
                .text_styles
                .insert(style_name, FontId::proportional(size));
        }
        style.text_styles.insert(TextStyle::Heading, display(30.0));
        style.text_styles.insert(TextStyle::Button, display(14.0));
        style
            .text_styles
            .insert(TextStyle::Monospace, FontId::monospace(15.0));
    });
    ctx.send_viewport_cmd(egui::ViewportCommand::SetTheme(egui::SystemTheme::Light));
}
pub fn icon() -> egui::IconData {
    eframe::icon_data::from_png_bytes(include_bytes!("../../assets/icons/telegram-my-id-128.png"))
        .expect("bundled icon must decode")
}
