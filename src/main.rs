mod http;
mod mappers;
mod types;

use crate::{http::HttpClient, mappers::map_error_to_alfred_config, types::AlfredConfig};
use clap::{arg, Parser};
use mappers::map_home_page_groups_to_alfred_config;
use reqwest::Error;
use serde_json::json;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Cli {
    /// Homepage url
    #[arg()]
    url: String,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match query(&cli.url) {
        Err(error) => {
            println!(
                "{}",
                json!(map_error_to_alfred_config(
                    &format!("{:#}", error).to_string()
                ))
            )
        }
        Ok(alfred_config) => println!("{}", json!(alfred_config)),
    }

    Ok(())
}

fn query(url: &str) -> Result<AlfredConfig, reqwest::Error> {
    let bookmarks = map_home_page_groups_to_alfred_config(HttpClient::get_bookmarks(&url)?);
    let services = map_home_page_groups_to_alfred_config(HttpClient::get_services(&url)?);

    Ok(AlfredConfig {
        items: [bookmarks, services].concat(),
        cache: Some(60),
    })
}
