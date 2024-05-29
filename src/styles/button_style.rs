use iced::{Background, Border, Color, Theme};
use iced::border::Radius;
use iced::widget::button;
use iced::widget::button::Appearance;
use crate::colors::{PRIMARY_COLOR, PRIMARY_HOVER_COLOR};

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
            background: Some(Background::Color(PRIMARY_COLOR)),
            border: Border {
                radius: Radius::from(4.0),
                width: 1.0,
                color: PRIMARY_COLOR,
            },
            text_color: Color::from_rgb8(209, 213, 219),
            shadow: Default::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        Appearance {
            background: Some(Background::Color(PRIMARY_HOVER_COLOR)),
            border: Border {
                radius: Radius::from(4.0),
                width: 1.0,
                color: PRIMARY_HOVER_COLOR,
            },
            ..self.active(style)
        }
    }
}