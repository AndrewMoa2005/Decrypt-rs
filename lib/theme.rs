// lib/theme.rs
use fltk_theme::{ColorTheme, color_themes};
// use fltk_theme::{WidgetScheme, SchemeType,};

pub fn using_theme() {
    let theme = ColorTheme::new(&color_themes::fleet::LIGHT);
    theme.apply();
    /*
    let widget_scheme = WidgetScheme::new(SchemeType::Fleet1);
    widget_scheme.apply();
    */
}