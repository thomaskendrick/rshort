use log::{debug, error};

use reqwest::{header, Error, StatusCode};
use serde::{Deserialize, Serialize};

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
    completed_at: Option<String>
}

impl Story {
    pub fn print_summary(&self){
        println!("sc-{} : {}", self.id, self.name);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Data<T> {
    data: Vec<T>,
}

pub async fn fetch_story(id: &u16) -> Result<Option<Story>, Error> {
    let api_key = env::var("SHORTCUT_API_KEY").unwrap();
    debug!("Fetching story {}", id);
    let client = reqwest::Client::new();

    let mut id_str = String::from("id:");
    id_str.push_str(&id.to_string());

    let query = HashMap::from([("query", id_str)]);
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
            let mut result = response.json::<ShortcutSearchResponse>().await?;

            debug!("Successfully parsed story {:?}", result);

            // TODO: Currently this just takes the last story of
            // any results, should probably handle this better.
            let story = result.stories.data.pop();

            Ok(story)
        }
        _ => {
            // TODO Should probably do proper error handling here.
            error!(
                "Recieved bad status code ({}) processing query. Message: {}",
                response.status(),
                response.text().await?
            );
            Ok(None)
        }
    }
}
