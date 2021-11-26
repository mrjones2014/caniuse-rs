use serde::Serialize;

use crate::feature::Feature;

#[derive(Serialize)]
pub struct AlfredItemList {
    pub items: Vec<AlfredItem>,
}

#[derive(Serialize)]
pub struct AlfredItem {
    pub title: String,
    pub subtitle: String,
    pub arg: String,
    pub r#match: String,
}

impl From<Feature> for AlfredItem {
    fn from(feature: Feature) -> Self {
        let match_str = feature.string_for_matching();
        AlfredItem {
            title: feature.title,
            subtitle: feature.description,
            arg: feature.url,
            r#match: match_str,
        }
    }
}

pub fn get_json(
    features: &[Feature],
    query: &str,
    pretty: &bool,
) -> Result<String, serde_json::Error> {
    let alfred_items = AlfredItemList {
        items: features
            .iter()
            .filter(|feature| {
                let match_str = feature.string_for_matching().to_lowercase();
                match_str.contains(&query) || query.contains(&match_str)
            })
            .map(|feature| AlfredItem::from(feature.to_owned()))
            .collect(),
    };

    match pretty {
        true => serde_json::to_string_pretty(&alfred_items),
        false => serde_json::to_string(&alfred_items),
    }
}
