use log::{debug, error};

use reqwest::{header, Error, StatusCode};
use serde::{Deserialize, Serialize};

use termion::{color, style};

use std::collections::HashMap;
use std::env;

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
            "{}{}sc-{}{}{}: {}",
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

pub async fn search_stories(query: &str) -> Result<Vec<Story>, Error> {
    let api_key = env::var("SHORTCUT_API_KEY").unwrap();
    let client = reqwest::Client::new();

    let query = HashMap::from([("query", query)]);
    debug!("Sending query payload {:?}", query);

    let response = client
        .get("https://api.app.shortcut.com/api/v3/search")
        .header("Shortcut-Token", api_key)
        .header(header::CONTENT_TYPE, "application/json")
        .json(&query)
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            let result = response.json::<ShortcutSearchResponse>().await?;

            Ok(result.stories.data)
        }
        _ => {
            // TODO Should probably do proper error handling here.
            error!(
                "Recieved bad status code ({}) processing query. Message: {}",
                response.status(),
                response.text().await?
            );
            panic!("recieved bad response")
        }
    }
}
