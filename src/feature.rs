use crate::browser::Browser;
use std::fmt::Display;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Feature {
    pub name: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub stats: [Option<Browser>; 4],
}

impl Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stats_string = self
            .stats
            .iter()
            .filter(|stat| stat.is_some())
            .map(|stat| format!("{}", stat.as_ref().unwrap()))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{} [{}]", self.title, stats_string)
    }
}

#[derive(Debug)]
pub enum Error {
    JsonParse(serde_json::error::Error),
    NotAnObject(serde_json::Value),
    VersionSupportNotAString(serde_json::Value),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::JsonParse(e) => write!(f, "Error parsing JSON: {}", e),
            Error::NotAnObject(e) => write!(f, "Expected an object but got not an object: {}", e),
            Error::VersionSupportNotAString(e) => write!(
                f,
                "Expected strings for browser version support, but isn't a string: {}",
                e
            ),
        }
    }
}

impl std::error::Error for Error {}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        Error::JsonParse(e)
    }
}

fn parse_browser_version(value: &serde_json::Value) -> Result<String, Error> {
    let fields = match value {
        serde_json::Value::Object(obj) => obj,
        _ => return Err(Error::NotAnObject(value.to_owned())),
    };

    let mut min_supported_version = 0.0;
    let mut final_support_str: Option<String> = None;
    for (_, (version_key, support_str_val)) in fields.iter().enumerate() {
        let version_key_as_float = version_key.parse::<f64>().unwrap_or(0.0);
        let support_str = match support_str_val {
            serde_json::Value::String(str_val) => str_val,
            _ => return Err(Error::VersionSupportNotAString(support_str_val.to_owned())),
        };

        if min_supported_version < version_key_as_float && support_str == "y" {
            final_support_str = Some(format!("{}+", version_key));
            continue;
        }

        if min_supported_version > version_key_as_float {
            continue;
        }

        if support_str.contains("y x") {
            min_supported_version = version_key_as_float;
            final_support_str = Some(format!("{} w/prefix", version_key));
        } else if support_str.contains("a") {
            min_supported_version = version_key_as_float;
            final_support_str = Some(format!("{} (partial)", version_key))
        } else if support_str.contains("p") {
            min_supported_version = version_key_as_float;
            final_support_str = Some(format!("{} w/polyfill", version_key))
        }
    }

    Ok(if final_support_str.is_some() {
        final_support_str.unwrap()
    } else {
        String::from("N/A")
    })
}

fn parse_browsers(value: &serde_json::Value) -> Result<[Option<Browser>; 4], Error> {
    let fields = match value {
        serde_json::Value::Object(obj) => obj,
        _ => return Err(Error::NotAnObject(value.to_owned())),
    };

    let mut browsers: [Option<Browser>; 4] = Default::default();
    let mut idx = 0;
    for (_, (key, item)) in fields.iter().enumerate() {
        let browser = match key.as_str() {
            "ie" => Some(Browser::IE(parse_browser_version(item)?)),
            "firefox" => Some(Browser::Firefox(parse_browser_version(item)?)),
            "chrome" => Some(Browser::Chrome(parse_browser_version(item)?)),
            "safari" => Some(Browser::Safari(parse_browser_version(item)?)),
            _ => None,
        };

        if browser.is_some() {
            browsers[idx] = Some(browser.unwrap());
            idx = idx + 1;
        }
    }

    Ok(browsers)
}

/// Convert JSON from the API to a `Vec<Feature>`. Note that this is
/// different JSON from the cached JSON that is stored by this crate.
pub fn json_to_features(json: String) -> Result<Vec<Feature>, Error> {
    let parsed = serde_json::from_str(&json)?;
    let obj = match parsed {
        serde_json::Value::Object(val) => val,
        _ => return Err(Error::NotAnObject(parsed)),
    };

    let features_data = match &obj["data"] {
        serde_json::Value::Object(data) => data,
        _ => return Err(Error::NotAnObject(obj["data"].to_owned())),
    };

    let mut features = Vec::new();

    for (_, (key, item)) in features_data.iter().enumerate() {
        let fields = match item {
            serde_json::Value::Object(entry) => entry,
            _ => return Err(Error::NotAnObject(item.to_owned())),
        };

        let name = String::from(key);
        let title = format!("{}", fields["title"]);
        let description = format!("{}", fields["title"]);
        let url = format!("https://caniuse.com/#feat={}", key);
        let stats = parse_browsers(&fields["stats"])?;

        features.push(Feature {
            name,
            title,
            description,
            url,
            stats,
        })
    }

    Ok(features)
}
