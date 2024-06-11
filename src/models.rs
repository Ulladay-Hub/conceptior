// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Menu {
    pub title: String,
    pub items: Vec<MenuItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuItem {
    pub name: String,
    pub action: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub menus: Vec<Menu>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetPackage {
    pub name: String,
    pub version: String,
    pub menus: Vec<Menu>,
    pub windows: Vec<Window>,
}
