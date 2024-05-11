use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// HomePage

#[derive(Deserialize, Clone)]
pub struct Service {
    pub name: String,
    pub href: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Bookmark {
    pub name: String,
    pub href: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct ServicesGroup {
    pub name: String,
    pub services: Vec<Service>,
}

#[derive(Deserialize, Clone)]
pub struct BookmarksGroup {
    pub name: String,
    pub bookmarks: Vec<Service>,
}

// Alfred

#[derive(Serialize, Deserialize, Clone)]
pub struct AlfredConfigItem {
    pub title: String,
    pub subtitle: Option<String>,
    pub arg: Option<String>,
    pub r#match: Option<String>,
    pub quicklookurl: Option<String>,
    pub mods: Option<HashMap<String, AlfredMod>>,
}

#[derive(Serialize, Deserialize)]
pub struct AlfredConfig {
    pub items: Vec<AlfredConfigItem>,
    pub cache: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AlfredMod {
    pub valid: Option<bool>,
    pub arg: Option<String>,
    pub subtitle: Option<String>,
}
