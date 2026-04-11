use macroquad::prelude::*;
mod defines;
mod draw;
mod infrastructure;
mod map;
mod menu;
mod random;
mod units;
mod utils;

use crate::defines::*;
use crate::draw::*;
use crate::infrastructure::infstrt::*;
use crate::map::terrain::*;
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

pub struct MouseTracker {
    start_cursor_position: Option<PixelOffset>,
    end_cursor_position: Option<PixelOffset>,
    tile_size: PixelOffset,
}

impl MouseTracker {
    pub fn new(t_size: PixelOffset) -> MouseTracker {
        MouseTracker {
            end_cursor_position: None,
            tile_size: t_size,
            start_cursor_position: None,
        }
    }
    pub fn process_mouse_action(self: &mut MouseTracker) {
        let (mouse_x, mouse_y) = mouse_position();

        if is_mouse_button_down(MouseButton::Left) {
            if self.start_cursor_position.is_none() {
                self.end_cursor_position = None;
                self.start_cursor_position = Some((mouse_x, mouse_y));
            }
        } else {
            if self.end_cursor_position.is_none() {
                self.end_cursor_position = Some((mouse_x, mouse_y));
            } else if self.end_cursor_position.is_some() {
                self.end_cursor_position = None;
                self.start_cursor_position = None;
            }
        }

        if is_mouse_button_down(MouseButton::Right) {}
        if is_mouse_button_down(MouseButton::Middle) {}
    }
    pub fn is_dragging(self: &MouseTracker) -> bool {
        is_mouse_button_down(MouseButton::Left)
            && self.start_cursor_position.is_some()
            && self.end_cursor_position.is_none()
    }
    pub fn get_cursor_pointed_tile(self: &MouseTracker) -> GridTile {
        let (mouse_x, mouse_y) = mouse_position();
        utils::conv::pixel_offset_to_grid((mouse_x, mouse_y))
    }
    pub fn get_new_tile_if_moved(self: &MouseTracker) -> Option<GridTile> {
        if self.start_cursor_position.is_some() && self.end_cursor_position.is_some() {
            let (mouse_x, mouse_y) = mouse_position();
            return Some(utils::conv::pixel_offset_to_grid((mouse_x, mouse_y)));
        }
        None
    }

    pub fn get_click_drag_draw_offset(self: &MouseTracker) -> PixelOffset {
        let cur_pos: PixelOffset = mouse_position();

        // this method should be called only when LMB was clicked and held, on calling unwrap on start_cursor_position is safe
        let original_tile = utils::conv::pixel_offset_to_grid(self.start_cursor_position.unwrap());
        let original_tile_offset = utils::conv::pixel_offset_of_gridtile(original_tile);
        let delta: PixelOffset = (
            self.start_cursor_position.unwrap().0 - original_tile_offset.0,
            self.start_cursor_position.unwrap().1 - original_tile_offset.1,
        );

        utils::conv::zero_floor_sub(cur_pos, delta)
    }
}
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
        } else if is_key_down(KeyCode::Left) && new_pos.0 > 0 {
            new_pos.0 -= 1;
        } else if is_key_down(KeyCode::Down) {
            new_pos.1 += 1;
        } else if is_key_down(KeyCode::Up) && new_pos.1 > 0 {
            new_pos.1 -= 1;
        }
        new_pos
    }
    pub fn inbounds_check(self: &PositionTracker, new_pos: GridTile) -> bool {
        new_pos.0 < self.bounds.0 && new_pos.1 < self.bounds.1 && !self.move_lock
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
pub fn process_unit_movement(
    cur_position: &mut PositionTracker,
    unit: &mut UnitInfo,
    map: &mut TerrainGrid,
) {
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
}

pub async fn draw_visible_enemy_units(
    map: &mut TerrainGrid,
    enemy_units: &AI_units,
    textures: &mut Textures,
    cur_position: GridTile,
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
                        .get_texture(unit.unit_type, TextTureType::Default),
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
                    unit.location == cur_position,
                )
                .await;
            }
        }
    }
}

pub async fn draw_player_unit(
    unit: &mut UnitInfo,
    textures: &mut Textures,
    cur_position: GridTile,
) {
    //draw player unit
    paint_tile(
        cur_position,
        TILE_SIZE,
        textures
            .units
            .get_texture(unit.unit_type, TextTureType::Default),
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
    for obj in infra_vector.infr_objects.iter() {
        if obj.detected {
            paint_tile(
                obj.location,
                TILE_SIZE,
                textures.infrastructure.get_infra_texture(obj.infr_type),
                false,
            )
            .await;
        }
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

    //Add infrastructure object and add few test items
    // add them to the map
    let mut infra_vector = InfrastructureContainer::new();
    infra_vector.Init();
    for obj in infra_vector.infr_objects.iter() {
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

    let mut cur_position = PositionTracker::new((2, 3), tile_count);
    let mouse = MouseTracker::new(TILE_SIZE);
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

                //MouseTracker.process_mouse_action();
                process_unit_movement(&mut cur_position, &mut unit, &mut map);

                //draw a grid (TODO: macroquad has draw_grid too, explore using it)
                draw::draw_grid(tile_count, TILE_SIZE).await;

                if get_time() - last_update > speed {
                    last_update = get_time();
                    cur_position.unlock_move();
                }

                draw_terrain(&mut textures, &mut map, tile_count).await;

                draw_player_unit(&mut unit, &mut textures, cur_position.get_position()).await;

                draw_visible_enemy_units(
                    &mut map,
                    &enemy_units,
                    &mut textures,
                    cur_position.get_position(),
                )
                .await;

                draw_infrastructure(&mut textures, &infra_vector).await;
            } //game state
        }

        next_frame().await
    }
}
