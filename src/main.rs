use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::cmp::Ordering;
use std::env;
use std::error::Error;

mod api;
mod browser;
mod feature;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data = api::get_json_data().await?;
    let features = feature::json_to_features(data)?;
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() < 2 || args[1].is_empty() {
        panic!("No query passed as argument.");
    }

    let query = &args[1];
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

    // TODO temporary serialize then deserialize to test (de)serialization
    let serialized = serde_json::to_string(&features_by_score)?;
    let deserialized: Vec<feature::Feature> = serde_json::from_str(&serialized)?;
    println!("{}", serde_json::to_string(&deserialized)?);
    Ok(())
}
