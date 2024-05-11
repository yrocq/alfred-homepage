use reqwest::blocking::get;
use reqwest::Result;

use crate::types::{BookmarksGroup, ServicesGroup};

pub struct HttpClient {}

impl HttpClient {
    pub fn get_services(url: &str) -> Result<Vec<ServicesGroup>> {
        get(HttpClient::build_endpoint(url, &"api/services".to_string()))?
            .json::<Vec<ServicesGroup>>()
    }

    pub fn get_bookmarks(url: &str) -> Result<Vec<BookmarksGroup>> {
        return get(HttpClient::build_endpoint(
            url,
            &"api/bookmarks".to_string(),
        ))?
        .json::<Vec<BookmarksGroup>>();
    }

    fn build_endpoint(url: &str, path: &str) -> String {
        format!("{url}/{path}")
    }
}
