# mini-scraper: a concurrent web scraper

A high-performance web scraper written in Rust that fetches titles from multiple websites concurrently. The scraper implements retry logic and performance monitoring to ensure reliable operation.

I built this project to fundamentally learn about threads in Rust.

## Features

- Concurrent scraping using threads
- Automatic retry mechanism for failed requests
- Performance metrics tracking
- Simple URL input via text file
- Error handling and logging

## Requirements

- Rust (latest stable version)
- Internet connection
- A `urls.txt` file containing target URLs (**one per line**)

## Dependencies

The project uses the following main dependencies:

- [`reqwest`](https://docs.rs/reqwest/latest/reqwest/) - HTTP client
- [`scraper`](https://docs.rs/scraper/latest/scraper/) - HTML parsing
- [`chrono`](https://docs.rs/chrono/latest/chrono/) - Time handling

For more information about Rust's threading model, see the [Rust Threading Documentation](https://doc.rust-lang.org/book/ch16-01-threads.html).

## Installation

1. Clone the repository:

```bash
git clone <repository-url>
cd <repository-name>
```

2. Build the project:

```bash
cargo build --release
```

## Usage

1. Create a `urls.txt` file in the project root directory with your target URLs, one per line:

```text
https://example.com
https://another-site.com
https://third-site.com
```

2. Run the scraper:

```bash
cargo run --release
```

## How it Works

The scraper:

1. Reads URLs from `urls.txt`
2. Launches a separate thread for each URL
3. Attempts to fetch each page's title up to 3 times
4. Reports success/failure and timing information
5. Runs multiple rounds to gather performance metrics
6. Calculates and displays average round duration

## Output

The scraper provides detailed output including:

- Thread start times
- Success/failure status for each URL
- Page titles
- Request durations
- Round completion times
- Average performance metrics

## Error Handling

- Failed requests are retried up to 3 times
- 1-second delay between retry attempts
- Detailed error messages for failed requests
- Graceful handling of missing titles

## Performance

The scraper is designed for performance:

- Concurrent execution using threads
- Efficient HTML parsing
- Minimal memory footprint
- Performance metrics tracking

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
