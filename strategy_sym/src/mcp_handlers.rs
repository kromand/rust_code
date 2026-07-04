use crate::defines::*;
use crate::game::{process_unit_movement, refresh_contested_tile};
use crate::game_assets::GameAssets;
use crate::infrastructure::infstrt::InfrastructureContainer;
use crate::map::terrain::TerrainGrid;
use crate::mcp_server::McpCommand;
use crate::units::unit::{UnitInfo, UnitsContainer, unit_has_destruction_animation};
use std::collections::HashSet;

// ---------------------------------------------------------------------------
// Per-command handlers — called by process_mcp_commands each frame
// ---------------------------------------------------------------------------

/// Moves one of the AI's own units. The MCP client commands the enemy faction,
/// so this operates on `enemy_units`; `player_units` is the opposing side, used
/// only for contested-tile bookkeeping.
pub fn mcp_move_unit(
    unit_id: usize,
    target: GridTile,
    enemy_units: &mut UnitsContainer,
    player_units: &UnitsContainer,
    map: &mut TerrainGrid,
    destroyed_units: &mut Vec<UnitInfo>,
    contested_tiles: &mut HashSet<GridTile>,
) -> String {
    let start_tile = match enemy_units.find_unit_tile(unit_id) {
        Some(t) => t,
        None => return format!("Unit {} not found", unit_id),
    };

    let movement_rate = enemy_units.units_by_tile[&start_tile].units[&unit_id].movement_rate;
    let dr = (target.col as i32 - start_tile.col as i32).unsigned_abs() as f32;
    let dc = (target.row as i32 - start_tile.row as i32).unsigned_abs() as f32;
    let chebyshev = dr.max(dc);

    if chebyshev > movement_rate {
        return format!(
            "Target ({},{}) out of range: distance {} exceeds movement rate {}",
            target.row, target.col, chebyshev as u32, movement_rate as u32
        );
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
            format!("Unit {} moved to ({},{})", unit_id, target.row, target.col)
        }
        MoveResult::UnitDestroyed => {
            if let Some(mut dead_unit) = enemy_units.pop_unit(start_tile, unit_id) {
                if unit_has_destruction_animation(dead_unit.unit_type) {
                    dead_unit.location = target;
                    dead_unit.start_destruction();
                    destroyed_units.push(dead_unit);
                }
            }
            refresh_contested_tile(start_tile, player_units, enemy_units, contested_tiles);
            format!(
                "Unit {} destroyed by mines at ({},{})",
                unit_id, target.row, target.col
            )
        }
        MoveResult::InvalidMove => format!(
            "Cannot move to ({},{}): terrain not passable for this unit type",
            target.row, target.col
        ),
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
        let obj = infr_arc.lock().unwrap();
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

pub fn mcp_tile_info(tile: GridTile, map: &TerrainGrid) -> String {
    match map.get_terrain_type(tile) {
        None => format!("Tile ({},{}) is out of bounds", tile.row, tile.col),
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
            format!(
                "Tile ({},{}): terrain={}, infrastructure={}",
                tile.row, tile.col, terrain, infra_str
            )
        }
    }
}

// ---------------------------------------------------------------------------
// Main dispatch — call once per frame from the game loop
// ---------------------------------------------------------------------------

pub fn process_mcp_commands(
    cmd_rx: &std::sync::mpsc::Receiver<McpCommand>,
    game_assets: &mut GameAssets,
) {
    let GameAssets {
        player_units_map: player_units,
        enemy_units_map: enemy_units,
        map,
        destroyed_units,
        contested_tiles,
        infr_container,
        ..
    } = game_assets;
    while let Ok(cmd) = cmd_rx.try_recv() {
        match cmd {
            McpCommand::MoveUnit {
                unit_id,
                target,
                resp,
            } => {
                let _ = resp.send(mcp_move_unit(unit_id, target, enemy_units, player_units, map, destroyed_units, contested_tiles));
            }
            McpCommand::ListMyUnits { resp } => {
                let _ = resp.send(mcp_list_my_units(enemy_units));
            }
            McpCommand::ListMyInfrastructure { resp } => {
                let _ = resp.send(mcp_list_my_infrastructure(infr_container));
            }
            McpCommand::ListVisibleEnemyUnits { resp } => {
                let _ = resp.send(mcp_list_visible_enemies(map, player_units));
            }
            McpCommand::TileInfo { tile, resp } => {
                let _ = resp.send(mcp_tile_info(tile, map));
            }
            McpCommand::GetMap { resp } => {
                let result = std::fs::read_to_string("assets/terrain_map.txt")
                    .unwrap_or_else(|e| format!("Failed to read map: {}", e));
                let _ = resp.send(result);
            }
        }
    }
}
