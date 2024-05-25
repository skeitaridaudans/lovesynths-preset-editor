use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub type ImageData = Vec<Vec<PointF>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PresetEntry {
    pub index: i32,
    pub image_filename: String,
    pub preset_filename: String,
}


#[derive(Debug, Clone)]
pub struct PresetData {
    pub path: String,
    pub preset_type: PresetType,
    pub presets: HashMap<i32, LoadedPresetEntry>,
    pub name: String
}

#[derive(Debug, Clone)]
pub struct LoadedPresetEntry {
    pub original_index: i32,
    pub original_side: Side,
    pub image_filename: String,
    pub preset_filename: String,
    pub image: ImageData,
    pub preset_data: String
}

impl LoadedPresetEntry {
    pub fn new(
        original_index: i32,
        original_side: Side,
        image_filename: String,
        preset_filename: String,
        image: ImageData,
        preset_data: String
    ) -> Self {
        Self {
            original_index,
            original_side,
            image_filename,
            preset_filename,
            image,
            preset_data
        }
    }
}

#[derive(Debug, Clone)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

impl PointF {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for PointF {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Side {
    Left,
    Right
}

impl Side {
    pub fn other(&self) -> Self {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PresetType {
    Timbre,
    System
}
