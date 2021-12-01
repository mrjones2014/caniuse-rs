extern crate skim;
use crate::feature::Feature;
use crate::url;
use skim::prelude::*;
use skim::{ItemPreview, SkimItem};

impl SkimItem for Feature {
    fn text(&self) -> std::borrow::Cow<str> {
        Cow::from(self.string_for_matching())
    }

    fn preview(&self, _context: skim::PreviewContext) -> skim::ItemPreview {
        ItemPreview::Text(format!(
            "{}\n{}\n{}",
            self.name, self.title, self.description
        ))
    }
}

fn features_to_receiver(items: &[Feature]) -> SkimItemReceiver {
    let (tx_items, rx_items): (SkimItemSender, SkimItemReceiver) = unbounded();
    items.iter().for_each(|feature| {
        let _ = tx_items.send(Arc::new(feature.to_owned()));
    });
    drop(tx_items); // indicates that all items have been sent
    rx_items
}

pub fn find_noninteractive(items: &[Feature], query: String) -> Vec<Feature> {
    let fuzzy_engine = ExactOrFuzzyEngineFactory::builder()
        .exact_mode(false)
        .fuzzy_algorithm(FuzzyAlgorithm::SkimV2)
        .build()
        .create_engine_with_case(&query, CaseMatching::Smart);
    let receiver = features_to_receiver(items);

    let mut results: Vec<(i32, Feature)> = Vec::new();
    receiver.into_iter().for_each(|feature| {
        let match_result = fuzzy_engine.match_item(feature.clone());
        if match_result.is_some() {
            let match_item = match_result.unwrap();
            let feature_converted = (*feature)
                .as_any()
                .downcast_ref::<Feature>()
                .unwrap()
                .to_owned();
            let rank = match_item.rank;
            results.push((rank.iter().sum(), feature_converted));
        }
    });

    results.sort_by_cached_key(|feature_rank| feature_rank.0);
    results
        .iter()
        .map(|feature_rank| feature_rank.1.to_owned())
        .collect()
}

pub fn find_interactive(items: &[Feature]) {
    let skim_options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(false)
        .build()
        .unwrap();

    let receiver = features_to_receiver(items);

    let _ = Skim::run_with(&skim_options, Some(receiver)).map(|out| {
        if out.final_key != Key::Enter {
            return;
        }

        let selected = out.selected_items.first();
        match selected {
            Some(item) => {
                let selected_feature = (**item).as_any().downcast_ref::<Feature>().unwrap();
                url::open(&selected_feature.url);
            }
            None => url::open(format!("https://caniuse.com/?search={}", out.query)),
        }
    });
}
