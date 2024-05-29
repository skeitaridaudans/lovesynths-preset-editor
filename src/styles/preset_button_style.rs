use crate::colors::{HIGHLIGHT_BACKGROUND_COLOR, PRESET_COLOR, PRESET_HOVER_COLOR, PRESET_SELECTED_COLOR};
use iced::border::Radius;
use iced::widget::button;
use iced::widget::button::Appearance;
use iced::{Background, Border, Color, Theme};

pub struct PresetButtonStyle {
    pub selected: bool,
}

impl PresetButtonStyle {
    pub fn new() -> Self {
        Self { selected: false }
    }
    pub fn selected() -> Self {
        Self { selected: true }
    }
}

impl button::StyleSheet for PresetButtonStyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Default::default(),
            background: None,
            border: Border {
                radius: Radius::from(8.0),
                width: if self.selected { 2.0 } else { 1.0 },
                color: if self.selected {
                    PRESET_SELECTED_COLOR
                } else {
                    PRESET_COLOR
                },
            },
            text_color: Color::WHITE,
            shadow: Default::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        Appearance {
            background: Some(Background::Color(HIGHLIGHT_BACKGROUND_COLOR)),
            border: Border {
                radius: Radius::from(8.0),
                width: 1.0,
                color: PRESET_HOVER_COLOR,
            },
            ..self.active(style)
        }
    }
}
