use std::fmt::Display;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Browser {
    IE(String),
    Safari(String),
    Chrome(String),
    Firefox(String),
}

impl Display for Browser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Browser::IE(version_string) => write!(f, "IE:{}", version_string),
            Browser::Safari(version_string) => write!(f, "Safari:{}", version_string),
            Browser::Chrome(version_string) => write!(f, "Chrome:{}", version_string),
            Browser::Firefox(version_string) => write!(f, "FF:{}", version_string),
        }
    }
}
