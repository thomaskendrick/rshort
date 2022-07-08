use log::debug;

use anyhow::{anyhow, Result};

use reqwest::{header, StatusCode};
use serde::{Deserialize, Serialize};

use termion::{color, style};

use std::collections::HashMap;

use crate::RShortConfig;

#[derive(Serialize, Deserialize, Debug)]
struct ShortcutSearchResponse {
    stories: Data<Story>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
    id: isize,
    name: String,
    app_url: String,
    created_at: String,
    completed_at: Option<String>,
}

impl Story {
    pub fn print_line(&self) {
        println!(
            "{}{}#{}{}{}: {}",
            style::Bold,
            color::Fg(color::Green),
            self.id,
            style::Reset,
            color::Fg(color::Reset),
            self.name
        );
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Data<T> {
    data: Vec<T>,
}

pub async fn search_stories(query: &str, cfg: &RShortConfig) -> Result<Vec<Story>> {
    let client = reqwest::Client::new();

    let query = HashMap::from([("query", query)]);
    debug!("Sending query payload {:?}", query);

    let response = client
        .get("https://api.app.shortcut.com/api/v3/search")
        .header("Shortcut-Token", cfg.api_key.to_owned())
        .header(header::CONTENT_TYPE, "application/json")
        .json(&query)
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            let result = response.json::<ShortcutSearchResponse>().await?;
            Ok(result.stories.data)
        }
        _ => Err(anyhow!(
            "Recieved a bad status code when searching stories: {}",
            response.status()
        )),
    }
}
