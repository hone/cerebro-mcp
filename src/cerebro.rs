use reqwest::{Client, Url};
use rmcp::{
    model::{CallToolResult, Content, ErrorData, ServerCapabilities, ServerInfo},
    tool, ServerHandler,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

mod cards;
mod packs;
mod sets;

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

#[tool(tool_box)]
impl Cerebro {
    pub fn new() -> Cerebro {
        Cerebro {
            client: Client::new(),
            base_url: Url::parse(BASE_URL).unwrap(),
        }
    }

    #[tool(description = "Fetch a list of Marvel Champions card data")]
    pub async fn get_cards(
        &self,
        #[tool(aggr)] request: cards::Request,
    ) -> Result<CallToolResult, rmcp::Error> {
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

        Ok(CallToolResult::success(vec![Content::text(text)]))
    }

    #[tool(description = "Fetch a list of Marvel Champions pack data")]
    pub async fn get_packs(
        &self,
        #[tool(aggr)] params: packs::Request,
    ) -> Result<CallToolResult, rmcp::Error> {
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

        Ok(CallToolResult::success(vec![Content::text(text)]))
    }

    #[tool(description = "Fetch a list of Marvel Champions set data")]
    pub async fn get_sets(
        &self,
        #[tool(aggr)] params: sets::Request,
    ) -> Result<CallToolResult, rmcp::Error> {
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

        Ok(CallToolResult::success(vec![Content::text(text)]))
    }
}

#[tool(tool_box)]
impl ServerHandler for Cerebro {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Cerebro fixture data".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
