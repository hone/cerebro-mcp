use reqwest::{Client, Url};
use rmcp::{
    ServerHandler,
    model::{CallToolResult, Content, ErrorData, ServerCapabilities, ServerInfo},
    tool,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

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
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Parameters for filtering cards from the Cerebro API.")]
pub struct CardsRequest {
    #[schemars(
        description = "Filter by card origin ('official', 'unofficial', or 'all'). If omitted, the API defaults might apply (often 'ALL')."
    )]
    pub origin: Option<Origin>,

    #[schemars(description = "Filter incomplete cards.")]
    pub incomplete: Option<bool>,

    #[schemars(description = "Filter by card author ID.")]
    pub author: Option<String>,

    #[schemars(
        description = "Filter by boost icon value (e.g., '{b}' or '{s}'. For more than one boost icon, append them one after another: '{b}{b}'. If there is a '{s}', it always comes first.)."
    )]
    pub boost: Option<String>,

    #[schemars(
        description = "Filter by classification ('encounter', 'basic', 'protection', '). This is sometimes called \"aspect\". Note: API uses lowercase."
    )]
    pub classification: Option<String>,

    #[schemars(description = "Filter by card cost (e.g., '-', '0', '1', 'X').")]
    pub cost: Option<String>,

    #[serde(rename = "excludeCampaign")]
    #[schemars(description = "If present (e.g., 'true'), exclude campaign cards.")]
    pub exclude_campaign: Option<bool>,

    #[schemars(description = "Filter by card name (partial match).")]
    pub name: Option<String>,

    #[schemars(
        description = "Filter by printed resource ('{p}' for Phyical, '{m}' for Mental, '{e}' for Energy, '{w}' for Wild, or 'none'). To filter by more than one resource, just append them (i.e. {{Energy}}{{Physical}}). Note: API uses lowercase."
    )]
    pub resource: Option<String>,

    #[schemars(description = "Filter by text in the card's rules or special text.")]
    pub text: Option<String>,

    #[schemars(
        description = "Filter by traits (comma-separated list, e.g., 'Avenger,S.H.I.E.L.D.'). All specified traits must be present."
    )]
    pub traits: Option<String>,

    #[serde(rename = "type")]
    #[schemars(
        description = "Filter by card type (e.g., 'ally', 'event', 'hero', 'villain', 'minion', etc.). Note: API uses lowercase."
    )]
    pub type_: Option<String>,

    #[schemars(description = "Filter by pack code or name.")]
    pub pack: Option<String>,

    #[schemars(description = "Filter by set code or name.")]
    pub set: Option<String>,
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
        #[tool(aggr)] request: CardsRequest,
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
    pub async fn get_packs(&self) -> Result<CallToolResult, rmcp::Error> {
        let mut url = self.base_url.clone();
        url.set_path("packs");

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
