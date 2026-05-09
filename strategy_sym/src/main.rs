use macroquad::prelude::*;
mod defines;
mod draw;
mod infrastructure;
mod map;
mod menu;
mod mouse;
mod random;
mod units;
mod utils;

use crate::defines::*;
use crate::draw::*;
use crate::infrastructure::infstrt::*;
use crate::map::terrain::*;
use crate::menu::MenuType;
use crate::mouse::MouseTracker;
use crate::units::Units::*;

pub struct Textures {
    terrain: Box<TerrainTiles>,
    units: Box<AnimateUnit>,
    infrastructure: Box<InfrastuctureTextures>,
}
impl Textures {
    pub async fn new() -> Result<Textures, macroquad::Error> {
        let terr = load_terrain_textures().await?;
        let un = AnimateUnit::new().await?;
        let infr = InfrastuctureTextures::new().await?;

        Ok(Textures {
            terrain: terr,
            units: un,
            infrastructure: infr,
        })
    }
}

pub fn process_unit_movement(
    new_pos: GridTile,
    unit: &mut UnitInfo,
    map: &mut TerrainGrid,
) -> MoveResult {
    if unit.location != new_pos && unit.allowed_move(map.get_titletype_for_cord(new_pos).unwrap()) {
        let (move_successfull, mine_damage) =
            map.move_unit_to_new_tile(unit.unit_id, unit.location, new_pos, Entity::Player);
        if mine_damage {
            if unit.assess_damage(random::random_nums::generate(100)) {
                //unit dead. Remove from map and player units
                map.remove_unit(unit.unit_id, unit.location, Entity::Player);

                return MoveResult::UnitDestroyed;
            }
        }
        if move_successfull {
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

    return MoveResult::InvalidMove;
}

pub async fn draw_visible_enemy_units(
    map: &mut TerrainGrid,
    enemy_units: &AI_units,
    textures: &mut Textures,
    enemy_units_present: bool,
) {
    //draw visible AI units
    for (_, unit_map) in &map.visible_units_per_tile {
        for unit_id in unit_map {
            if let Some(unit) = enemy_units.units.get(unit_id) {
                paint_tile(
                    unit.location,
                    TILE_SIZE,
                    textures
                        .units
                        .get_texture(unit.unit_type, TextureType::Default),
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
                    enemy_units_present,
                )
                .await;
            }
        }
    }
}

pub async fn draw_player_unit(unit: &UnitInfo, textures: &mut Textures, cur_position: GridTile) {
    //draw player unit
    paint_tile(
        cur_position,
        TILE_SIZE,
        textures
            .units
            .get_texture(unit.unit_type, TextureType::Default),
        match unit.player_id {
            Entity::Player => false,
            Entity::AI => true,
        },
    )
    .await;

    //draw player unit health
    draw_health_bar(
        TILE_SIZE,
        cur_position,
        unit.get_health_bar(),
        Entity::Player,
        false,
    )
    .await;
}

pub async fn draw_terrain(textures: &mut Textures, map: &mut TerrainGrid, tile_count: GridTile) {
    //draw terrain
    for y in 0..tile_count.0 {
        for x in 0..tile_count.1 {
            let t_tile = (y, x);
            if let Some(t_type) = map.get_titletype_for_cord(t_tile) {
                paint_tile(
                    t_tile,
                    TILE_SIZE,
                    textures.terrain.get_tile_texture(t_type),
                    false,
                )
                .await;
            }
        }
    }
}

pub async fn draw_infrastructure(textures: &mut Textures, infra_vector: &InfrastructureContainer) {
    //draw infrastructure objects
    for infr_arc in infra_vector.infr_objects.iter() {
        let (loc, tp, detected) = {
            let obj = infr_arc.lock().unwrap();
            (obj.location, obj.infr_type, obj.detected)
        };

        if detected {
            paint_tile(
                loc,
                TILE_SIZE,
                textures.infrastructure.get_infra_texture(tp),
                false,
            )
            .await;
        }
    }
}

pub fn init_player_units(id_gen: &mut UnitId) -> PlayerUnits {
    let mut result = PlayerUnits::new();
    //result.add_unit_at(UnitTilesEnum::Tank, id_gen, Entity::Player, (2, 3));
    //result.add_unit_at(UnitTilesEnum::APC, id_gen, Entity::Player, (3, 3));
    result
}

/// Adds a unit to both the player_units_map and the map
pub fn add_unit(player_units_map: &mut PlayerUnits, map: &mut TerrainGrid, unit: UnitInfo) {
    // Add to map as hidden player unit
    map.add_hidden_unit(unit.unit_id, unit.location, Entity::Player);

    // Add to player_units_map
    player_units_map.add_unit(unit);
}

pub async fn draw_player_units(
    textures: &mut Textures,
    player_units_map: &PlayerUnits,
    excl: Option<GridTile>,
) {
    for (tile, units) in &player_units_map.units_by_tile {
        if excl.is_some() && excl.unwrap() == *tile {
            continue;
        }
        for unit in units.values() {
            draw_player_unit(unit, textures, unit.location).await;
        }
    }
}

pub async fn handle_unit_interaction(
    mouse: &MouseTracker,
    player_units_map: &mut PlayerUnits,
    textures: &mut Textures,
    map: &mut TerrainGrid,
    enemy_units: &AI_units,
) -> Option<GridTile> {
    if mouse.is_dragging() {
        let pixel = mouse.get_click_drag_draw_offset();
        let draw_unit_exception = mouse.get_start_cursor_tile();
        let id = mouse.get_selected_unit_id();
        if let Some(unit) = player_units_map
            .units_by_tile
            .get(&draw_unit_exception.unwrap())
            .and_then(|units| units.get(&id))
        {
            paint_tile_at_pixel(
                pixel,
                TILE_SIZE,
                textures
                    .units
                    .get_texture(unit.unit_type, TextureType::Default),
                false,
            )
            .await;
        }
        draw_unit_exception
    } else {
        if let Some(new_position) = mouse.get_new_tile_if_moved() {
            let id = mouse.get_selected_unit_id();
            let start_pos = mouse.get_start_cursor_tile().unwrap();

            if let Some(unit) = player_units_map
                .units_by_tile
                .get_mut(&start_pos)
                .unwrap()
                .get_mut(&id)
            {
                match process_unit_movement(new_position, unit, map) {
                    MoveResult::Success => {
                        player_units_map.move_unit(start_pos, id, new_position);
                    }
                    MoveResult::InvalidMove => {
                        // no action, unit was dropped back to original tile
                    }
                    MoveResult::UnitDestroyed => {
                        player_units_map.remove_unit(start_pos, id);
                    }
                }
            }
        }
        None
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
    let mut player_units_map = init_player_units(&mut id_gen);

    //Add infrastructure object and add few test items
    // add them to the map
    let mut infr_container = InfrastructureContainer::new();
    infr_container.Init();
    for obj in infr_container.infr_objects.iter() {
        map.add_infr(obj.clone());
    }

    let mut enemy_units = AI_units::new();
    enemy_units.add_test_units(&mut id_gen);

    for (unit_id, unit) in &enemy_units.units {
        map.add_hidden_unit(*unit_id, unit.location, Entity::AI);
    }

    menu::initialize_menu(&menu_skin).await;

    //load textures: terrain, units and infrastructure
    let mut textures = Textures::new().await.expect("Failed to load textures");

    let tile_count: GridTile = (
        (screen_width() / TILE_SIZE.0) as u16,
        (screen_height() / TILE_SIZE.1) as u16,
    );

    let mut mouse = MouseTracker::new();
    let mut menu_content: MenuType = MenuType::Main;

    loop {
        match state {
            menu::GameState::Menu => {
                menu::show_menu(&mut state).await;
                if state == menu::GameState::Game {
                    menu::clear_ui_skin();
                }
            }
            menu::GameState::Exit => {
                break;
            }
            _ => {
                clear_background(LIGHTGRAY);

                //draw a grid (TODO: macroquad has draw_grid too, explore using it)
                draw::draw_grid(tile_count, TILE_SIZE).await;

                draw_terrain(&mut textures, &mut map, tile_count).await;

                mouse.process_mouse_action(&player_units_map);

                //handle unit interaction and get the tile to exclude from drawing if unit is being dragged,
                //this is to prevent drawing the unit at its original tile while dragging
                let draw_unit_exception = handle_unit_interaction(
                    &mouse,
                    &mut player_units_map,
                    &mut textures,
                    &mut map,
                    &enemy_units,
                )
                .await;

                draw_infrastructure(&mut textures, &infr_container).await;

                //checking for new units to add to the map from infrastructure and adding them to player units if found
                let new_units = infr_container.iterate_infrastructure();
                for unit in new_units {
                    add_unit(&mut player_units_map, &mut map, *unit);
                }

                draw_player_units(&mut textures, &player_units_map, draw_unit_exception).await;

                draw_visible_enemy_units(&mut map, &enemy_units, &mut textures, false).await;
                //right click popup menu
                menu::show_popup_menu(&mut mouse, &mut menu_content, &mut map);
            } //game state
        }

        next_frame().await
    }
}
