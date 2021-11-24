use serde::Serialize;

use crate::feature::Feature;

#[derive(Serialize)]
pub struct AlfredItemList {
    pub items: Vec<AlfredItem>,
}

#[derive(Serialize)]
pub struct AlfredOpenUrlAction {
    pub url: String,
}

#[derive(Serialize)]
pub struct AlfredItem {
    pub title: String,
    pub subtitle: String,
    pub arg: String,
    pub r#match: String,
    pub action: AlfredOpenUrlAction,
}

impl From<Feature> for AlfredItem {
    fn from(feature: Feature) -> Self {
        let match_str = feature.string_for_matching();
        AlfredItem {
            title: feature.title,
            subtitle: feature.description,
            arg: feature.name,
            r#match: match_str,
            action: AlfredOpenUrlAction { url: feature.url },
        }
    }
}
