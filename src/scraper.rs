// #src/scraper.rs
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;

// Function to scrape book titles and prices from the given URL
pub fn scrape_books(url: &str) -> Result<(), Box<dyn Error>> {
    println!("Fetching content from: {}", url);

    // Create an HTTP client
    let client = Client::new();

    // Fetch the HTML content of the page
    let response = client.get(url).send()?.text()?;

    // Parse the HTML document
    let document = Html::parse_document(&response);

    // Define CSS selectors for book titles and prices
    let title_selector = Selector::parse(".product_pod h3 a").unwrap();
    let price_selector = Selector::parse(".product_pod .price_color").unwrap();

    // Ensure the /data directory exists
    let output_dir = "data";
    fs::create_dir_all(output_dir)?;

    // Open the file to write the data
    let file_path = format!("{}/output.csv", output_dir);
    let mut file = File::create(&file_path)?;
    writeln!(file, "Title,Price")?; // Write the CSV header

    // Extract and save titles and prices
    for (title_element, price_element) in document.select(&title_selector).zip(document.select(&price_selector)) {
        let title = title_element.value().attr("title").unwrap_or("No title");
        let price = price_element.text().collect::<Vec<_>>().join(" ");
        writeln!(file, "\"{}\",\"{}\"", title, price)?; // Write data in CSV format
    }

    println!("Data has been written to {}", file_path);
    Ok(())
}
