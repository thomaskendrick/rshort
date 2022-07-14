use anyhow::{anyhow, Result};
use log::debug;
use reqwest::{header, Client, ClientBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::story::Story;
use crate::task::Task;
use crate::RShortConfig;

#[derive(Serialize, Deserialize, Debug)]
struct ShortcutSearchResponse {
    stories: Data<Story>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data<T> {
    data: Vec<T>,
}

pub struct StorybookClient {
    client: Client,
}

impl StorybookClient {
    pub fn new(cfg: &RShortConfig) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Shortcut-Token",
            header::HeaderValue::from_str(&cfg.api_key).unwrap(),
        );
        Self {
            client: ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }
    pub async fn search_stories(&self, query: &str) -> Result<Vec<Story>> {
        let query = HashMap::from([("query", query)]);
        debug!("Sending query payload {:?}", query);

        let response = self
            .client
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
    pub async fn get_story(&self, id: usize) -> Result<Option<Story>> {
        let response = self
            .client
            .get(format!(
                "https://api.app.shortcut.com/api/v3/stories/{}",
                id
            ))
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let story = response.json::<Story>().await?;
                Ok(Some(story))
            }
            StatusCode::NOT_FOUND => Ok(None),
            _ => Err(anyhow!(
                "Recieved a bad status code when searching stories: {}",
                response.status()
            )),
        }
    }
    pub async fn add_story_task(&self, story_id: &usize, message: &str) -> Result<Option<Task>> {
        let body = HashMap::from([("description", message)]);
        let response = self
            .client
            .post(format!(
                "https://api.app.shortcut.com/api/v3/stories/{}/tasks",
                story_id
            ))
            .json(&body)
            .send()
            .await?;
        match response.status() {
            StatusCode::CREATED => {
                let task = response.json::<Task>().await?;
                Ok(Some(task))
            }
            StatusCode::NOT_FOUND => Ok(None),
            _ => Err(anyhow!(
                "Recieved a bad status code when posting new task: {}, {}",
                response.status(),
                response.text().await?
            )),
        }
    }
}
