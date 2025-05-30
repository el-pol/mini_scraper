use reqwest;
use scraper;
use std::error::Error;
use std::thread;
use std::time::Instant;

fn fetch_title(url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    let selector = scraper::Selector::parse("title").unwrap();
    let document = scraper::Html::parse_document(&body);

    let title = document
        .select(&selector)
        .next()
        .map(|t| t.text().collect::<Vec<_>>().join(""))
        .unwrap_or("(no title found)".to_string());

    Ok(title)
}

fn main() {
    let multiple = vec![
        "https://google.com",
        "https://as.com",
        "https://theverge.com",
        "https://news.ycombinator.com",
        "https://eldiario.es",
    ];

    let mut handles = vec![];

    for url in multiple {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            match fetch_title(&url) {
                Ok(title) => {
                    let duration = start.elapsed();
                    println!("[{url}] Title: {title} (took {:?})", duration);
                }
                Err(e) => eprintln!("Error: {e}"),
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
