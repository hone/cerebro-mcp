use reqwest::{Client, Url};
use rmcp::{
    ServerHandler,
    model::{ServerCapabilities, ServerInfo},
    tool,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, JsonSchema, Clone, Copy)]
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
#[derive(Debug, Deserialize, JsonSchema)]
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
        description = "Filter by classification ('player' or 'encounter'). Note: API uses lowercase."
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
        description = "Filter by traits (comma-separated list, e.g., 'avenger,shield'). All specified traits must be present. Note: API uses lowercase."
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
}

#[tool(tool_box)]
impl Cerebro {
    pub fn new() -> Cerebro {
        Cerebro {
            client: Client::new(),
        }
    }

    #[tool(description = "Fetch a list of Marvel Champions card data")]
    pub async fn cards(
        &self,
        #[tool(aggr)] CardsRequest {
            origin,
            incomplete,
            author,
            boost,
            classification,
            cost,
            exclude_campaign,
            name,
            resource,
            text,
            traits,
            type_,
            pack,
            set,
        }: CardsRequest,
    ) -> String {
        let mut url = Url::parse("https://cerebro-beta-bot.herokuapp.com/cards").unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(origin) = origin {
                query_pairs.append_pair("origin", origin.to_string().as_str());
            }
            if let Some(incomplete) = incomplete {
                query_pairs.append_pair("incomplete", incomplete.to_string().as_str());
            }
            if let Some(author) = author {
                query_pairs.append_pair("author", author.to_string().as_str());
            }
            if let Some(boost) = boost {
                query_pairs.append_pair("boost", boost.to_string().as_str());
            }
            if let Some(classification) = classification {
                query_pairs.append_pair("classification", classification.to_string().as_str());
            }
            if let Some(cost) = cost {
                query_pairs.append_pair("cost", cost.to_string().as_str());
            }
            if let Some(exclude_campaign) = exclude_campaign {
                query_pairs.append_pair("excludeCampaign", exclude_campaign.to_string().as_str());
            }
            if let Some(name) = name {
                query_pairs.append_pair("name", name.to_string().as_str());
            }
            if let Some(resource) = resource {
                query_pairs.append_pair("resource", resource.to_string().as_str());
            }
            if let Some(text) = text {
                query_pairs.append_pair("text", text.to_string().as_str());
            }
            if let Some(traits) = traits {
                query_pairs.append_pair("traits", traits.to_string().as_str());
            }
            if let Some(type_) = type_ {
                query_pairs.append_pair("type", type_.to_string().as_str());
            }
            if let Some(pack) = pack {
                query_pairs.append_pair("pack", pack.to_string().as_str());
            }
            if let Some(set) = set {
                query_pairs.append_pair("set", set.to_string().as_str());
            }
        }

        self.client
            .get(url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
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
