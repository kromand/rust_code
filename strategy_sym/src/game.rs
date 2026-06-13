use crate::defines::*;
use crate::draw::{Textures, paint_tile_at_pixel};
use crate::map::terrain::TerrainGrid;
use crate::mouse::MouseTracker;
use crate::random;
use crate::units::unit::*;
use std::collections::HashSet;

pub fn process_unit_movement(
    new_pos: GridTile,
    unit: &mut UnitInfo,
    map: &mut TerrainGrid,
) -> MoveResult {
    if unit.location != new_pos && unit.allowed_move(map.get_terrain_at(new_pos).unwrap()) {
        let (move_successful, mine_damage) =
            map.move_unit_to_new_tile(unit.unit_id, unit.location, new_pos, Entity::Player);

        if mine_damage
            && !is_air_unit(unit.unit_type)
            && unit.assess_damage(random::random_nums::generate(100))
        {
            map.remove_unit(unit.unit_id, unit.location, Entity::Player);
            return MoveResult::UnitDestroyed;
        }
        if move_successful {
            unit.location = new_pos;
            map.unit_detection_chance(
                new_pos,
                unit.visibility_range,
                unit.prob_to_detect_units,
                Entity::Player,
            );
            return MoveResult::Success;
        }
    }
    MoveResult::InvalidMove
}

/// Registers a new unit in both the player unit map and the terrain grid.
pub fn add_unit(player_units_map: &mut UnitsContainer, map: &mut TerrainGrid, unit: UnitInfo) {
    map.add_hidden_unit(unit.unit_id, unit.location, Entity::Player);
    player_units_map.add_unit(unit);
}

pub fn init_player_units(_id_gen: &mut UnitId) -> UnitsContainer {
    UnitsContainer::new()
}

/// Inserts or removes `tile` from `contested_tiles` based on whether both sides
/// currently have units there. Call after any move or death at that tile.
pub fn refresh_contested_tile(
    tile: GridTile,
    player_units: &UnitsContainer,
    enemy_units: &UnitsContainer,
    contested_tiles: &mut HashSet<GridTile>,
) {
    let has_player = player_units.has_units_at_tile(tile);
    let has_enemy = enemy_units.has_units_at_tile(tile);
    
    if has_player && has_enemy {
        contested_tiles.insert(tile);
        dbg!("Added contested tile at ({},{})", tile.row, tile.col);
    } else {
        contested_tiles.remove(&tile);
    }
}

fn apply_damage(
    tile: GridTile,
    unit_ids: &[usize],
    total_damage: f32,
    units: &mut UnitsContainer,
    entity: Entity,
    map: &mut TerrainGrid,
    destroyed_units: &mut Vec<UnitInfo>,
) {
    let dmg_each = total_damage / unit_ids.len() as f32;
    let dead = units.damage_units_at(tile, unit_ids, dmg_each);
    for (id, loc) in units.kill_units_at(tile, &dead, destroyed_units) {
        map.remove_unit(id, loc, entity);
    }
}

/// Resolve simultaneous combat on every tile in `contested_tiles`.
/// Damage is split evenly among units on each side; dead units are removed and
/// queued for their destruction animation when one exists.
pub fn resolve_combat(
    player_units: &mut UnitsContainer,
    enemy_units: &mut UnitsContainer,
    map: &mut TerrainGrid,
    destroyed_units: &mut Vec<UnitInfo>,
    contested_tiles: &mut HashSet<GridTile>,
) {
    let da = DamageAssessment::new();
    let tiles: Vec<GridTile> = contested_tiles.iter().copied().collect();

    for tile in tiles {
        let player_ids = player_units.unit_ids_at(tile);
        let enemy_ids = enemy_units.unit_ids_at(tile);

        if player_ids.is_empty() || enemy_ids.is_empty() {
            contested_tiles.remove(&tile);
            continue;
        }

        let player_slice = player_units.unit_refs_at(tile, &player_ids);
        let enemy_slice = enemy_units.unit_refs_at(tile, &enemy_ids);
        tracing::info!(
            "Combat at ({},{}) — {} player vs {} enemy",
            tile.row, tile.col, player_slice.len(), enemy_slice.len()
        );
        let (dmg_to_players, dmg_to_ai) = da.resolve_combat(&player_slice, &enemy_slice);
        dbg!(
            "Damage to players: {:.1}, damage to enemy: {:.1}",
            dmg_to_players,
            dmg_to_ai
        );
        apply_damage(tile, &player_ids, dmg_to_players, player_units, Entity::Player, map, destroyed_units);
        apply_damage(tile, &enemy_ids, dmg_to_ai, enemy_units, Entity::Enemy, map, destroyed_units);
        refresh_contested_tile(tile, player_units, enemy_units, contested_tiles);
    }
}

/// Handles drag-to-move mouse interaction.
/// While dragging, renders the unit under the cursor and returns the source tile
/// so the caller can skip drawing it at its original position.
/// On drop, validates and applies the move.
pub async fn mouse_unit_drag_handler(
    mouse: &MouseTracker,
    player_units_map: &mut UnitsContainer,
    textures: &mut Textures,
    map: &mut TerrainGrid,
    enemy_units: &UnitsContainer,
    destroyed_units: &mut Vec<UnitInfo>,
    contested_tiles: &mut HashSet<GridTile>,
) -> Option<GridTile> {
    if mouse.is_dragging() {
        let pixel = mouse.get_click_drag_draw_offset();
        let drag_source = mouse.get_start_cursor_tile();
        let id = mouse.get_selected_unit_id();

        if let Some(unit) = player_units_map
            .units_by_tile
            .get_mut(&drag_source.unwrap())
            .and_then(|stack| stack.units.get_mut(&id))
        {
            let texture = textures.units.get_texture(
                unit.unit_type,
                health_to_texture_type(unit.health / unit.max_health),
                &mut unit.frame_itr,
            );
            paint_tile_at_pixel(pixel, TILE_SIZE, texture, false).await;
        }
        drag_source
    } else {
        if let Some(new_position) = mouse.get_new_tile_if_moved() {
            let id = mouse.get_selected_unit_id();
            let start_pos = mouse.get_start_cursor_tile().unwrap();

            if let Some(unit) = player_units_map
                .units_by_tile
                .get_mut(&start_pos)
                .unwrap()
                .units
                .get_mut(&id)
            {
                match process_unit_movement(new_position, unit, map) {
                    MoveResult::Success => {
                        player_units_map.move_unit(start_pos, id, new_position);
                        refresh_contested_tile(
                            start_pos,
                            player_units_map,
                            enemy_units,
                            contested_tiles,
                        );
                        refresh_contested_tile(
                            new_position,
                            player_units_map,
                            enemy_units,
                            contested_tiles,
                        );
                    }
                    MoveResult::InvalidMove => {}
                    MoveResult::UnitDestroyed => {
                        if let Some(mut dead_unit) = player_units_map.pop_unit(start_pos, id) {
                            if unit_has_destruction_animation(dead_unit.unit_type) {
                                dead_unit.location = new_position;
                                dead_unit.start_destruction();
                                destroyed_units.push(dead_unit);
                            }
                        }
                        refresh_contested_tile(
                            start_pos,
                            player_units_map,
                            enemy_units,
                            contested_tiles,
                        );
                    }
                }
            }
        }
        None
    }
}
