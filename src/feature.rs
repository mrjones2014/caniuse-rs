use crate::browser::Browser;
use std::fmt::Display;

pub struct Feature {
    pub name: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub stats: [Browser; 4],
}

impl Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stats_string = self
            .stats
            .iter()
            .map(|stat| format!("{}", stat))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{} [{}]", self.title, stats_string)
    }
}

pub enum Error {
    JsonParse(serde_json::error::Error),
    NotAnObject(serde_json::Value),
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        Error::JsonParse(e)
    }
}

pub fn json_to_features(json: String) -> Result<Vec<Feature>, Error> {
    let parsed = serde_json::from_str(&json)?;
    let obj = match parsed {
        serde_json::Value::Object(val) => val,
        _ => return Err(Error::NotAnObject(parsed)),
    };

    let mut features = Vec::new();

    for (_, (key, item)) in obj.iter().enumerate() {
        let fields = match item {
            serde_json::Value::Object(entry) => entry,
            _ => return Err(Error::NotAnObject(item.to_owned())),
        };

        let title = format!("{}", fields["title"]);
        let description = format!("{}", fields["title"]);
        let url = format!("https://caniuse.com/#feat={}", key);
        let stats = [
            Browser::IE("".to_string()),
            Browser::IE("".to_string()),
            Browser::IE("".to_string()),
            Browser::IE("".to_string()),
        ];

        features.push(Feature {
            name: String::from(key),
            title,
            description,
            url,
            stats,
        })
    }

    Ok(features)
}
