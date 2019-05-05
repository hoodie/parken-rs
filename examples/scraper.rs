use reqwest;
use scraper::{Selector, Html};
use scraper::element_ref::ElementRef;

#[derive(Debug)]
struct Lot {
    name: String,
    capacity: Option<u32>,
    free: Option<u32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let url = "https://apps.dresden.de/ords/f?p=1110:1:0:::::";
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()?;

    let name = Selector::parse("table.uReportContainer td[headers='BEZEICHNUNG'] div a").unwrap();
    let cap = Selector::parse("table.uReportContainer td[headers='KAPAZITAET'] div").unwrap();
    let free = Selector::parse("table.uReportContainer td[headers='FREI'] div").unwrap();

    // first fetch for the cookies
    client.get(url).send().is_err();

    let input = client.get(url).send()?.text()?;
    let document = Html::parse_document(&input);

    let inner = |elem_ref| ElementRef::inner_html(&elem_ref);

    let table_iter = 
        document.select(&name).map(inner)
        .zip(document.select(&cap).map(inner)
        .zip(document.select(&free).map(inner)))
        .map(|(name, (cap, free))| Lot {
            name: name.into(),
            capacity: cap.parse().ok(),
            free: free.parse().ok(),
        })
        ;

    for node in table_iter {
        println!("{:?}", node);
    }

    Ok(())
}