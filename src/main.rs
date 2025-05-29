use reqwest;
use scraper;
use std::error::Error;
use std::thread;

fn fetch_status(url: &str) -> Result<u16, Box<dyn Error>> {
    let resp = reqwest::blocking::get(url)?;

    Ok(resp.status().as_u16())
}

fn fetch_title(url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    let parsed = scraper::Html::parse_document(&body);

    Ok("parsed".to_string())
}

fn main() {
    let multiple = vec![
        "https://google.com",
        "https://as.com",
        "https://theverge.com",
    ];

    let mut handles = vec![];

    for url in multiple {
        let handle = thread::spawn(|| match fetch_status(url) {
            Ok(status) => println!("Status code: {status}"),
            Err(e) => eprintln!("Error: {e}"),
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
