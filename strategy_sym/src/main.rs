use macroquad::prelude::*;
mod defines;
mod draw;
mod infrastructure;
mod map;
mod menu;
mod random;
mod units;

use crate::defines::*;
use crate::draw::*;
use crate::map::terrain::*;
use crate::units::Units::*;
use crate::infrastructure::infstrt::*;

pub struct PositionTracker {
    position: GridTile,
    bounds: GridTile,
    move_lock: bool,
}

impl PositionTracker {
    pub fn new(start: GridTile, bnds: GridTile) -> PositionTracker {
        PositionTracker {
            position: start,
            bounds: bnds,
            move_lock: false,
        }
    }
    pub fn get_new_position(self: &mut PositionTracker) -> GridTile {
        let mut new_pos: GridTile = self.position;
        if is_key_down(KeyCode::Right) {
            new_pos.0 += 1;
        } else if is_key_down(KeyCode::Left) {
            new_pos.0 -= 1;
        } else if is_key_down(KeyCode::Down) {
            new_pos.1 += 1;
        } else if is_key_down(KeyCode::Up) {
            new_pos.1 -= 1;
        }
        new_pos
    }
    pub fn inbounds_check(self: &PositionTracker, new_pos: GridTile) -> bool {
        new_pos.0 < self.bounds.0
            && new_pos.0 >= 0
            && new_pos.1 < self.bounds.1
            && new_pos.1 >= 0
            && !self.move_lock
    }

    pub fn move_unit(self: &mut PositionTracker, new_pos: GridTile) {
        self.position = new_pos;
        self.move_lock = true;
    }
    pub fn unlock_move(self: &mut PositionTracker) {
        self.move_lock = false;
    }
    pub fn get_position(self: &mut PositionTracker) -> GridTile {
        self.position
    }
}
#[macroquad::main("Strategy")]
async fn main() {
    let mut state = menu::GameState::Menu;
    let mut id_gen = UnitId::new();
    let menu_paths = menu::MenuObjectPaths::new();
    let menu_skin = menu::create_menu_skin(&menu_paths)
        .await
        .expect("Failed to load menu assets");
    let mut map = TerrainGrid::new("assets/terrain_map.txt");
    let mut unit = UnitInfo::new(
        UnitTilesEnum::AttackHeli,
        &mut id_gen,
        Entity::Player,
        (2, 3),
    );

    let mut enemy_units = AI_units::new();
    enemy_units.add_test_units(&mut id_gen);

    for (unit_id, unit) in &enemy_units.units {
        map.add_hidden_unit(*unit_id, unit.location, Entity::AI);
    }

    menu::initialize_menu(&menu_skin).await;

    //load terrain textures
    let terrain_textures = load_terrain_textures()
        .await
        .expect("Failed to load terrain textures");

    //load infracture assets
    let mut infr_textures = load_default_infra_textures(20)
        .await
        .expect("Failed to load infrastructure textures");

    //mutable since contains frame iterators, which will change
    let mut unit_textures = AnimateUnit::new()
        .await
        .expect("Failed to load unit textures");

    let tile_count: (i16, i16) = (
        (screen_width() / TILE_SIZE.0) as i16,
        (screen_height() / TILE_SIZE.1) as i16,
    );

    let mut cur_position = PositionTracker::new((2, 3), tile_count);
    let speed = 0.6;
    let mut last_update = get_time();

    loop {
        match state {
            menu::GameState::Menu => {
                menu::show_menu(&mut state).await;
            }
            menu::GameState::Exit => {
                break;
            }
            _ => {
                clear_background(LIGHTGRAY);

                let new_pos = cur_position.get_new_position();
                if cur_position.get_position() != new_pos
                    && cur_position.inbounds_check(new_pos)
                    && unit.allowed_move(map.get_titletype_for_cord(new_pos).unwrap())
                {
                    let (move_successfull, mine_damage) = map.process_unit_movement(
                        unit.unit_id,
                        cur_position.get_position(),
                        new_pos,
                        Entity::Player,
                    );
                    if mine_damage {
                        unit.assess_damage(random::random_nums::generate(100));
                    }
                    if move_successfull {
                        cur_position.move_unit(new_pos);
                        map.unit_detection_chance(
                            new_pos,
                            unit.visibility_range,
                            unit.prob_to_detect_units,
                            Entity::Player,
                        );
                    }
                }
                //draw a grid (TODO: macroquad has draw_grid too, explore using it)
                draw::draw_grid(tile_count, TILE_SIZE).await;

                if get_time() - last_update > speed {
                    last_update = get_time();
                    cur_position.unlock_move();
                }
                //draw terrain
                for y in 0..tile_count.0 {
                    for x in 0..tile_count.1 {
                        let t_tile = (y, x);
                        if let Some(t_type) = map.get_titletype_for_cord(t_tile) {
                            paint_tile(
                                t_tile,
                                TILE_SIZE,
                                terrain_textures.get_tile_texture(t_type),
                                false,
                            )
                            .await;
                        }
                    }
                }

                //draw player unit
                paint_tile(
                    cur_position.get_position(),
                    TILE_SIZE,
                    unit_textures.get_texture(unit.unit_type, TextTureType::Default),
                    match unit.player_id {
                        Entity::Player => false,
                        Entity::AI => true,
                    },
                )
                .await;

                //draw player unit health
                draw_health_bar(
                    TILE_SIZE,
                    cur_position.get_position(),
                    unit.get_health_bar(),
                    Entity::Player,
                    false,
                )
                .await;

                //draw visible AI units
                for (tile, unit_map) in &map.visible_units_per_tile {
                    for unit_id in unit_map {
                        if let Some(unit) = enemy_units.units.get(unit_id) {
                            paint_tile(
                                unit.location,
                                TILE_SIZE,
                                unit_textures.get_texture(unit.unit_type, TextTureType::Default),
                                match unit.player_id {
                                    Entity::Player => false,
                                    Entity::AI => true,
                                },
                            )
                            .await;

                            draw_health_bar(
                                TILE_SIZE,
                                unit.location,
                                unit.get_health_bar(),
                                Entity::AI,
                                unit.location == cur_position.get_position(),
                            )
                            .await;
                        }
                    }
                }
                //draw infrastructure
                paint_tile(
                    (18,5),
                    TILE_SIZE,
                    infr_textures.get_unit_texture(InfrastructureEnum::Fatory),
                    false,
                ).await;
            } //game state
        }

        next_frame().await
    }
}
