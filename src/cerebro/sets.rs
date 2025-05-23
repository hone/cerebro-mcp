use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::cerebro::Origin;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Parameters for filtering sets from the Cerebro API.")]
pub struct Request {
    #[schemars(
        description = "Filter by origin ('official', 'unofficial', or 'all'). If omitted, the API defaults to 'all'."
    )]
    pub origin: Option<Origin>,
    #[schemars(description = "Filter by set ID.")]
    pub id: Option<String>,
    #[schemars(description = "Filter by set name.")]
    pub name: Option<String>,
    #[schemars(description = "Filter by set type.")]
    pub r#type: Option<String>,
}
