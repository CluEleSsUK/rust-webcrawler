mod site;
mod crawler;
mod config;

extern crate regex;

use std::io::{stdin};
use crate::crawler::{Crawler};

#[tokio::main]
async fn main() {
    let unvalidated_url = user_input_line();
    let crawler = create_default_crawler(&unvalidated_url);
    let sitemap = crawler.create_sitemap().await;

    println!("{}", sitemap.await.to_string());
}

fn user_input_line() -> String {
    println!("Enter a URL to start crawling: ");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    input.clone()
}

fn create_default_crawler(base_url: &str) -> Crawler {
    Crawler::new(base_url)
}
