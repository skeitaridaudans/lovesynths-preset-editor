use crate::components::preset_image::{empty_preset_image, preset_image};
use crate::styles::button_style::GeneralButtonStyle;
use crate::types::{LoadedPresetEntry, Side};
use crate::AppMessage;
use iced::theme::Button;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Application, Element, Font, Length};
use itertools::Itertools;
use nom::Parser;
use std::collections::HashMap;
use std::hash::Hash;

pub fn preset_list(
    presets: &HashMap<i32, LoadedPresetEntry>,
    selected: Option<(i32, Side)>,
    side: Side,
) -> Element<'static, AppMessage> {
    let columns = 6;
    // As many rows as required to show the lowest preset (plus at least 1)
    let rows = presets.keys()
        .max()
        .map(|x| (x / columns) + 2)
        .unwrap_or(1);

    scrollable(
        container(
            column((0..rows).map(|r| {
                row((0..columns).map(|c| {
                    let index = r * columns + c;
                    presets.get(&index).map_or_else(
                        || empty_preset_image(index, side),
                        |p| preset_image(index, p, selected, side),
                    )
                }))
                .spacing(10)
                .into()
            }))
            .spacing(10)
            .align_items(Alignment::Start),
        )
        .center_x()
        .padding([10, 20, 10, 10]),
    )
    .height(Length::Fill)
    .width(Length::Fill)
    .into()
}

pub fn preset_container(
    name: &str,
    presets: &HashMap<i32, LoadedPresetEntry>,
    selected: Option<(i32, Side)>,
    side: Side,
) -> Element<'static, AppMessage> {
    column([
        row([
            text(name).size(22).width(Length::Fill).into(),
            button("Save")
                .style(Button::Custom(Box::new(GeneralButtonStyle::new())))
                .padding([8, 12])
                .on_press(AppMessage::Save(side))
                .into(),
            button("Close")
                .style(Button::Custom(Box::new(GeneralButtonStyle::new())))
                .padding([8, 12])
                .on_press(AppMessage::Close(side))
                .into(),
        ])
        .width(Length::Fill)
        .spacing(10)
        .padding([12, 16])
        .align_items(Alignment::Center)
        .into(),
        preset_list(presets, selected, side),
    ])
    .height(Length::Fill)
    .width(Length::Fill)
    .into()
}
