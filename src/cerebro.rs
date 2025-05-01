use reqwest::{Client, Url};
use rmcp::model::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

pub mod cards;
pub mod packs;
pub mod sets;

const BASE_URL: &str = "https://cerebro-beta-bot.herokuapp.com/";

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Origin {
    All,
    Official,
    Unofficial,
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Origin::All => write!(f, "all"),
            Origin::Official => write!(f, "official"),
            Origin::Unofficial => write!(f, "unofficial"),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Cerebro {
    client: Client,
    base_url: Url,
}

impl Cerebro {
    pub fn new() -> Cerebro {
        Cerebro {
            client: Client::new(),
            base_url: Url::parse(BASE_URL).unwrap(),
        }
    }

    pub async fn get_cards(&self, request: cards::Request) -> Result<String, rmcp::Error> {
        let mut url = self.base_url.clone();
        url.set_path("cards");

        let query = serde_urlencoded::to_string(request).map_err(|e| {
            ErrorData::internal_error(format!("Failed to serialize request: {}", e), None)
        })?;

        url.set_query(Some(&query));

        tracing::info!("URL: {url}");

        let response =
            self.client.get(url).send().await.map_err(|e| {
                ErrorData::internal_error(format!("HTTP request failed: {}", e), None)
            })?;

        let text = response.text().await.map_err(|e| {
            ErrorData::internal_error(format!("Failed to read response body: {}", e), None)
        })?;

        Ok(text)
    }

    pub async fn get_packs(&self, params: packs::Request) -> Result<String, rmcp::Error> {
        let mut url = self.base_url.clone();
        url.set_path("packs");

        let query = serde_urlencoded::to_string(params).map_err(|e| {
            ErrorData::internal_error(format!("Failed to serialize request: {}", e), None)
        })?;

        url.set_query(Some(&query));

        tracing::info!("URL: {url}");

        let response =
            self.client.get(url).send().await.map_err(|e| {
                ErrorData::internal_error(format!("HTTP request failed: {}", e), None)
            })?;

        let text = response.text().await.map_err(|e| {
            ErrorData::internal_error(format!("Failed to read response body: {}", e), None)
        })?;

        Ok(text)
    }

    pub async fn get_sets(&self, params: sets::Request) -> Result<String, rmcp::Error> {
        let mut url = self.base_url.clone();
        url.set_path("sets");

        let query = serde_urlencoded::to_string(params).map_err(|e| {
            ErrorData::internal_error(format!("Failed to serialize request: {}", e), None)
        })?;

        url.set_query(Some(&query));

        tracing::info!("URL: {url}");

        let response =
            self.client.get(url).send().await.map_err(|e| {
                ErrorData::internal_error(format!("HTTP request failed: {}", e), None)
            })?;

        let text = response.text().await.map_err(|e| {
            ErrorData::internal_error(format!("Failed to read response body: {}", e), None)
        })?;

        Ok(text)
    }
}
