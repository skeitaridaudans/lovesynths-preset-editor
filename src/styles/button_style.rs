use iced::{Background, Border, Color, Theme};
use iced::border::Radius;
use iced::widget::button;
use iced::widget::button::Appearance;

pub struct GeneralButtonStyle {}

impl GeneralButtonStyle {
    pub fn new() -> GeneralButtonStyle {
        GeneralButtonStyle {}
    }
}

impl button::StyleSheet for GeneralButtonStyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Default::default(),
            background: Some(Background::Color([0.11, 0.31, 0.85].into())),
            border: Border {
                radius: Radius::from(8.0),
                width: 1.0,
                color: [0.12, 0.25, 0.69].into(),
            },
            text_color: Color::from_rgb8(209, 213, 219),
            shadow: Default::default(),
        }
    }
}