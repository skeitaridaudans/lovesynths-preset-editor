use crate::colors::{preset_color, preset_selected_color};
use iced::border::Radius;
use iced::widget::button;
use iced::widget::button::Appearance;
use iced::{Border, Color, Theme};

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
                    preset_selected_color()
                } else {
                    preset_color()
                },
            },
            text_color: Color::WHITE,
            shadow: Default::default(),
        }
    }
}
