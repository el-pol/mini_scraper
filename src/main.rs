use chrono::Local;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
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

fn fetch_with_attempts(url: &str) -> Option<Duration> {
    let now = Local::now();
    println!(
        "[{url}] 🕒 Thread started at: {}",
        now.format("%H:%M:%S%.3f")
    );
    let start = Instant::now();
    let number_of_attempts = 3;
    for attempt in 1..=number_of_attempts {
        match fetch_title(url) {
            Ok(title) => {
                println!("Success in attempt {:?}", attempt);
                let duration = start.elapsed();
                println!("[{url}] Title: {title} (took {:?})", duration);
                return Some(duration);
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
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string("urls.txt")?;
    println!("{}", contents);

    let urls: Vec<String> = contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect();

    let rounds = 10;

    let mut round_durations = vec![];
    let mut results_as_hashmap: HashMap<String, Vec<Duration>> = HashMap::new();
    for round in 1..=rounds {
        println!("🔁 Round {round}");

        let start = Instant::now();
        // Launch threads to scrape
        let mut handles = vec![];

        for url in &urls {
            let url = url.clone();
            let handle = spawn(move || {
                let fetch_result = fetch_with_attempts(&url);
                (url, fetch_result)
            });
            handles.push(handle);
        }
        // Join threads
        for handle in handles {
            let (url, dur_option) = handle.join().unwrap();
            if let Some(dur) = dur_option {
                results_as_hashmap.entry(url).or_default().push(dur);
            }
        }

        let duration = start.elapsed();
        println!("Round {round} took {:?}", duration);
        round_durations.push(duration);
    }
    let total: Duration = round_durations.iter().sum();
    let avg = total / rounds;
    println!("📊 Average round duration: {:?}", avg);

    for (url, durations) in results_as_hashmap {
        let count = durations.len() as u32;
        if count > 0 {
            let total_duration: Duration = durations.iter().sum();
            let average_duration = total_duration / count;
            println!(
                "🔗 URL: {url} - Average duration over {count} successful requests: {:?}",
                average_duration
            );
        }
    }

    Ok(())
}
