use anyhow::Result;
use rmcp::{
    ServiceExt, handler::server::wrapper::Parameters, schemars, tool, tool_router, transport::stdio,
};
use tracing_subscriber::EnvFilter;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
/// Request parameters for the terrain info operation
pub struct TerrainInfoRequest {
    #[schemars(description = "the location coordinates")]
    pub location: (i32, i32),
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
/// Request parameters for the subtraction operation
pub struct SubRequest {
    #[schemars(description = "the left hand side number")]
    pub a: i32,
    #[schemars(description = "the right hand side number")]
    pub b: i32,
}

#[derive(Debug, Clone)]
pub struct StratCommands;

#[tool_router(server_handler)]
impl StratCommands {
    #[tool(description = "Provide terrain information")]
    fn terrain_info(
        &self,
        Parameters(TerrainInfoRequest { location }): Parameters<TerrainInfoRequest>,
    ) -> String {
        format!("Terrain info for location: {:?}", location)
    }

    #[tool(description = "Calculate the difference of two numbers")]
    fn sub(&self, Parameters(SubRequest { a, b }): Parameters<SubRequest>) -> String {
        (a - b).to_string()
    }
}

pub fn start_mcp_server() -> std::thread::JoinHandle<Result<()>> {
    std::thread::spawn(|| {
        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()),
            )
            .with_writer(std::io::stderr)
            .with_ansi(false)
            .init();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        rt.block_on(async {
            let server = StratCommands;
            let service = server.serve(stdio()).await?;
            tracing::info!("Strategy sym MCP server initialized and waiting for requests");
            service.waiting().await?;
            Ok(())
        })
    })
}
