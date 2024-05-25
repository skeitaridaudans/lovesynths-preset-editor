use iced::Command;
use iced::font::{Family, Font, load, Style};

pub const FONT_AWESOME_BRANDS: Font = Font {
    family: Family::Name("Font Awesome 6 Brands"),
    weight: iced::font::Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: Style::Normal,
};

pub const FONT_AWESOME_BRANDS_DATA: &[u8] = include_bytes!("../fonts/Font Awesome 6 Brands-Regular-400.otf");


pub const FONT_AWESOME_REGULAR: Font = Font {
    family: Family::Name("Font Awesome 6 Free"),
    weight: iced::font::Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: Style::Normal,
};

pub const FONT_AWESOME_REGULAR_DATA: &[u8] = include_bytes!("../fonts/Font Awesome 6 Free-Regular-400.otf");

pub const FONT_AWESOME_SOLID: Font = Font {
    family: Family::Name("Font Awesome 6 Free"),
    weight: iced::font::Weight::Black,
    stretch: iced::font::Stretch::Normal,
    style: Style::Normal,
};

pub const FONT_AWESOME_SOLID_DATA: &[u8] = include_bytes!("../fonts/Font Awesome 6 Free-Solid-900.otf");

pub fn load_fonts() -> Command<Result<(), iced::font::Error>> {
    Command::batch(vec![
        load(FONT_AWESOME_BRANDS_DATA),
        load(FONT_AWESOME_REGULAR_DATA),
        load(FONT_AWESOME_SOLID_DATA),
    ])
}