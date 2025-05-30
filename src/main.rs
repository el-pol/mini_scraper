use chrono::Local;
use reqwest;
use scraper;
use std::error::Error;
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

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

fn fetch_with_attempts(url: &str) {
    let now = Local::now();
    println!(
        "[{url}] 🕒 Thread started at: {}",
        now.format("%H:%M:%S%.3f")
    );
    let start = Instant::now();
    let number_of_attempts = 3;
    for attempt in 1..=number_of_attempts {
        println!("{attempt}");
        match fetch_title(&url) {
            Ok(title) => {
                println!("Success in attempt {:?}", attempt);
                let duration = start.elapsed();
                println!("[{url}] Title: {title} (took {:?})", duration);
                break;
            }
            Err(e) => {
                if attempt < number_of_attempts {
                    sleep(Duration::from_secs(1));
                } else {
                    println!("Giving up on URL: {url} with error {e}");
                }
            }
        }
    }
}

fn main() {
    let multiple = vec![
        "https://google.com",
        "https://as.com",
        "https://theverge.com",
        "https://news.ycombinator.com",
        "https://eldiario.es",
        "https://osldiario.es",
    ];

    let mut handles = vec![];

    for url in multiple {
        let handle = spawn(move || fetch_with_attempts(&url));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
