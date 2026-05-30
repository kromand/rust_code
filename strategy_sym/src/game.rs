use crate::defines::*;
use crate::draw::{Textures, paint_tile_at_pixel};
use crate::map::terrain::TerrainGrid;
use crate::mouse::MouseTracker;
use crate::random;
use crate::units::units::*;

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
pub fn add_unit(player_units_map: &mut PlayerUnits, map: &mut TerrainGrid, unit: UnitInfo) {
    map.add_hidden_unit(unit.unit_id, unit.location, Entity::Player);
    player_units_map.add_unit(unit);
}

pub fn init_player_units(_id_gen: &mut UnitId) -> PlayerUnits {
    PlayerUnits::new()
}

/// Handles drag-to-move mouse interaction.
/// While dragging, renders the unit under the cursor and returns the source tile
/// so the caller can skip drawing it at its original position.
/// On drop, validates and applies the move.
pub async fn handle_unit_interaction(
    mouse: &MouseTracker,
    player_units_map: &mut PlayerUnits,
    textures: &mut Textures,
    map: &mut TerrainGrid,
    _enemy_units: &AiUnits,
) -> Option<GridTile> {
    if mouse.is_dragging() {
        let pixel = mouse.get_click_drag_draw_offset();
        let drag_source = mouse.get_start_cursor_tile();
        let id = mouse.get_selected_unit_id();

        if let Some(unit) = player_units_map
            .units_by_tile
            .get(&drag_source.unwrap())
            .and_then(|stack| stack.units.get(&id))
        {
            paint_tile_at_pixel(
                pixel,
                TILE_SIZE,
                textures.units.get_texture(
                    unit.unit_type,
                    health_to_texture_type(unit.health / unit.max_health),
                ),
                false,
            )
            .await;
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
                    }
                    MoveResult::InvalidMove => {}
                    MoveResult::UnitDestroyed => {
                        player_units_map.remove_unit(start_pos, id);
                    }
                }
            }
        }
        None
    }
}
