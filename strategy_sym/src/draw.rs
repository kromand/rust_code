use crate::defines::*;
use crate::infrastructure::infstrt::*;
use crate::map::terrain::*;
use crate::units::units::*;
use macroquad::prelude::*;

// ---------------------------------------------------------------------------
// Texture bundle — owns all loaded rendering assets for the current session
// ---------------------------------------------------------------------------

pub struct Textures {
    pub terrain: Box<TerrainTiles>,
    pub units: Box<AnimateUnit>,
    pub infrastructure: Box<InfrastructureTextures>,
}

impl Textures {
    pub async fn new() -> Result<Textures, macroquad::Error> {
        Ok(Textures {
            terrain: load_terrain_textures().await?,
            units: AnimateUnit::new().await?,
            infrastructure: InfrastructureTextures::new().await?,
        })
    }
}

// ---------------------------------------------------------------------------
// Scene-level draw calls
// ---------------------------------------------------------------------------

pub async fn draw_terrain(textures: &mut Textures, map: &mut TerrainGrid, tile_count: GridTile) {
    for c in 0..tile_count.col {
        for r in 0..tile_count.row {
            let tile = GridTile::new(r, c);
            if let Some(t_type) = map.get_terrain_at(tile) {
                paint_tile(
                    tile,
                    TILE_SIZE,
                    textures.terrain.get_tile_texture(t_type),
                    false,
                )
                .await;
            }
        }
    }
}

pub async fn draw_infrastructure(
    textures: &mut Textures,
    infra_container: &InfrastructureContainer,
) {
    for infr_arc in infra_container.infr_objects.iter() {
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

pub async fn draw_player_unit(unit: &UnitInfo, textures: &mut Textures, position: GridTile) {
    let flip = matches!(unit.player_id, Entity::AI);
    paint_tile(
        position,
        TILE_SIZE,
        textures.units.get_texture(
            unit.unit_type,
            health_to_texture_type(unit.health / unit.max_health),
        ),
        flip,
    )
    .await;

    draw_health_bar(
        TILE_SIZE,
        position,
        unit.get_health_bar(),
        Entity::Player,
        false,
    )
    .await;
}

pub async fn draw_player_units(
    textures: &mut Textures,
    player_units_map: &PlayerUnits,
    exclude_tile: Option<GridTile>,
) {
    for (tile, unit_stack) in &player_units_map.units_by_tile {
        if exclude_tile == Some(*tile) {
            continue;
        }
        for unit in unit_stack.units.values() {
            draw_player_unit(unit, textures, unit.location).await;
        }
    }
}

pub async fn draw_visible_enemy_units(
    map: &mut TerrainGrid,
    enemy_units: &AiUnits,
    textures: &mut Textures,
    enemy_units_present: bool,
) {
    for (_, unit_ids) in &map.visible_units_per_tile {
        for unit_id in unit_ids {
            if let Some(unit) = enemy_units.units.get(unit_id) {
                let flip = matches!(unit.player_id, Entity::AI);
                paint_tile(
                    unit.location,
                    TILE_SIZE,
                    textures.units.get_texture(
                        unit.unit_type,
                        health_to_texture_type(unit.health / unit.max_health),
                    ),
                    flip,
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

// ---------------------------------------------------------------------------
// Tile-level primitives
// ---------------------------------------------------------------------------

pub async fn paint_tile(tile: GridTile, size: (f32, f32), texture: &Texture2D, flip: bool) {
    draw_texture_ex(
        texture,
        (tile.col as f32) * size.0,
        (tile.row as f32) * size.1,
        LIGHTGRAY,
        DrawTextureParams {
            dest_size: Some(Vec2 {
                x: size.0,
                y: size.1,
            }),
            flip_x: flip,
            ..Default::default()
        },
    );
}

pub async fn paint_tile_at_pixel(
    pix_location: PixelOffset,
    size: (f32, f32),
    texture: &Texture2D,
    flip: bool,
) {
    draw_texture_ex(
        texture,
        pix_location.0,
        pix_location.1,
        LIGHTGRAY,
        DrawTextureParams {
            dest_size: Some(Vec2 {
                x: size.0,
                y: size.1,
            }),
            flip_x: flip,
            ..Default::default()
        },
    );
}

pub async fn draw_grid(tile_count: GridTile, tile_size: (f32, f32)) {
    for i in 1..tile_count.row {
        draw_line(
            0.0,
            tile_size.1 * i as f32,
            screen_width(),
            tile_size.1 * i as f32,
            1.,
            BLACK,
        );
    }
    for i in 1..tile_count.col {
        draw_line(
            tile_size.0 * i as f32,
            0.0,
            tile_size.0 * i as f32,
            screen_height(),
            1.,
            BLACK,
        );
    }
}

pub async fn draw_health_bar(
    tile_size: (f32, f32),
    tile: GridTile,
    health_ratio: f32,
    ent: Entity,
    both_players_present: bool,
) {
    let col = match ent {
        Entity::Player => BLUE,
        Entity::AI => RED,
    };
    let bar_thickness = 2.0;
    let offset = match ent {
        Entity::Player => 1.0,
        Entity::AI if both_players_present => 1.0 + bar_thickness,
        Entity::AI => 1.0,
    };
    draw_line(
        tile_size.0 * (tile.col as f32),
        tile_size.1 * (tile.row + 1) as f32 - offset,
        tile_size.0 * (tile.col as f32) + tile_size.1 * health_ratio,
        tile_size.1 * (tile.row + 1) as f32 - offset,
        bar_thickness,
        col,
    );
}
