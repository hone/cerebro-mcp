use reqwest::{Client, Url};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

pub mod cards;
pub mod packs;
pub mod sets;

#[derive(Error, Debug)]
pub enum CerebroError {
    #[error("Failed to serialize request parameters: {0}")]
    SerializationError(#[from] serde_urlencoded::ser::Error),

    #[error("HTTP request failed: {0}")]
    HttpRequestError(#[from] reqwest::Error),

    #[error("Failed to read response body: {0}")]
    ResponseBodyError(String), // reqwest::Error doesn't implement Display for body errors
}

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
            // This constant should always unwrap
            base_url: Url::parse(BASE_URL).unwrap(),
        }
    }

    pub async fn get_cards(&self, request: cards::Request) -> Result<String, CerebroError> {
        let mut url = self.base_url.clone();
        url.set_path("cards");

        let query = serde_urlencoded::to_string(request)?;

        url.set_query(Some(&query));

        tracing::info!("URL: {url}");

        let response = self.client.get(url).send().await?;

        let text = response
            .text()
            .await
            .map_err(|e| CerebroError::ResponseBodyError(e.to_string()))?;

        Ok(text)
    }

    pub async fn get_packs(&self, params: packs::Request) -> Result<String, CerebroError> {
        let mut url = self.base_url.clone();
        url.set_path("packs");

        let query = serde_urlencoded::to_string(params)?;

        url.set_query(Some(&query));

        tracing::info!("URL: {url}");

        let response = self.client.get(url).send().await?;

        let text = response
            .text()
            .await
            .map_err(|e| CerebroError::ResponseBodyError(e.to_string()))?;

        Ok(text)
    }

    pub async fn get_sets(&self, params: sets::Request) -> Result<String, CerebroError> {
        let mut url = self.base_url.clone();
        url.set_path("sets");

        let query = serde_urlencoded::to_string(params)?;

        url.set_query(Some(&query));

        tracing::info!("URL: {url}");

        let response = self.client.get(url).send().await?;

        let text = response
            .text()
            .await
            .map_err(|e| CerebroError::ResponseBodyError(e.to_string()))?;

        Ok(text)
    }
}
