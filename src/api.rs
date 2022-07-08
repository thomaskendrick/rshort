use anyhow::{anyhow, Result};
use log::debug;
use reqwest::{header, StatusCode, ClientBuilder, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


use crate::RShortConfig;
use crate::story::Story;

#[derive(Serialize, Deserialize, Debug)]
struct ShortcutSearchResponse {
    stories: Data<Story>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data<T> {
    data: Vec<T>,
}

pub struct StorybookClient {
    client: Client
}

impl StorybookClient {
    pub fn new(cfg : &RShortConfig) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("Shortcut-Token", header::HeaderValue::from_str(&cfg.api_key).unwrap());
        Self {
           client: ClientBuilder::new().default_headers(headers).build().unwrap()
        }    
    }
    pub async fn search_stories(&self, query: &str) -> Result<Vec<Story>> {
        let query = HashMap::from([("query", query)]);
        debug!("Sending query payload {:?}", query);

        let response = self.client
            .get("https://api.app.shortcut.com/api/v3/search")
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
}

