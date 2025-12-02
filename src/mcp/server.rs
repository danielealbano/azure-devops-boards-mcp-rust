use crate::azure::client::AzureDevOpsClient;

use crate::mcp::tools::classification_nodes::{ListAreaPathsArgs, ListIterationPathsArgs};
use crate::mcp::tools::organizations::{GetCurrentUserArgs, ListOrganizationsArgs};
use crate::mcp::tools::projects::ListProjectsArgs;
use crate::mcp::tools::tags::ListTagsArgs;
use crate::mcp::tools::teams::{
    GetTeamArgs, GetTeamCurrentIterationArgs, ListTeamMembersArgs, ListTeamsArgs,
    boards::{GetBoardArgs, ListBoardColumnsArgs, ListBoardRowsArgs, ListBoardsArgs},
};
use crate::mcp::tools::work_item_types::ListWorkItemTypesArgs;
use crate::mcp::tools::work_items::{
    AddCommentArgs, CreateWorkItemArgs, GetWorkItemArgs, GetWorkItemsArgs, LinkWorkItemsArgs,
    QueryWorkItemsArgs, QueryWorkItemsArgsWiql, UpdateWorkItemArgs,
};
use rmcp::{
    ErrorData as McpError,
    handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Implementation, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AzureMcpServer {
    client: Arc<AzureDevOpsClient>,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl AzureMcpServer {
    pub fn new(client: AzureDevOpsClient) -> Self {
        Self {
            client: Arc::new(client),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "List teams in the project")]
    async fn azdo_list_teams(
        &self,
        args: Parameters<ListTeamsArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::teams::list_teams(&self.client, args.0).await
    }

    #[tool(description = "List team members")]
    async fn azdo_list_team_members(
        &self,
        args: Parameters<ListTeamMembersArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::teams::list_team_members(&self.client, args.0).await
    }

    #[tool(description = "Get current user profile")]
    async fn azdo_get_current_user(
        &self,
        args: Parameters<GetCurrentUserArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::organizations::get_current_user(&self.client, args.0).await
    }

    #[tool(description = "List AzDO organizations")]
    async fn azdo_list_organizations(
        &self,
        args: Parameters<ListOrganizationsArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::organizations::list_organizations(&self.client, args.0).await
    }

    #[tool(description = "List projects in an organization")]
    async fn azdo_list_projects(
        &self,
        args: Parameters<ListProjectsArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::projects::list_projects(&self.client, args.0).await
    }

    #[tool(description = "List area paths for a project")]
    async fn azdo_list_area_paths(
        &self,
        args: Parameters<ListAreaPathsArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::classification_nodes::list_area_paths(&self.client, args.0).await
    }

    #[tool(description = "Get team details")]
    async fn azdo_get_team(
        &self,
        args: Parameters<GetTeamArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::teams::get_team(&self.client, args.0).await
    }

    #[tool(description = "List work item types")]
    async fn azdo_list_work_item_types(
        &self,
        args: Parameters<ListWorkItemTypesArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::work_item_types::list_work_item_types(&self.client, args.0).await
    }

    #[tool(description = "List tags")]
    async fn azdo_list_tags(
        &self,
        args: Parameters<ListTagsArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::tags::list_tags(&self.client, args.0).await
    }

    #[tool(description = "Get current iteration/sprint for team")]
    async fn azdo_get_team_current_iteration(
        &self,
        args: Parameters<GetTeamCurrentIterationArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::teams::get_team_current_iteration(&self.client, args.0).await
    }

    #[tool(description = "List iteration paths for a project or team")]
    async fn azdo_list_iteration_paths(
        &self,
        args: Parameters<ListIterationPathsArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::classification_nodes::list_iteration_paths(&self.client, args.0).await
    }

    #[tool(description = "List boards")]
    async fn azdo_list_team_boards(
        &self,
        args: Parameters<ListBoardsArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::teams::boards::list_team_boards(&self.client, args.0).await
    }

    #[tool(description = "Get board details")]
    async fn azdo_get_team_board(
        &self,
        args: Parameters<GetBoardArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::teams::boards::get_team_board(&self.client, args.0).await
    }

    #[tool(description = "List board columns")]
    async fn azdo_list_board_columns(
        &self,
        args: Parameters<ListBoardColumnsArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::teams::boards::list_board_columns(&self.client, args.0).await
    }

    #[tool(description = "List board rows (swimlanes)")]
    async fn azdo_list_board_rows(
        &self,
        args: Parameters<ListBoardRowsArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::teams::boards::list_board_rows(&self.client, args.0).await
    }

    #[tool(description = "Get work item by ID")]
    async fn azdo_get_work_item(
        &self,
        args: Parameters<GetWorkItemArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::work_items::get_work_item(&self.client, args.0).await
    }

    #[tool(description = "Get multiple work items by IDs")]
    async fn azdo_get_work_items(
        &self,
        args: Parameters<GetWorkItemsArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::work_items::get_work_items(&self.client, args.0).await
    }

    #[tool(description = "Add a comment to a work item")]
    async fn azdo_add_comment(
        &self,
        args: Parameters<AddCommentArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::work_items::add_comment(&self.client, args.0).await
    }

    #[tool(description = "Link work items")]
    async fn azdo_link_work_items(
        &self,
        args: Parameters<LinkWorkItemsArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::work_items::link_work_items(&self.client, args.0).await
    }

    #[tool(description = "Query work items using WIQL")]
    async fn azdo_query_work_items_by_wiql(
        &self,
        args: Parameters<QueryWorkItemsArgsWiql>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::work_items::query_work_items_by_wiql(&self.client, args.0).await
    }

    #[tool(description = "Create work item")]
    async fn azdo_create_work_item(
        &self,
        args: Parameters<CreateWorkItemArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::work_items::create_work_item(&self.client, args.0).await
    }

    #[tool(description = "Query work items by filters")]
    async fn azdo_query_work_items(
        &self,
        args: Parameters<QueryWorkItemsArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::work_items::query_work_items(&self.client, args.0).await
    }

    #[tool(description = "Update work item")]
    async fn azdo_update_work_item(
        &self,
        args: Parameters<UpdateWorkItemArgs>,
    ) -> Result<CallToolResult, McpError> {
        crate::mcp::tools::work_items::update_work_item(&self.client, args.0).await
    }
}

#[tool_handler]
impl rmcp::ServerHandler for AzureMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: env!("CARGO_PKG_NAME").into(),
                version: env!("CARGO_PKG_VERSION").into(),
                icons: None,
                title: None,
                website_url: Some(env!("CARGO_PKG_HOMEPAGE").into()),
            },
            instructions: Some(
                "Use this tool to interact with Azure DevOps Boards and Work Items".into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
