use crate::config::DefaultConfig;
use crate::site::Site;
use crate::site::Site::{Found, Visited};
use std::future::Future;
use std::pin::Pin;

pub trait CrawlConfig {
    fn should_crawl_site(&self, url: &str) -> bool;
    fn url_extractor(&self, text: &str) -> Vec<String>;
    fn network_fetch_text(&self, url: &str) -> Pin<Box<dyn Future<Output=Result<String, ()>>>>;
}

pub struct Crawler<'a> {
    pub starting_point: &'a str,
    pub config: Box<dyn CrawlConfig + 'a>,
}

impl<'a> Crawler<'a> {
    pub fn new(starting_point: &'a str) -> Crawler {
        Crawler {
            starting_point,
            config: Box::new(DefaultConfig::new(starting_point)),
        }
    }

    pub async fn create_sitemap(&'a self) -> Pin<Box<dyn Future<Output=Site> + 'a>> {
        self.sitemap_of(self.starting_point)
    }

    fn sitemap_of(&'a self, url: &'a str) -> Pin<Box<dyn Future<Output=Site> + 'a>> {
        let sitemap = async move {
            let url = url.clone();

            if !self.config.should_crawl_site(url) {
                return Found { url: url.to_string() };
            }

            let urls = self.config.network_fetch_text(url).await
                .map(|text| self.config.url_extractor(text.as_str()));

            match urls {
                Ok(urls) => {
                    let mut children = vec!();

                    for u in urls {
                        children.push(self.sitemap_of(u.as_str()).await);
                    }

                    Visited {
                        url: url.to_string(),
                        children,
                        is_error: false,
                    }
                }
                Err(_) => Visited {
                    url: url.to_string(),
                    children: vec!(),
                    is_error: true,
                },
            }
        };
        Box::pin(sitemap)
    }
}

