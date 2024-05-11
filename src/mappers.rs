use std::collections::HashMap;

use crate::types::{
    AlfredConfig, AlfredConfigItem, AlfredMod, Bookmark, BookmarksGroup, Service, ServicesGroup,
};

pub fn map_home_page_groups_to_alfred_config(
    groups: Vec<impl MapGroupToAlfredConfigItems>,
) -> Vec<AlfredConfigItem> {
    groups
        .iter()
        .flat_map(|group| group.map_to_alfred_config_items())
        .collect()
}

pub fn map_error_to_alfred_config(error: &str) -> AlfredConfig {
    AlfredConfig {
        items: vec![AlfredConfigItem {
            title: "Error".to_string(),
            subtitle: Some(format!("Error: {}", error)),
            arg: None,
            r#match: None,
            quicklookurl: None,
            mods: None,
        }],
        cache: None,
    }
}

impl MapGroupItemToAlfredConfigItem for Bookmark {
    fn map_to_alfred_config(&self, group_name: &str) -> AlfredConfigItem {
        let subtitle = if let Some(description) = &self.description {
            format!("{description} [{group_name}]")
        } else {
            group_name.to_string()
        };

        AlfredConfigItem {
            title: self.name.to_string(),
            subtitle: Some(subtitle.to_string()),
            arg: Some(self.href.to_string()),
            r#match: Some([self.name.to_string(), subtitle.to_string()].join(" ")),
            quicklookurl: Some(self.href.to_string()),
            mods: None,
        }
    }
}

pub trait MapGroupItemToAlfredConfigItem {
    fn map_to_alfred_config(&self, group: &str) -> AlfredConfigItem;
}

pub trait MapGroupToAlfredConfigItems {
    fn map_to_alfred_config_items(&self) -> Vec<AlfredConfigItem>;
}

impl MapGroupItemToAlfredConfigItem for Service {
    fn map_to_alfred_config(&self, group_name: &str) -> AlfredConfigItem {
        let subtitle = if let Some(description) = &self.description {
            format!("{description} [{group_name}]")
        } else {
            group_name.to_string()
        };

        let quicklookurl = self.href.to_string();

        AlfredConfigItem {
            title: self.name.to_string(),
            subtitle: Some(subtitle.to_string()),
            arg: Some(self.href.to_string()),
            r#match: Some([self.name.to_string(), subtitle.to_string()].join(" ")),
            quicklookurl: Some(quicklookurl),
            mods: Some(HashMap::from([(
                "cmd".to_string(),
                AlfredMod {
                    arg: Some(self.href.to_string()),
                    subtitle: Some(self.href.to_string()),
                    valid: Some(true),
                },
            )])),
        }
    }
}

impl MapGroupToAlfredConfigItems for ServicesGroup {
    fn map_to_alfred_config_items(&self) -> Vec<AlfredConfigItem> {
        self.services
            .iter()
            .map(|service| service.map_to_alfred_config(&self.name))
            .collect()
    }
}

impl MapGroupToAlfredConfigItems for BookmarksGroup {
    fn map_to_alfred_config_items(&self) -> Vec<AlfredConfigItem> {
        self.bookmarks
            .iter()
            .map(|service| service.map_to_alfred_config(&self.name))
            .collect()
    }
}
