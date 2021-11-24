use crate::constants;
use crate::feature::{self, Feature};
use std::fmt::Display;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::{self, Write};
use std::os::unix::prelude::MetadataExt;
use std::path::Path;
use std::time::{SystemTime, SystemTimeError};

const ONE_DAY_IN_SECONDS: u64 = 86400;

#[derive(Debug)]
pub enum ApiError {
    Http(reqwest::Error),
    IO(io::Error),
    SystemTime(SystemTimeError),
    FeatureParsing(feature::Error),
    Serialization(serde_json::Error),
}

impl From<reqwest::Error> for ApiError {
    fn from(e: reqwest::Error) -> Self {
        ApiError::Http(e)
    }
}

impl From<io::Error> for ApiError {
    fn from(e: io::Error) -> Self {
        ApiError::IO(e)
    }
}

impl From<SystemTimeError> for ApiError {
    fn from(e: SystemTimeError) -> Self {
        ApiError::SystemTime(e)
    }
}

impl From<feature::Error> for ApiError {
    fn from(e: feature::Error) -> Self {
        ApiError::FeatureParsing(e)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        ApiError::Serialization(e)
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Http(e) => write!(f, "An error occurred fetching the latest data: {}", e),
            ApiError::IO(e) => write!(
                f,
                "An error occurred writing or reading the cache file: {}",
                e
            ),
            ApiError::SystemTime(e) => {
                write!(f, "An error occurred getting the system time: {}", e)
            }
            ApiError::FeatureParsing(e) => {
                write!(f, "An error occurred parsing the features data: {}", e)
            }
            ApiError::Serialization(e) => {
                write!(f, "An error occurred serializing the features data: {}", e)
            }
        }
    }
}

impl std::error::Error for ApiError {}

fn touch_cache_file() -> io::Result<()> {
    let path = Path::new(&*constants::CACHE_PATH);
    if !path.exists() {
        let directory = path.parent().unwrap();
        fs::create_dir_all(directory)?;
        File::create(path)?;
    }

    Ok(())
}

pub fn get_json_data() -> Result<String, ApiError> {
    let body =
        reqwest::blocking::get("https://github.com/Fyrd/caniuse/raw/main/data.json")?.text()?;
    Ok(body)
}

pub fn ensure_cached_data() -> Result<(), ApiError> {
    touch_cache_file()?;
    let metadata = fs::metadata(&*constants::CACHE_PATH)?;
    let since_last_modified = SystemTime::now()
        .duration_since(metadata.modified()?)?
        .as_secs();

    if metadata.size() == 0 || since_last_modified > ONE_DAY_IN_SECONDS {
        let json = get_json_data()?;
        let features = feature::json_to_features(json)?;
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&*constants::CACHE_PATH)?;
        file.write_all(serde_json::to_string(&features)?.as_bytes())?;
    }

    Ok(())
}

pub fn get_data() -> Result<Vec<Feature>, ApiError> {
    let json = fs::read_to_string(&*constants::CACHE_PATH)?;
    let features_result: serde_json::Result<Vec<Feature>> = serde_json::from_str(&json);
    match features_result {
        Ok(features) => Ok(features),
        Err(_) => {
            // if error, try updating the cache and try again
            ensure_cached_data()?;
            let json = fs::read_to_string(&*constants::CACHE_PATH)?;
            let features: Vec<Feature> = serde_json::from_str(&json)?;
            Ok(features)
        }
    }
}
