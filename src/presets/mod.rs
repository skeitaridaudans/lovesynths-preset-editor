mod lsi_parser;

use crate::components::preset_image::PresetImage;
use crate::presets::lsi_parser::parse_lsi_image;
use crate::types::PresetType::{System, Timbre};
use crate::types::{
    ImageData, LoadedPresetEntry, PointF, PresetData, PresetEntry, PresetType, Side,
};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::io::Read;
use std::path::Path;

const SYNTH_IMAGE_COLUMNS: i32 = 6;

pub fn load_preset_list(path: impl AsRef<Path>) -> anyhow::Result<HashMap<i32, PresetEntry>> {
    let preset_list: Vec<PresetEntry> = serde_json::from_str(&fs::read_to_string(path)?)?;
    let preset_map = preset_list.into_iter().map(|p| (p.index, p)).collect();

    Ok(preset_map)
}

pub fn load_preset_image(path: impl AsRef<Path>) -> anyhow::Result<Vec<Vec<PointF>>> {
    let data = fs::read(path)?;
    parse_lsi_image(data.as_slice())
}

pub fn load_presets(path: &str, preset_type: PresetType, side: Side) -> anyhow::Result<PresetData> {
    use PresetType::*;
    let preset_list_file = match preset_type {
        Timbre => "preset_list.json",
        System => "system_preset_list.json",
    };
    let preset_list = load_preset_list(&format!("{path}/{preset_list_file}"))?;
    let loaded_preset_list: anyhow::Result<HashMap<i32, LoadedPresetEntry>> = preset_list
        .into_iter()
        .map(|(index, p)| {
            let image_data = load_preset_image(format!("{path}/{}", &p.image_filename))?;
            Ok((
                index,
                LoadedPresetEntry::new(
                    p.index,
                    side,
                    p.image_filename,
                    p.preset_filename.clone(),
                    image_data,
                    fs::read_to_string(format!("{path}/{}", p.preset_filename))?,
                ),
            ))
        })
        .collect();
    let preset_name = Path::new(path)
        .file_name()
        .map(|n| n.to_str())
        .flatten()
        .unwrap_or("Untitled");

    Ok(PresetData {
        path: path.to_string(),
        preset_type,
        presets: loaded_preset_list?,
        name: preset_name.to_string()
    })
}

fn preset_filename_location(index: i32) -> String {
    let row = index / SYNTH_IMAGE_COLUMNS;
    let column = index % SYNTH_IMAGE_COLUMNS;

    format!("{row}_{column}")
}

fn preset_image_filename(index: i32, preset_type: PresetType) -> String {
    use PresetType::*;
    let prefix = match preset_type {
        Timbre => "image_timbre",
        System => "image_system",
    };
    let suffix = preset_filename_location(index);

    format!("{prefix}{suffix}.lsi")
}

fn preset_filename(index: i32, preset_type: PresetType) -> String {
    use PresetType::*;
    let prefix = match preset_type {
        Timbre => "settings_timbre",
        System => "settings_system",
    };
    let suffix = preset_filename_location(index);

    format!("{prefix}{suffix}.json")
}

fn save_image(path: impl AsRef<Path>, image: &ImageData) -> anyhow::Result<()> {
    let image_data = image
        .into_iter()
        .flat_map(|line| {
            let line_bytes = line
                .iter()
                .flat_map(|p| [p.x.to_le_bytes(), p.y.to_le_bytes()].concat())
                .collect_vec();

            [(line.len() as i32).to_le_bytes().to_vec(), line_bytes].concat()
        })
        .collect_vec();

    fs::write(path, image_data)?;
    Ok(())
}

pub fn save_presets(
    path: &str,
    presets: &HashMap<i32, LoadedPresetEntry>,
    preset_type: PresetType,
) -> anyhow::Result<()> {
    // Clear the directory before creating all files again
    fs::remove_dir_all(path)?;
    fs::create_dir(path)?;

    let presets: HashMap<i32, LoadedPresetEntry> = presets
        .iter()
        .map(|(i, p)| {
            (
                *i,
                LoadedPresetEntry {
                    preset_filename: preset_filename(*i, preset_type),
                    image_filename: preset_image_filename(*i, preset_type),
                    ..p.clone()
                },
            )
        })
        .collect();

    for (_, entry) in &presets {
        save_image(format!("{path}/{}", &entry.image_filename), &entry.image)?;
        fs::write(
            format!("{path}/{}", &entry.preset_filename),
            &entry.preset_data,
        )?;
    }

    let preset_list_filename = match preset_type {
        Timbre => "preset_list.json",
        System => "system_preset_list.json",
    };
    let preset_list: Vec<PresetEntry> = presets
        .into_iter()
        .map(|(i, p)| PresetEntry {
            index: i,
            preset_filename: p.preset_filename,
            image_filename: p.image_filename,
        })
        .collect();
    let preset_json = serde_json::to_string(&preset_list)?;
    fs::write(format!("{path}/{preset_list_filename}"), preset_json)?;

    Ok(())
}
