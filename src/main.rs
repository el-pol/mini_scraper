fn fetch_status(url: &str) -> Result<u16, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?;

    Ok(resp.status().as_u16())
}

fn main() {
    let url = "https://www.rust-lang.org";
    match fetch_status(url) {
        Ok(status) => println!("Status code: {status}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
