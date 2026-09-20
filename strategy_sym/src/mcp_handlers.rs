use crate::defines::*;
use crate::game::{process_unit_movement, refresh_contested_tile};
use crate::game_assets::GameAssets;
use crate::infrastructure::infstrt::InfrastructureContainer;
use crate::map::terrain::TerrainGrid;
use crate::mcp_server::McpCommand;
use crate::units::unit::{UnitsContainer, unit_has_destruction_animation};
use tokio::sync::oneshot;

// ---------------------------------------------------------------------------
// Per-command handlers — called by process_mcp_commands each frame
// ---------------------------------------------------------------------------

/// Moves one of the AI's own units. The MCP client commands the enemy faction,
/// so this operates on `enemy_units`; `player_units` is the opposing side, used
/// only for contested-tile bookkeeping.
pub fn mcp_move_unit(
    unit_id: usize,
    target: GridTile,
    game_assets: &mut GameAssets,
) -> Result<String, String> {
    let GameAssets {
        enemy_units_map: enemy_units,
        player_units_map: player_units,
        map,
        destroyed_units,
        contested_tiles,
        ..
    } = game_assets;
    let start_tile = match enemy_units.find_unit_tile(unit_id) {
        Some(t) => t,
        None => return Err(format!("Unit {} not found", unit_id)),
    };

    let movement_rate = enemy_units.units_by_tile[&start_tile].units[&unit_id].movement_rate;
    let dr = (target.col as i32 - start_tile.col as i32).unsigned_abs() as f32;
    let dc = (target.row as i32 - start_tile.row as i32).unsigned_abs() as f32;
    let chebyshev = dr.max(dc);

    if chebyshev > movement_rate {
        return Err(format!(
            "Target ({},{}) out of range: distance {} exceeds movement rate {}",
            target.row, target.col, chebyshev as u32, movement_rate as u32
        ));
    }

    let unit = enemy_units
        .units_by_tile
        .get_mut(&start_tile)
        .and_then(|s| s.units.get_mut(&unit_id))
        .unwrap();

    match process_unit_movement(target, unit, map, Entity::Enemy) {
        MoveResult::Success => {
            enemy_units.move_unit(start_tile, unit_id, target);
            refresh_contested_tile(start_tile, player_units, enemy_units, contested_tiles);
            refresh_contested_tile(target, player_units, enemy_units, contested_tiles);
            Ok(format!(
                "Unit {} moved to ({},{})",
                unit_id, target.row, target.col
            ))
        }
        MoveResult::UnitDestroyed => {
            if let Some(mut dead_unit) = enemy_units.pop_unit(start_tile, unit_id) 
                && unit_has_destruction_animation(dead_unit.unit_type) {
                    dead_unit.location = target;
                    dead_unit.start_destruction();
                    destroyed_units.push(dead_unit);
            }
            refresh_contested_tile(start_tile, player_units, enemy_units, contested_tiles);
            Ok(format!(
                "Unit {} destroyed by mines at ({},{})",
                unit_id, target.row, target.col
            ))
        }
        MoveResult::InvalidMove => Err(format!(
            "Cannot move to ({},{}): terrain not passable for this unit type",
            target.row, target.col
        )),
    }
}

/// Lists the AI's own units. The MCP client controls the in-game *enemy*
/// faction, so "my units" are the enemy units — listed in full, no fog of war.
pub fn mcp_list_my_units(enemy_units: &UnitsContainer) -> String {
    let mut lines = vec!["Your units:".to_string()];
    for (_, stack) in &enemy_units.units_by_tile {
        for (id, unit) in &stack.units {
            lines.push(format!(
                "  id={} name={} type={} loc=({},{}) hp={:.0}/{:.0}",
                id,
                unit.unit_name,
                unit.unit_type,
                unit.location.row,
                unit.location.col,
                unit.health,
                unit.max_health
            ));
        }
    }
    if lines.len() == 1 {
        lines.push("  (none)".to_string());
    }
    lines.join("\n")
}

/// Lists the opposing units the AI can currently see. From the MCP client's
/// perspective the opponent is the *player* faction, so these are player units
/// that the enemy side has detected.
pub fn mcp_list_visible_enemies(map: &TerrainGrid, player_units: &UnitsContainer) -> String {
    let mut lines = vec!["Visible enemy units:".to_string()];
    for (tile, id) in map.visible_units_for(Entity::Player) {
        if let Some(unit) = player_units.units_by_tile.get(&tile).and_then(|s| s.units.get(&id)) {
            lines.push(format!(
                "  id={} type={} loc=({},{}) hp={:.0}/{:.0}",
                id, unit.unit_type, tile.row, tile.col, unit.health, unit.max_health
            ));
        }
    }
    if lines.len() == 1 {
        lines.push("  (none)".to_string());
    }
    lines.join("\n")
}

/// Lists the AI's own infrastructure. The MCP client commands the enemy
/// faction, so these are the enemy-owned buildings.
pub fn mcp_list_my_infrastructure(infr_container: &InfrastructureContainer) -> String {
    let mut lines = vec!["Your infrastructure:".to_string()];
    for infr_arc in &infr_container.infr_objects {
        let obj = infr_arc.borrow();
        if obj.owner == Entity::Enemy {
            lines.push(format!(
                "  type={} loc=({},{})",
                obj.infr_type, obj.location.row, obj.location.col
            ));
        }
    }
    if lines.len() == 1 {
        lines.push("  (none)".to_string());
    }
    lines.join("\n")
}

