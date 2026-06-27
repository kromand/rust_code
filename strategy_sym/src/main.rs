use macroquad::prelude::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod defines;
mod draw;
mod game;
mod game_assets;
mod infrastructure;
mod map;
mod mcp_handlers;
mod mcp_server;
mod menu;
mod mouse;
mod random;
mod units;
mod utils;

use crate::defines::*;
use crate::draw::*;
use crate::game::*;
use crate::game_assets::GameAssets;
use crate::menu::MenuType;
use crate::units::unit::*;

//TODO:
/*
1. Fix MCP server - still not working claude code sends GET request. 
3. Add more unit animations
4. Ranged units attacks
5. MCP server should also provide infrastructure control, not just units
7. Expand map size
8. Add roads and general asset work
9. Terrain tile textures boundaries - started working on forest to plains transition but it needs more work to look good
10. resolve_combat still seems a bit over complicated, getting  unit refs should be enough raher than getting refs and ids
11. id_gen possibly should be moved to game_assets and mouse removed from it
Features to add:
1. Tech tree
2. Unit experience and leveling up

AI suggested:
-Refactor main loop to separate game logic and rendering more cleanly, possibly using a game state manager pattern.
-Implement a more robust event system for handling user input and game events, rather than directly processing them in the main loop.
-Add error handling and logging throughout the codebase to improve debugging and maintainability.

*/
/// Initializes the tracing subscriber, appending logs to `log/events.log`.
fn init_logging() {
    use std::fs;
    use std::sync::Mutex;
    fs::create_dir_all("log").expect("failed to create log dir");
    let log_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("log/events.log")
        .expect("failed to open log/events.log");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(Mutex::new(log_file))
                .with_ansi(false),
        )
        .init();
}

#[macroquad::main("Strategy")]
async fn main() {
    init_logging();

    let mut state = menu::GameState::Menu;
    let mut id_gen = UnitId::new();

    let menu_paths = menu::MenuObjectPaths::new();
    let menu_skin = menu::create_menu_skin(&menu_paths)
        .await
        .expect("Failed to load menu assets");

    menu::initialize_menu(&menu_skin).await;

    let mut game_assets = GameAssets::new(&mut id_gen).await;

    let tile_count = GridTile::new(
        (screen_height() / TILE_SIZE.1) as u16,
        (screen_width() / TILE_SIZE.0) as u16,
    );

    let (mcp_cmd_tx, mcp_cmd_rx) = std::sync::mpsc::channel::<mcp_server::McpCommand>();
    let _mcp_server =
        mcp_server::start_mcp_server(mcp_cmd_tx, mcp_server::McpTransport::from_env());

    let mut menu_content: MenuType = MenuType::Main;
    let mut last_combat_time = get_time();

    loop {
        match state {
            menu::GameState::Menu => {
                menu::show_menu(&mut state).await;
                if state == menu::GameState::Game {
                    menu::clear_ui_skin();
                }
            }
            menu::GameState::Exit => break,
            _ => {
                clear_background(LIGHTGRAY);

                draw::draw_grid(tile_count, TILE_SIZE).await;
                draw_terrain(&mut game_assets, tile_count).await;
                //TODO: fix calling process_mouse_action and passing another reference to game_assets, this is a bit messy and should be refactored
                game_assets
                    .mouse
                    .process_mouse_action(&game_assets.player_units_map);

                mcp_handlers::process_mcp_commands(&mcp_cmd_rx, &mut game_assets);

                let draw_unit_exception = mouse_unit_drag_handler(&mut game_assets).await;

                if get_time() - last_combat_time >= 2.0 {
                    resolve_combat(&mut game_assets);
                    last_combat_time = get_time();
                }

                draw_infrastructure(&mut game_assets).await;

                let new_units = game_assets.infr_container.iterate_infrastructure(&mut id_gen);
                for unit in new_units {
                    add_unit(&mut game_assets, *unit);
                }

                draw_player_units(&mut game_assets, draw_unit_exception).await;
                draw_visible_enemy_units(&mut game_assets).await;
                draw_destroyed_units(&mut game_assets).await;

                menu::show_popup_menu(&mut game_assets, &mut menu_content);
            }
        }

        next_frame().await
    }
}
