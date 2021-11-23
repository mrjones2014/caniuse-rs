use crate::feature::{self, Feature};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::cmp::Ordering;
use std::env;
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
    HTTP(reqwest::Error),
    IO(io::Error),
    SystemTime(SystemTimeError),
    FeatureParsing(feature::Error),
    Serialization(serde_json::Error),
}

impl From<reqwest::Error> for ApiError {
    fn from(e: reqwest::Error) -> Self {
        ApiError::HTTP(e)
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
            ApiError::HTTP(e) => write!(f, "An error occurred fetching the latest data: {}", e),
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

fn get_cache_path() -> String {
    match env::var("HOME") {
        Ok(home) => format!("{}/.cache/caniuse-rs/data.json", home),
        Err(_) => String::from("/usr/local/share/caniuse-rs/data.json"),
    }
}

fn touch_cache_file() -> io::Result<()> {
    let cache_path = get_cache_path();
    let path = Path::new(&cache_path);
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
    let metadata = fs::metadata(&get_cache_path())?;
    let since_last_modified = SystemTime::now()
        .duration_since(metadata.modified()?)?
        .as_secs();

    if metadata.size() == 0 || since_last_modified > ONE_DAY_IN_SECONDS {
        let json = get_json_data()?;
        let features = feature::json_to_features(json)?;
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&get_cache_path())?;
        file.write_all(serde_json::to_string(&features)?.as_bytes())?;
    }

    Ok(())
}

pub fn get_data() -> Result<Vec<Feature>, ApiError> {
    let cache_path = get_cache_path();
    let json = fs::read_to_string(&cache_path)?;
    let features_result: serde_json::Result<Vec<Feature>> = serde_json::from_str(&json);
    match features_result {
        Ok(features) => Ok(features),
        Err(_) => {
            // if error, try updating the cache and try again
            ensure_cached_data()?;
            let json = fs::read_to_string(&cache_path)?;
            let features: Vec<Feature> = serde_json::from_str(&json)?;
            Ok(features)
        }
    }
}

pub fn fuzzy_find(query: &str, features: &Vec<Feature>) -> Vec<Feature> {
    let matcher = SkimMatcherV2::default();
    let mut scored_features: Vec<(feature::Feature, f64)> = Vec::new();
    for feature in features.iter() {
        // weight 1.5 for feature name
        let name_score = (matcher.fuzzy_match(&feature.name, query).unwrap_or(0) as f64) * 1.5;
        // weight 1.3 for title
        let title_score = (matcher.fuzzy_match(&feature.title, query).unwrap_or(0) as f64) * 1.3;
        // weight 0.8 for description
        let desc_score = (matcher
            .fuzzy_match(&feature.description, query)
            .unwrap_or(0) as f64)
            * 0.8;
        let score = name_score + title_score + desc_score;
        if score > 0.0 {
            scored_features.push((feature.to_owned(), score));
        }
    }

    scored_features.sort_by(|a, b| {
        if a.1 < b.1 {
            return Ordering::Less;
        }

        if a.1 > b.1 {
            return Ordering::Greater;
        }

        Ordering::Equal
    });

    let max_items = std::cmp::min(scored_features.len() - 1, 20);
    let mut features_by_score = scored_features
        .iter()
        .map(|tuple| tuple.to_owned().0)
        .collect::<Vec<feature::Feature>>()[0..max_items]
        .to_vec();

    if features_by_score.is_empty() {
        features_by_score.push(feature::Feature {
            name: String::from(query),
            title: String::from(query),
            description: format!("Search caniuse.com for: {}", query),
            url: format!("https://caniuse.com/?search={}", query),
            stats: Default::default(),
        });
    }

    features_by_score
}
