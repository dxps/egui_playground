use egui::{Color32, CornerRadius, FontFamily, FontId, Stroke, Style, TextStyle, Theme, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppTheme {
    Dark,
    Light,
}

impl AppTheme {
    pub fn egui_theme(self) -> Theme {
        match self {
            Self::Dark => Theme::Dark,
            Self::Light => Theme::Light,
        }
    }

    pub fn toggle(&mut self) {
        *self = match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        };
    }
}

pub fn install_theme(ctx: &egui::Context, initial_theme: AppTheme) {
    ctx.set_style_of(Theme::Dark, dark_style());
    ctx.set_style_of(Theme::Light, light_style());
    ctx.set_theme(initial_theme.egui_theme());
}

pub fn set_theme(ctx: &egui::Context, theme: AppTheme) {
    ctx.set_theme(theme.egui_theme());
}

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb(
        ((hex >> 16) & 0xff) as u8,
        ((hex >> 8) & 0xff) as u8,
        (hex & 0xff) as u8,
    )
}

mod gruvbox {
    use super::{Color32, rgb};

    // Dark neutrals
    pub const DARK0_HARD: Color32 = rgb(0x1d2021);
    pub const DARK0: Color32 = rgb(0x282828);
    pub const DARK0_SOFT: Color32 = rgb(0x32302f);
    pub const DARK1: Color32 = rgb(0x3c3836);
    pub const DARK2: Color32 = rgb(0x504945);
    pub const DARK3: Color32 = rgb(0x665c54);
    pub const DARK4: Color32 = rgb(0x7c6f64);

    // Light neutrals
    pub const LIGHT0_HARD: Color32 = rgb(0xf9f5d7);
    pub const LIGHT0: Color32 = rgb(0xfbf1c7);
    pub const LIGHT0_SOFT: Color32 = rgb(0xf2e5bc);
    pub const LIGHT1: Color32 = rgb(0xebdbb2);
    pub const LIGHT2: Color32 = rgb(0xd5c4a1);
    pub const LIGHT3: Color32 = rgb(0xbdae93);
    pub const LIGHT4: Color32 = rgb(0xa89984);

    // Neutral gray
    pub const GRAY: Color32 = rgb(0x928374);

    // Bright accents
    pub const BRIGHT_RED: Color32 = rgb(0xfb4934);
    pub const BRIGHT_GREEN: Color32 = rgb(0xb8bb26);
    pub const BRIGHT_YELLOW: Color32 = rgb(0xfabd2f);
    pub const BRIGHT_BLUE: Color32 = rgb(0x83a598);
    pub const BRIGHT_PURPLE: Color32 = rgb(0xd3869b);
    pub const BRIGHT_AQUA: Color32 = rgb(0x8ec07c);
    pub const BRIGHT_ORANGE: Color32 = rgb(0xfe8019);

    // Muted accents
    pub const MUTED_RED: Color32 = rgb(0xcc241d);
    pub const MUTED_GREEN: Color32 = rgb(0x98971a);
    pub const MUTED_YELLOW: Color32 = rgb(0xd79921);
    pub const MUTED_BLUE: Color32 = rgb(0x458588);
    pub const MUTED_PURPLE: Color32 = rgb(0xb16286);
    pub const MUTED_AQUA: Color32 = rgb(0x689d6a);
    pub const MUTED_ORANGE: Color32 = rgb(0xd65d0e);
}

fn dark_style() -> Style {
    use gruvbox::*;

    let mut visuals = Visuals::dark();

    let bg = DARK0;
    let surface = DARK0_SOFT;
    let surface_2 = DARK1;
    let border = DARK3;
    let text = LIGHT1;
    let weak_text = LIGHT4;
    let accent = BRIGHT_BLUE;
    let accent_hover = BRIGHT_AQUA;

    visuals.panel_fill = bg;
    visuals.window_fill = surface;
    visuals.window_stroke = Stroke::new(1.0, border);
    visuals.faint_bg_color = DARK0_HARD;
    visuals.extreme_bg_color = DARK0_HARD;
    visuals.code_bg_color = DARK1;
    visuals.hyperlink_color = BRIGHT_BLUE;
    visuals.warn_fg_color = BRIGHT_YELLOW;
    visuals.error_fg_color = BRIGHT_RED;
    visuals.selection.bg_fill = DARK3;
    visuals.selection.stroke = Stroke::new(1.0, BRIGHT_BLUE);
    visuals.weak_text_color = Some(weak_text);

    tune_widgets(
        &mut visuals,
        surface,
        surface_2,
        border,
        text,
        accent,
        accent_hover,
    );

    base_style(visuals)
}

