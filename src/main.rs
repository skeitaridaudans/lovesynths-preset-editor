#![windows_subsystem = "windows"]

pub mod colors;
pub mod components;
mod fonts;
mod presets;
pub mod styles;
mod types;
mod utils;

use crate::components::preset_list::{preset_container, preset_list};
use crate::fonts::load_fonts;
use crate::presets::{load_presets, save_presets};
use crate::styles::button_style::GeneralButtonStyle;
use crate::types::{LoadedPresetEntry, PresetData, PresetEntry, PresetType, Side};
use iced::alignment::Horizontal;
use iced::theme::{Button, Palette, Text};
use iced::widget::{button, canvas, column, container, row, text, Canvas, Space};
use iced::{
    executor, window, Alignment, Application, Color, Command, Element, Length, Settings, Size,
    Theme,
};
use nom::Parser;
use rfd::FileDialog;
use std::collections::HashMap;
use std::fs;
use tap::Pipe;
use crate::colors::{BACKGROUND_COLOR, PRIMARY_COLOR};

fn main() {
    MainWindow::run(Settings {
        window: window::Settings {
            size: Size::new(880.0, 600.0),
            min_size: Some(Size::new(880.0, 400.0)),
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        ..Settings::default()
    })
    .unwrap();
}

#[derive(Debug, Clone)]
enum BottomMessage {
    Success(String),
    Error(String),
    None,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    FontsLoaded,
    ClickPreset(i32, Side),
    Save(Side),
    Close(Side),
    LoadPreset(Side, PresetType),
}

struct MainWindow {
    preset_lists: HashMap<Side, PresetData>,
    selected: Option<(i32, Side)>,
    bottom_message: BottomMessage,
}

impl Application for MainWindow {
    type Executor = executor::Default;
    type Flags = ();
    type Message = AppMessage;
    type Theme = Theme;

    fn new(_flags: ()) -> (MainWindow, Command<Self::Message>) {
        (
            MainWindow {
                preset_lists: HashMap::new(),
                selected: None,
                bottom_message: BottomMessage::None,
            },
            load_fonts().map(|_| AppMessage::FontsLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("Preset manager")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        self.bottom_message = BottomMessage::None;

        match _message {
            AppMessage::FontsLoaded => Command::none(),
            AppMessage::ClickPreset(i, side) => {
                match self.selected {
                    Some((from, from_side)) => {
                        let to_side = side;
                        if !self.preset_lists.contains_key(&from_side)
                            || !self.preset_lists.contains_key(&to_side)
                        {
                            self.show_error("Either side is not loaded, cannot proceed with move");
                            return Command::none();
                        }

                        let from_preset = self
                            .preset_lists
                            .get_mut(&from_side)
                            .unwrap()
                            .presets
                            .remove(&from);
                        let to_preset = self
                            .preset_lists
                            .get_mut(&to_side)
                            .unwrap()
                            .presets
                            .remove(&i);

                        match from_preset {
                            Some(e) => {
                                self.preset_lists
                                    .get_mut(&to_side)
                                    .unwrap()
                                    .presets
                                    .insert(i, e);
                            }
                            None => {
                                self.show_error("Selected item does not exist, cannot move");
                            }
                        }

                        // Swap the items if there is a preset in both slots
                        match to_preset {
                            Some(e) => {
                                self.preset_lists
                                    .get_mut(&from_side)
                                    .unwrap()
                                    .presets
                                    .insert(from, e);
                            }
                            None => (),
                        }

                        self.selected = None;
                    }
                    None if !self.preset_lists.contains_key(&side) => {
                        self.show_error("Side not loaded, cannot select preset");
                    }
                    None if self.preset_lists[&side].presets.contains_key(&i) => {
                        self.selected = Some((i, side));
                    }
                    None => (),
                }
                Command::none()
            }
            AppMessage::Save(side) => {
                let Some(preset_data) = self.preset_lists.get(&side) else {
                    self.show_error("Cannot save side, side not loaded");
                    return Command::none();
                };
                match save_presets(
                    &preset_data.path,
                    &preset_data.presets,
                    preset_data.preset_type,
                ) {
                    Ok(()) => self.show_success(&format!("Preset {} saved!", &preset_data.name)),
                    Err(e) => {
                        self.show_error(&format!("Failed to save preset. {}", e.to_string()));
                    }
                }
                Command::none()
            }
            AppMessage::Close(side) => {
                self.preset_lists.remove(&side);
                Command::none()
            }
            AppMessage::LoadPreset(side, preset_type) => {
                let Some(preset_path) = FileDialog::new().pick_folder() else {
                    return Command::none();
                };
                if self
                    .preset_lists
                    .get(&side.other())
                    .map(|s| s.preset_type != preset_type)
                    .unwrap_or(false)
                {
                    self.show_error("Both sides have the same type of preset (timbre or system)");
                    return Command::none();
                }

                match load_presets(preset_path.to_str().unwrap(), preset_type, side) {
                    Ok(preset_data) => {
                        self.preset_lists.insert(side, preset_data);
                    }
                    Err(e) => {
                        self.show_error(&format!("Cannot load preset. {}", e.to_string()));
                    }
                }

                Command::none()
            }
        }
    }

    fn theme(&self) -> Self::Theme {
        Theme::custom(
            "Main theme".to_string(),
            Palette {
                background: BACKGROUND_COLOR,
                text: Color::WHITE,
                primary: PRIMARY_COLOR,
                success: Color::from_rgb(0.13, 0.77, 0.37),
                danger: Color::from_rgb(0.94, 0.27, 0.27),
            },
        )
    }

    fn view(&self) -> Element<Self::Message> {
        let green = Color::new(0.0, 1.0, 0.0, 1.0);
        let red = Color::new(1.0, 0.0, 0.0, 1.0);

        use BottomMessage::*;
        column([
            row([
                self.preset_list_view(Side::Left),
                self.preset_list_view(Side::Right),
            ])
            .height(Length::Fill)
            .into(),
            container(match &self.bottom_message {
                Success(msg) => text(msg).style(Text::Color(self.theme().palette().success)),
                Error(msg) => text(msg).style(Text::Color(self.theme().palette().danger)),
                None => text(""),
            })
            .padding([10, 0])
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .into(),
        ])
        .into()
    }
}

impl MainWindow {
    fn preset_list_view(&self, side: Side) -> Element<AppMessage> {
        self.preset_lists.get(&side).map_or_else(
            || self.preset_not_loaded_view(side),
            |p| preset_container(&p.name, &p.presets, self.selected, side),
        )
    }

    fn preset_not_loaded_view(&self, side: Side) -> Element<AppMessage> {
        column([
            text("No preset loaded").into(),
            Space::with_height(10).into(),
            button("Load timbre preset")
                .style(Button::Custom(Box::new(GeneralButtonStyle::new())))
                .padding([8, 12])
                .on_press(AppMessage::LoadPreset(side, PresetType::Timbre))
                .into(),
            button("Load system preset")
                .style(Button::Custom(Box::new(GeneralButtonStyle::new())))
                .padding([8, 12])
                .on_press(AppMessage::LoadPreset(side, PresetType::System))
                .into(),
        ])
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([24, 0])
        .align_items(Alignment::Center)
        .into()
    }

    fn show_error(&mut self, message: &str) {
        self.bottom_message = BottomMessage::Error(message.to_string())
    }

    fn show_success(&mut self, message: &str) {
        self.bottom_message = BottomMessage::Success(message.to_string())
    }
}
