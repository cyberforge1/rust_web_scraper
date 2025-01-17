// #src/utils.rs
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::error::Error;

// Function to create an HTTP client with custom headers
pub fn create_http_client() -> Result<Client, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Rust Web Scraper/1.0 (+https://example.com)"),
    );

    let client = Client::builder().default_headers(headers).build()?;
    Ok(client)
}