fn light_style() -> Style {
    use gruvbox::*;

    let mut visuals = Visuals::light();

    let bg = LIGHT0_SOFT;
    let surface = LIGHT0;
    let surface_2 = LIGHT1;
    let border = LIGHT3;
    let text = DARK1;
    let weak_text = DARK4;
    let accent = MUTED_BLUE;
    let accent_hover = MUTED_AQUA;

    visuals.panel_fill = bg;
    visuals.window_fill = surface;
    visuals.window_stroke = Stroke::new(1.0, border);
    visuals.faint_bg_color = LIGHT1;
    visuals.extreme_bg_color = LIGHT0;
    visuals.code_bg_color = LIGHT1;
    visuals.hyperlink_color = MUTED_BLUE;
    visuals.warn_fg_color = MUTED_YELLOW;
    visuals.error_fg_color = MUTED_RED;
    visuals.selection.bg_fill = LIGHT2;
    visuals.selection.stroke = Stroke::new(1.0, MUTED_BLUE);
    visuals.weak_text_color = Some(weak_text);

    tune_widgets(
        &mut visuals,
        surface,
        surface_2,
        border,
        text,
        accent,
        accent_hover,
    );

    base_style(visuals)
}

fn tune_widgets(
    visuals: &mut Visuals,
    surface: Color32,
    surface_2: Color32,
    border: Color32,
    text: Color32,
    accent: Color32,
    accent_hover: Color32,
) {
    let radius = CornerRadius::same(6);

    visuals.widgets.noninteractive.bg_fill = surface;
    visuals.widgets.noninteractive.weak_bg_fill = surface;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, border);
    visuals.widgets.noninteractive.corner_radius = radius;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, text);

    visuals.widgets.inactive.bg_fill = surface;
    visuals.widgets.inactive.weak_bg_fill = surface_2;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, border);
    visuals.widgets.inactive.corner_radius = radius;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, text);

    visuals.widgets.hovered.bg_fill = surface_2;
    visuals.widgets.hovered.weak_bg_fill = surface_2;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, accent_hover);
    visuals.widgets.hovered.corner_radius = radius;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, text);
    visuals.widgets.hovered.expansion = 1.0;

    visuals.widgets.active.bg_fill = accent;
    visuals.widgets.active.weak_bg_fill = accent;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, accent);
    visuals.widgets.active.corner_radius = radius;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);

    visuals.widgets.open = visuals.widgets.hovered;

    visuals.window_corner_radius = CornerRadius::same(10);
    visuals.menu_corner_radius = CornerRadius::same(8);
    visuals.button_frame = true;
    visuals.collapsing_header_frame = false;
    visuals.striped = true;
    visuals.slider_trailing_fill = true;
}

fn base_style(visuals: Visuals) -> Style {
    let mut style = Style::default();

    style.visuals = visuals;

    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(10.0, 6.0);
    style.spacing.menu_margin = egui::Margin::same(8);
    style.spacing.window_margin = egui::Margin::same(12);
    style.spacing.interact_size.y = 28.0;

    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(20.0, FontFamily::Proportional),
    );
    style
        .text_styles
        .insert(TextStyle::Body, FontId::new(14.0, FontFamily::Proportional));
    style.text_styles.insert(
        TextStyle::Button,
        FontId::new(14.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Small,
        FontId::new(11.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Monospace,
        FontId::new(13.0, FontFamily::Monospace),
    );

    style
}
