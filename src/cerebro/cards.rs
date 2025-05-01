use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::cerebro::Origin;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Parameters for filtering cards from the Cerebro API.")]
pub struct Request {
    #[schemars(
        description = "Filter by card origin ('official', 'unofficial', or 'all'). If omitted, the API defaults to 'all'."
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
