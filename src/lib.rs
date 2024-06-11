// src/lib.rs
pub mod models;

use models::{Menu, MenuItem, Window, AssetPackage};
use serde_json;
use std::fs::File;
use std::io::{self, Read};

pub fn generate_menu(title: &str, items: Vec<(&str, &str)>) -> Menu {
    Menu {
        title: title.to_string(),
        items: items
            .into_iter()
            .map(|(name, action)| MenuItem {
                name: name.to_string(),
                action: action.to_string(),
            })
            .collect(),
    }
}

pub fn generate_window(title: &str, width: u32, height: u32, menus: Vec<Menu>) -> Window {
    Window {
        title: title.to_string(),
        width,
        height,
        menus,
    }
}

pub fn load_asset_package(path: &str) -> io::Result<AssetPackage> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let package: AssetPackage = serde_json::from_str(&contents)?;
    Ok(package)
}
