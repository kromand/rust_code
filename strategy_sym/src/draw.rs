use crate::defines::*;
use macroquad::prelude::*;

pub async fn paint_tile(tile: GridTile, size: (f32, f32), texture: &Texture2D,flip:bool) {
    let param = DrawTextureParams {
        dest_size: Some(Vec2 {
            x: size.0,
            y: size.1,
        }),
        source: None,
        rotation: 0.0,
        flip_x: flip,
        flip_y: false,
        pivot: None,
    };
    draw_texture_ex(
        texture,
        (tile.0 as f32) * size.0,
        (tile.1 as f32) * size.1,
        LIGHTGRAY,
        param,
    );
}

pub async fn draw_grid(tile_count: GridTile, tile_size: (f32, f32)) {
    //draw horizontal
    for i in 1..tile_count.1 {
        draw_line(
            0.0,
            tile_size.1 * i as f32,
            screen_width(),
            tile_size.1 * i as f32,
            1.,
            BLACK,
        );
    }
    //draw vertical
    for i in 1..tile_count.0 {
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
pub async fn draw_health_bar(tile_size: (f32, f32), 
        tile: GridTile, 
        health_ratio: f32, 
        ent:Entity,
        both_players_present:bool) {
    let col = match ent 
    {
        Entity::Player =>    BLUE,
        Entity::AI => RED
    };
    // if opposing units are being drawn on the same tile, draw AI health bar 
    // with offset so the player health bar will be visible below AI's
    let bar_thickness = 2.0;
    let offset = match ent 
    {
        Entity::Player =>    1.0,
        Entity::AI if both_players_present => 1.0 + bar_thickness ,
        Entity::AI => 1.0
    };

    // add one tile to y tile cord so that health bar appears at the bottom
    //substracting y from the pixel count makes the health bar appear above tile boundary
    
    draw_line(
        tile_size.0 * (tile.0 as f32),
        tile_size.1 * (tile.1 + 1) as f32 - offset,
        tile_size.0 * (tile.0 as f32) + tile_size.1 * health_ratio,
        tile_size.1 * (tile.1 + 1) as f32 - offset,
        bar_thickness,
        col,
    );
}
