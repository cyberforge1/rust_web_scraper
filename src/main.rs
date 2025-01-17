// #src/main.rs

mod scraper;
mod utils;

fn main() {
    println!("Starting the Books to Scrape web scraper...");

    // URL of the website to scrape
    let url = "http://books.toscrape.com/";

    // Call the scraper
    if let Err(e) = scraper::scrape_books(url) {
        eprintln!("Error occurred: {}", e);
    }
}
