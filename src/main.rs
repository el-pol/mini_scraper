use std::thread;

fn fetch_status(url: &str) -> Result<u16, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?;

    Ok(resp.status().as_u16())
}

fn main() {
    let multiple = vec![
        "https://google.com",
        "https://as.com",
        "https://theverge.com",
    ];

    let handle = thread::spawn(|| {
        for i in multiple {
            match fetch_status(i) {
                Ok(status) => println!("Status code: {status}"),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
    });

    handle.join().unwrap();
}
