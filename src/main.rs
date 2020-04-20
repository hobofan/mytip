use tippytap::prelude::*;

pub fn sci_hub_tooltip(doi: &str) -> TipUrlLine {
    TipUrlLine {
        label: "SciHub".to_owned(),
        value: format!("https://sci-hub.tw/{}", doi),
    }
}

fn main() {
    let input = std::env::args().nth(1).expect("Missing input");

    let output = vec![
        TipTextLine {
            value: format!("Input {}", input),
        }
        .into(),
        sci_hub_tooltip(&input).into(),
    ];
    print_tooltips(&output);
}
