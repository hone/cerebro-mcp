use rmcp::{
    model::{CallToolResult, Content, ServerCapabilities, ServerInfo},
    tool, ServerHandler,
};

use crate::cerebro::{self, Cerebro};

#[derive(Clone)]
pub(crate) struct Server {
    cerebro: Cerebro,
}

#[tool(tool_box)]
impl Server {
    pub fn new() -> Self {
        Self {
            cerebro: Cerebro::new(),
        }
    }

    #[tool(description = "Fetch a list of Marvel Champions card data")]
    pub async fn get_cards(
        &self,
        #[tool(aggr)] request: cerebro::cards::Request,
    ) -> Result<CallToolResult, rmcp::Error> {
        Ok(CallToolResult::success(vec![Content::text(
            self.cerebro.get_cards(request).await?,
        )]))
    }

    #[tool(description = "Fetch a list of Marvel Champions pack data")]
    pub async fn get_packs(
        &self,
        #[tool(aggr)] params: cerebro::packs::Request,
    ) -> Result<CallToolResult, rmcp::Error> {
        Ok(CallToolResult::success(vec![Content::text(
            self.cerebro.get_packs(params).await?,
        )]))
    }

    #[tool(description = "Fetch a list of Marvel Champions set data")]
    pub async fn get_sets(
        &self,
        #[tool(aggr)] params: cerebro::sets::Request,
    ) -> Result<CallToolResult, rmcp::Error> {
        Ok(CallToolResult::success(vec![Content::text(
            self.cerebro.get_sets(params).await?,
        )]))
    }
}

#[tool(tool_box)]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Cerebro API".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