/// Queues a unit for production at the AI's own (enemy-owned) factory or
/// airfield on `tile`. The unit must be buildable at that building.
pub fn mcp_enqueue_unit(
    tile: GridTile,
    unit_type_name: &str,
    map: &mut TerrainGrid,
) -> Result<String, String> {
    let unit_type = match UnitTilesEnum::from_name(unit_type_name) {
        Some(t) => t,
        None => return Err(format!("Unknown unit type '{}'", unit_type_name)),
    };

    if map.has_infrastructure(tile, InfrastructureEnum::Factory, Entity::Enemy) {
        if !map.get_factory_allowed_units(tile).contains(&unit_type) {
            return Err(format!(
                "Factory at ({},{}) cannot build {}",
                tile.row, tile.col, unit_type
            ));
        }
        map.enqueue_unit_in_factory(tile, unit_type);
        return Ok(format!(
            "Queued {} at factory ({},{})",
            unit_type, tile.row, tile.col
        ));
    }

    if map.has_infrastructure(tile, InfrastructureEnum::Airfield, Entity::Enemy) {
        if !map.get_airfield_allowed_units(tile).contains(&unit_type) {
            return Err(format!(
                "Airfield at ({},{}) cannot build {}",
                tile.row, tile.col, unit_type
            ));
        }
        map.enqueue_unit_in_airfield(tile, unit_type);
        return Ok(format!(
            "Queued {} at airfield ({},{})",
            unit_type, tile.row, tile.col
        ));
    }

    Err(format!(
        "No factory or airfield you own at ({},{})",
        tile.row, tile.col
    ))
}

pub fn mcp_tile_info(tile: GridTile, map: &TerrainGrid) -> Result<String, String> {
    match map.get_terrain_type(tile) {
        None => Err(format!("Tile ({},{}) is out of bounds", tile.row, tile.col)),
        Some(terrain) => {
            let infra = map.get_tile_infrastructure(tile);
            let infra_str = if infra.is_empty() {
                "none".to_string()
            } else {
                infra
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            Ok(format!(
                "Tile ({},{}): terrain={}, infrastructure={}",
                tile.row, tile.col, terrain, infra_str
            ))
        }
    }
}

// ---------------------------------------------------------------------------
// Main dispatch — call once per frame from the game loop
// ---------------------------------------------------------------------------

/// Sends a fallible handler's result back to the MCP client, logging failures
/// at WARN. The failure message is still returned to the caller.
fn reply(resp: oneshot::Sender<String>, result: Result<String, String>) {
    let msg = match result {
        Ok(msg) => msg,
        Err(msg) => {
            tracing::warn!("MCP command failed: {}", msg);
            msg
        }
    };
    let _ = resp.send(msg);
}

pub fn process_mcp_commands(
    cmd_rx: &std::sync::mpsc::Receiver<McpCommand>,
    game_assets: &mut GameAssets,
) {
    while let Ok(cmd) = cmd_rx.try_recv() {
        match cmd {
            McpCommand::MoveUnit {unit_id,target,resp,
            } => {
                tracing::info!("MCP MoveUnit: unit={} target=({},{})",unit_id,target.row,target.col);
                reply(resp, mcp_move_unit(unit_id, target, game_assets));
            }
            McpCommand::ListMyUnits { resp } => {
                tracing::info!("MCP ListMyUnits");
                let _ = resp.send(mcp_list_my_units(&game_assets.enemy_units_map));
            }
            McpCommand::ListMyInfrastructure { resp } => {
                tracing::info!("MCP ListMyInfrastructure");
                let _ = resp.send(mcp_list_my_infrastructure(&game_assets.infr_container));
            }
            McpCommand::ListVisibleEnemyUnits { resp } => {
                tracing::info!("MCP ListVisibleEnemyUnits");
                let _ = resp.send(mcp_list_visible_enemies(
                    &game_assets.map,
                    &game_assets.player_units_map,
                ));
            }
            McpCommand::TileInfo { tile, resp } => {
                tracing::info!("MCP TileInfo: tile=({},{})", tile.row, tile.col);
                reply(resp, mcp_tile_info(tile, &game_assets.map));
            }
            McpCommand::EnqueueUnit {
                tile,
                unit_type,
                resp,
            } => {
                tracing::info!(
                    "MCP EnqueueUnit: tile=({},{}) unit_type={}",
                    tile.row,
                    tile.col,
                    unit_type
                );
                reply(resp, mcp_enqueue_unit(tile, &unit_type, &mut game_assets.map));
            }
            McpCommand::GetMap { resp } => {
                tracing::info!("MCP GetMap");
                let result = match std::fs::read_to_string("assets/terrain_map.txt") {
                    Ok(contents) => contents,
                    Err(e) => {
                        let msg = format!("Failed to read map: {}", e);
                        tracing::warn!("MCP command failed: {}", msg);
                        msg
                    }
                };
                let _ = resp.send(result);
            }
        }
    }
}
