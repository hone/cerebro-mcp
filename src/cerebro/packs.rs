use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::cerebro::Origin;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Parameters for filtering packs from the Cerebro API.")]
pub struct Request {
    #[schemars(
        description = "Filter by pack origin ('official', 'unofficial', or 'all'). If omitted, the API defaults to 'all'."
    )]
    pub origin: Option<Origin>,
    #[schemars(description = "Filter incomplete cards.")]
    pub incomplete: Option<bool>,
    #[schemars(description = "Filter by pack ID.")]
    pub id: Option<String>,
    #[schemars(description = "Filter by pack name.")]
    pub name: Option<String>,
}
