use crate::types::{ImageData, LoadedPresetEntry, Side};
use crate::styles::preset_button_style::PresetButtonStyle;
use crate::utils::make_display_point;
use crate::AppMessage;
use iced::mouse::Cursor;
use iced::theme::Button;
use iced::widget::canvas::{Frame, Geometry, Path, Program, Stroke};
use iced::widget::{button, container, Canvas, Component};
use iced::{Alignment, Color, Element, Length, Point, Rectangle, Renderer, Theme};
use nom::Parser;
use crate::colors::PRESET_COLOR;

struct PresetImage {
    image: ImageData,
}

impl<Message> Program<Message> for PresetImage {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let path = Path::new(|builder| {
            for line in &self.image {
                let Some(first) = line.iter().next() else {
                    continue;
                };

                builder.move_to(make_display_point(first, frame.width()));
                for point in line.iter().skip(1) {
                    builder.line_to(make_display_point(point, frame.width()));
                }
            }
        });

        frame.stroke(&path, Stroke::default().with_color(PRESET_COLOR));
        vec![frame.into_geometry()]
    }
}

pub fn preset_image(index: i32, entry: &LoadedPresetEntry, selected: Option<(i32, Side)>, side: Side) -> Element<'static, AppMessage> {
    let style = if matches!(selected, Some((i, s)) if i == index && s == side) {
        PresetButtonStyle::selected()
    }
    else {
        PresetButtonStyle::new()
    };

    button(Canvas::new(PresetImage {
        image: entry.image.clone(),
    }))
    .on_press(AppMessage::ClickPreset(index, side))
    .width(60)
    .height(60)
    .style(Button::Custom(Box::new(style)))
    .into()
}

pub fn empty_preset_image(index: i32, side: Side) -> Element<'static, AppMessage> {
    button("")
        .on_press(AppMessage::ClickPreset(index, side))
        .width(60)
        .height(60)
        .style(Button::Custom(Box::new(PresetButtonStyle::new())))
        .into()
}
