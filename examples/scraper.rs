use reqwest;
use scraper::{Selector, Html};

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let url = "https://apps.dresden.de/ords/f?p=1110:1:0:::::";
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()?;

    let selector = Selector::parse("table.uReportContainer td[headers='FREI'] div").unwrap();

    client.get(url).send();
    let input = client.get(url).send()?.text()?;
    let document = Html::parse_document(&input);

    for node in document.select(&selector) {
        println!("{:?}", node.inner_html());
    }

    Ok(())
}