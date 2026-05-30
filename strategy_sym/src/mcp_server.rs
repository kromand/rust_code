use anyhow::Result;
use crate::defines::GridTile;
use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};
use std::sync::mpsc;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const MCP_BIND_ADDRESS: &str = "127.0.0.1:8000";

// ---------------------------------------------------------------------------
// Commands sent from MCP tool handlers to the game loop.
// Each variant carries a oneshot sender for the response; the game loop
// fills it in synchronously and the awaiting tool handler unblocks.
// ---------------------------------------------------------------------------
pub enum McpCommand {
    MoveUnit {
        unit_id: usize,
        target: GridTile,
        resp: oneshot::Sender<String>,
    },
    ListPlayerUnits {
        resp: oneshot::Sender<String>,
    },
    ListVisibleEnemyUnits {
        resp: oneshot::Sender<String>,
    },
    TileInfo {
        tile: GridTile,
        resp: oneshot::Sender<String>,
    },
    GetMap {
        resp: oneshot::Sender<String>,
    },
}

// ---------------------------------------------------------------------------
// Request parameter structs (schema-annotated for MCP)
// ---------------------------------------------------------------------------

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct MoveUnitRequest {
    #[schemars(description = "numeric ID of the unit to move")]
    pub unit_id: usize,
    #[schemars(description = "target tile row (y)")]
    pub row: u16,
    #[schemars(description = "target tile column (x)")]
    pub col: u16,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct TileInfoRequest {
    #[schemars(description = "tile row (y)")]
    pub row: u16,
    #[schemars(description = "tile column (x)")]
    pub col: u16,
}

// ---------------------------------------------------------------------------
// StratCommands: MCP server handler
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct StratCommands {
    cmd_tx: mpsc::Sender<McpCommand>,
}

impl StratCommands {
    pub fn new(cmd_tx: mpsc::Sender<McpCommand>) -> Self {
        StratCommands { cmd_tx }
    }

    // Sends a command to the game loop and awaits the string response.
    async fn request(
        &self,
        make_cmd: impl FnOnce(oneshot::Sender<String>) -> McpCommand,
    ) -> String {
        let (tx, rx) = oneshot::channel();
        if self.cmd_tx.send(make_cmd(tx)).is_err() {
            return "Game loop unavailable".to_string();
        }
        rx.await.unwrap_or_else(|_| "No response received".to_string())
    }
}

#[tool_router(server_handler)]
impl StratCommands {
    #[tool(description = "Move a player unit to a target tile. The move is validated against \
        the unit's allowed terrain types and its maximum movement rate (Chebyshev distance).")]
    async fn move_unit(
        &self,
        Parameters(MoveUnitRequest { unit_id, row, col }): Parameters<MoveUnitRequest>,
    ) -> String {
        self.request(|resp| McpCommand::MoveUnit {
            unit_id,
            target: GridTile::new(row, col),
            resp,
        })
        .await
    }

    #[tool(description = "List all player-controlled units with their IDs, names, types, \
        tile coordinates, and current health.")]
    async fn list_player_units(&self) -> String {
        self.request(|resp| McpCommand::ListPlayerUnits { resp }).await
    }

    #[tool(description = "List all enemy units currently visible to the player, \
        including their types, tile coordinates, and health.")]
    async fn list_visible_enemy_units(&self) -> String {
        self.request(|resp| McpCommand::ListVisibleEnemyUnits { resp }).await
    }

    #[tool(description = "Return the terrain type and any infrastructure present on a specific tile.")]
    async fn tile_info(
        &self,
        Parameters(TileInfoRequest { row, col }): Parameters<TileInfoRequest>,
    ) -> String {
        self.request(|resp| McpCommand::TileInfo { tile: GridTile::new(row, col), resp }).await
    }

    #[tool(description = "Return the entire terrain map as a character grid. \
        Each cell encodes terrain type: F=Forest O=Ocean L=Lake M=Mountain G=Grass U=Urban. \
        Rows are separated by newlines.")]
    async fn get_map(&self) -> String {
        self.request(|resp| McpCommand::GetMap { resp }).await
    }
}

// ---------------------------------------------------------------------------
// Server startup
// ---------------------------------------------------------------------------

pub fn start_mcp_server(cmd_tx: mpsc::Sender<McpCommand>) -> std::thread::JoinHandle<Result<()>> {
    std::thread::spawn(move || {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "debug".into()),
            )
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(std::io::stderr)
                    .with_ansi(false),
            )
            .init();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        rt.block_on(async {
            let ct = CancellationToken::new();

            let service = StreamableHttpService::new(
                move || Ok(StratCommands::new(cmd_tx.clone())),
                LocalSessionManager::default().into(),
                StreamableHttpServerConfig::default().with_cancellation_token(ct.child_token()),
            );

            let router = axum::Router::new().nest_service("/mcp", service);
            let tcp_listener = tokio::net::TcpListener::bind(MCP_BIND_ADDRESS).await?;
            tracing::info!("Strategy sym MCP server listening on http://{}/mcp", MCP_BIND_ADDRESS);
            axum::serve(tcp_listener, router)
                .with_graceful_shutdown(async move {
                    tokio::signal::ctrl_c().await.unwrap();
                    ct.cancel();
                })
                .await?;
            Ok(())
        })
    })
}
