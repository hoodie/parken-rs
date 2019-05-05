use reqwest;
use scraper::{Selector, Html};
use scraper::element_ref::ElementRef;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Lot {
    name: String,
    capacity: Option<u32>,
    free: Option<u32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let url = "https://apps.dresden.de/ords/f?p=1110:1:0:::::";

    let start_client = Instant::now();
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()?;
    let done_client = Instant::now();

    let start_selectors = Instant::now();
    let name = Selector::parse("table.uReportContainer td[headers='BEZEICHNUNG'] div a").unwrap();
    let cap = Selector::parse("table.uReportContainer td[headers='KAPAZITAET'] div").unwrap();
    let free = Selector::parse("table.uReportContainer td[headers='FREI'] div").unwrap();
    let done_selectors = Instant::now();

    // first fetch for the cookies
    let start_first_get = Instant::now();
    client.get(url).send().is_err();
    let done_first_get = Instant::now();

    let start_second_get = Instant::now();
    let input = client.get(url).send()?.text()?;
    let done_second_get = Instant::now();

    let start_parsing = Instant::now();
    let document = Html::parse_document(&input);
    let done_parsing = Instant::now();

    let inner = |elem_ref| ElementRef::inner_html(&elem_ref);

    let start_scraping = Instant::now();
    let table_iter = 
        document.select(&name).map(inner)
        .zip(document.select(&cap).map(inner)
        .zip(document.select(&free).map(inner)))
        .map(|(name, (cap, free))| Lot {
            name: name.into(),
            capacity: cap.parse().ok(),
            free: free.parse().ok(),
        }).collect::<Vec<_>>()
        ;
    let done_scraping = Instant::now();

    for node in &table_iter {
        println!("{:?}", node);
    }

    println!("init client     {:>8}ms", (done_client - start_client).as_micros());
    println!("parse selectors {:>8}ms", (done_selectors - start_selectors).as_micros());
    println!("first get       {:>8}ms", (done_first_get - start_first_get).as_micros());
    println!("second get      {:>8}ms", (done_second_get - start_second_get).as_micros());
    println!("parse document  {:>8}ms", (done_parsing - start_parsing).as_micros());
    println!("scrape content  {:>8}ms", (done_scraping - start_scraping).as_micros());

    Ok(())
}