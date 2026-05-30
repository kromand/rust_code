use crate::defines::*;
use crate::game::process_unit_movement;
use crate::map::terrain::TerrainGrid;
use crate::mcp_server::McpCommand;
use crate::units::units::{AiUnits, PlayerUnits};

// ---------------------------------------------------------------------------
// Per-command handlers — called by process_mcp_commands each frame
// ---------------------------------------------------------------------------

pub fn mcp_move_unit(
    unit_id: usize,
    target: GridTile,
    player_units: &mut PlayerUnits,
    map: &mut TerrainGrid,
) -> String {
    let start_tile = match player_units.find_unit_tile(unit_id) {
        Some(t) => t,
        None => return format!("Unit {} not found", unit_id),
    };

    let movement_rate = player_units.units_by_tile[&start_tile].units[&unit_id].movement_rate;
    let dr = (target.col as i32 - start_tile.col as i32).unsigned_abs() as f32;
    let dc = (target.row as i32 - start_tile.row as i32).unsigned_abs() as f32;
    let chebyshev = dr.max(dc);

    if chebyshev > movement_rate {
        return format!(
            "Target ({},{}) out of range: distance {} exceeds movement rate {}",
            target.row, target.col, chebyshev as u32, movement_rate as u32
        );
    }

    let unit = player_units
        .units_by_tile
        .get_mut(&start_tile)
        .and_then(|s| s.units.get_mut(&unit_id))
        .unwrap();

    match process_unit_movement(target, unit, map) {
        MoveResult::Success => {
            player_units.move_unit(start_tile, unit_id, target);
            format!("Unit {} moved to ({},{})", unit_id, target.row, target.col)
        }
        MoveResult::UnitDestroyed => {
            player_units.remove_unit(start_tile, unit_id);
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

pub fn mcp_list_player_units(player_units: &PlayerUnits) -> String {
    let mut lines = vec!["Player units:".to_string()];
    for (_, stack) in &player_units.units_by_tile {
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

pub fn mcp_list_visible_enemies(map: &TerrainGrid, enemy_units: &AiUnits) -> String {
    let mut lines = vec!["Visible enemy units:".to_string()];
    for (tile, unit_ids) in &map.visible_units_per_tile {
        for id in unit_ids {
            if let Some(unit) = enemy_units.units.get(id) {
                lines.push(format!(
                    "  id={} type={} loc=({},{}) hp={:.0}/{:.0}",
                    id, unit.unit_type, tile.row, tile.col, unit.health, unit.max_health
                ));
            }
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
    player_units: &mut PlayerUnits,
    enemy_units: &AiUnits,
    map: &mut TerrainGrid,
) {
    while let Ok(cmd) = cmd_rx.try_recv() {
        match cmd {
            McpCommand::MoveUnit {
                unit_id,
                target,
                resp,
            } => {
                let _ = resp.send(mcp_move_unit(unit_id, target, player_units, map));
            }
            McpCommand::ListPlayerUnits { resp } => {
                let _ = resp.send(mcp_list_player_units(player_units));
            }
            McpCommand::ListVisibleEnemyUnits { resp } => {
                let _ = resp.send(mcp_list_visible_enemies(map, enemy_units));
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
