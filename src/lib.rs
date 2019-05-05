use reqwest;
use scraper::element_ref::ElementRef;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Lot {
    pub name: String,
    pub capacity: Option<u32>,
    pub free: Option<u32>,
}

pub trait Parken {
    fn get_lots() -> Result<Vec<Lot>, Box<dyn std::error::Error>>;
}

pub struct Dresden;

impl Parken for Dresden {
    fn get_lots() -> Result<Vec<Lot>, Box<dyn std::error::Error>> {

        let url = "https://apps.dresden.de/ords/f?p=1110:1:0:::::";

        let client = reqwest::Client::builder().cookie_store(true).build()?;

        let name = Selector::parse("table.uReportContainer td[headers='BEZEICHNUNG'] div a").unwrap();
        let cap = Selector::parse("table.uReportContainer td[headers='KAPAZITAET'] div").unwrap();
        let free = Selector::parse("table.uReportContainer td[headers='FREI'] div").unwrap();

        // first fetch for the cookies
        client.get(url).send().is_err();

        let input = client.get(url).send()?.text()?;

        let document = Html::parse_document(&input);

        let inner = |elem_ref| ElementRef::inner_html(&elem_ref);

        Ok(
            document.select(&name).map(inner)
            .zip(document.select(&cap).map(inner)
            .zip(document.select(&free).map(inner))
            )
            .map(|(name, (cap, free))| Lot {
                name,
                capacity: cap.parse().ok(),
                free: free.parse().ok(),
            }).collect::<Vec<_>>())
    }
}
