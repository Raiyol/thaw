use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct TagTheme {
    pub default_font_color: String,
    pub default_background_color: String,
    pub default_border_color: String,
    pub success_background_color: String,
    pub success_border_color: String,
    pub warning_background_color: String,
    pub warning_border_color: String,
    pub error_background_color: String,
    pub error_border_color: String,
}

impl ThemeMethod for TagTheme {
    fn light() -> Self {
        Self {
            default_font_color: "#333639".into(),
            default_background_color: "#fafafc".into(),
            default_border_color: " #e0e0e6".into(),
            success_background_color: "#edf7f2".into(),
            success_border_color: "#c5e7d5".into(),
            warning_background_color: "#fef7ed".into(),
            warning_border_color: "#fae0b5".into(),
            error_background_color: "#fbeef1".into(),
            error_border_color: "#f3cbd3".into(),
        }
    }

    fn dark() -> Self {
        Self {
            default_font_color: "#333639".into(),
            default_background_color: "#fafafc".into(),
            default_border_color: " #e0e0e6".into(),
            success_background_color: "#edf7f2".into(),
            success_border_color: "#c5e7d5".into(),
            warning_background_color: "#fef7ed".into(),
            warning_border_color: "#fae0b5".into(),
            error_background_color: "#fbeef1".into(),
            error_border_color: "#f3cbd3".into(),
        }
    }
}
