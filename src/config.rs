use crate::crawler::CrawlConfig;
use regex::Regex;
use tokio::macros::support::Pin;
use std::future::Future;

pub struct DefaultConfig {
    starting_url: String
}

impl DefaultConfig {
    pub fn new(starting_url: &str) -> Self {
        DefaultConfig { starting_url: starting_url.to_string() }
    }
}

impl CrawlConfig for DefaultConfig {
    fn should_crawl_site(&self, url: &str) -> bool {
        url.starts_with(self.starting_url.as_str())
    }

    fn url_extractor(&self, text: &str) -> Vec<String> {
        extract_url_with_regex(text)
    }

    fn network_fetch_text(&self, url: &str) -> Pin<Box<dyn Future<Output=Result<String, ()>>>> {
        let url = url.to_string();
        Box::pin(async move {
            request_with_reqwest(url.as_str()).await.map_err(|_| ())
        })
    }
}

fn extract_url_with_regex(text: &str) -> Vec<String> {
    let magical_url_regex = "(https?|ftp|file)://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]";

    Regex::new(magical_url_regex)
        .unwrap()
        .find_iter(text)
        .map(|m| m.as_str().trim().to_string())
        .collect()
}

async fn request_with_reqwest(url: &str) -> reqwest::Result<String> {
    reqwest::get(url).await?.text().await
}
