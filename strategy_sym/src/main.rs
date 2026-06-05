use macroquad::prelude::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod defines;
mod draw;
mod game;
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
use crate::infrastructure::infstrt::*;
use crate::map::terrain::*;
use crate::menu::MenuType;
use crate::mouse::MouseTracker;
use crate::units::unit::*;


//TODO: 
/*
1. Get MCP server working - currently not able to connect
2. Add fighting. Damage system, is implemented but not used yet. Need to add attack actions and unit health.
3. Add more unit animations
4. Ranged units attacks
5. MCP server should also provide infrastructure control, not just units
6. Remove DamageUnit struct and just use UnitInfo with an optional field for damage taken. This will simplify the code and reduce the number of structs we need to manage.
7. Expand map size
8. Add roads and general asset work 
9. Terrain tile textures boundaries

Features to add:
1. Tech tree
2. Unit experience and leveling up

AI suggested:
-Refactor main loop to separate game logic and rendering more cleanly, possibly using a game state manager pattern.
-Implement a more robust event system for handling user input and game events, rather than directly processing them in the main loop.
-Add error handling and logging throughout the codebase to improve debugging and maintainability.

*/
#[macroquad::main("Strategy")]
async fn main() {
    {
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

    let mut state = menu::GameState::Menu;
    let mut id_gen = UnitId::new();

    let menu_paths = menu::MenuObjectPaths::new();
    let menu_skin = menu::create_menu_skin(&menu_paths)
        .await
        .expect("Failed to load menu assets");

    let mut map = TerrainGrid::new("assets/terrain_map.txt");
    let mut player_units_map = init_player_units(&mut id_gen);

    let mut infr_container = InfrastructureContainer::new();
    infr_container.init();
    for obj in infr_container.infr_objects.iter() {
        map.add_infr(obj.clone());
    }

    let mut enemy_units = AiUnits::new();
    enemy_units.add_test_units(&mut id_gen);
    for (unit_id, unit) in &enemy_units.units {
        map.add_hidden_unit(*unit_id, unit.location, Entity::AI);
    }

    menu::initialize_menu(&menu_skin).await;

    let mut textures = Textures::new().await.expect("Failed to load textures");

    let tile_count = GridTile::new(
        (screen_height() / TILE_SIZE.1) as u16,
        (screen_width() / TILE_SIZE.0) as u16,
    );

    let (mcp_cmd_tx, mcp_cmd_rx) = std::sync::mpsc::channel::<mcp_server::McpCommand>();
    let _mcp_server = mcp_server::start_mcp_server(mcp_cmd_tx);

    let mut mouse = MouseTracker::new();
    let mut menu_content: MenuType = MenuType::Main;
    let mut destroyed_units: Vec<DestroyedUnit> = Vec::new();

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
                draw_terrain(&mut textures, &mut map, tile_count).await;

                mouse.process_mouse_action(&player_units_map);

                mcp_handlers::process_mcp_commands(
                    &mcp_cmd_rx,
                    &mut player_units_map,
                    &enemy_units,
                    &mut map,
                    &mut destroyed_units,
                );

                let draw_unit_exception = handle_unit_interaction(
                    &mouse,
                    &mut player_units_map,
                    &mut textures,
                    &mut map,
                    &enemy_units,
                    &mut destroyed_units,
                )
                .await;

                draw_infrastructure(&mut textures, &infr_container).await;

                let new_units = infr_container.iterate_infrastructure(&mut id_gen);
                for unit in new_units {
                    add_unit(&mut player_units_map, &mut map, *unit);
                }

                draw_player_units(&mut textures, &mut player_units_map, draw_unit_exception).await;
                draw_visible_enemy_units(&mut map, &mut enemy_units, &mut textures, &player_units_map).await;
                draw_destroyed_units(&mut destroyed_units, &mut textures).await;

                menu::show_popup_menu(&mut mouse, &mut menu_content, &mut map, &player_units_map);
            }
        }

        next_frame().await
    }
}
