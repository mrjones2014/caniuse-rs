extern crate skim;
use crate::feature::Feature;
use crate::url;
use skim::prelude::*;
use skim::{ItemPreview, SkimItem};

impl SkimItem for Feature {
    fn text(&self) -> std::borrow::Cow<str> {
        Cow::from(format!("{}", self))
    }

    fn preview(&self, _context: skim::PreviewContext) -> skim::ItemPreview {
        ItemPreview::Text(format!(
            "{}\n{}\n{}",
            self.name, self.title, self.description
        ))
    }
}

pub fn find_with_skim(items: &[Feature]) {
    let skim_options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(false)
        .build()
        .unwrap();

    let (tx_items, rx_items): (SkimItemSender, SkimItemReceiver) = unbounded();
    items.iter().for_each(|feature| {
        let _ = tx_items.send(Arc::new(feature.to_owned()));
    });
    drop(tx_items); // informs Skim that all items have been sent

    let _ = Skim::run_with(&skim_options, Some(rx_items)).map(|out| {
        if out.final_key != Key::Enter {
            return;
        }

        let selected = out.selected_items.first();
        if let Some(item) = selected {
            let selected_feature = (**item).as_any().downcast_ref::<Feature>().unwrap();
            url::open(&selected_feature.url);
        }
    });
}
