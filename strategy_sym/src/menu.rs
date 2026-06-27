use macroquad::prelude::*;
use macroquad::ui::{Skin, Ui, hash, root_ui, widgets};

use crate::defines::{GridTile, InfrastructureEnum, TILE_SIZE};
use crate::game_assets::GameAssets;
use crate::infrastructure::infstrt::InfrastructureContainer;
use crate::map::terrain::TerrainGrid;
use crate::mouse::MouseTracker;
use crate::units::unit::UnitsContainer;

#[derive(PartialEq, Eq)]
pub enum GameState {
    Menu,
    Game,
    Exit,
}

pub enum MenuType {
    Main,
    Factory,
    Airfield,
    Unit,
}
pub struct MenuObjectPaths {
    window_background: &'static str,
    button_font: &'static str,
    button_background: &'static str,
    button_hover: &'static str,
    button_pressed: &'static str,
}

impl MenuObjectPaths {
    pub fn new() -> MenuObjectPaths {
        MenuObjectPaths {
            window_background: "assets/window_background.png",
            button_font: "assets/HTOWERT.TTF",
            button_background: "assets/button_background.png",
            button_hover: "assets/button_hovered_background.png",
            button_pressed: "assets/button_clicked_background.png",
        }
    }
}

pub async fn create_menu_skin(ui_paths: &MenuObjectPaths) -> Result<Box<Skin>, macroquad::Error> {
    let window_background = load_image(ui_paths.window_background).await?;
    let button_background = load_image(ui_paths.button_background).await?;
    let button_clicked_background = load_image(ui_paths.button_pressed).await?;
    let button_clicked_hover = load_image(ui_paths.button_hover).await?;
    let font = load_file(ui_paths.button_font).await?;

    let window_style = root_ui()
        .style_builder()
        .background(window_background)
        .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
        .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
        .build();

    let button_style = root_ui()
        .style_builder()
        .background(button_background)
        .background_clicked(button_clicked_background)
        .background_hovered(button_clicked_hover)
        .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
        .font(&font)
        .unwrap()
        .text_color(WHITE)
        .font_size(64)
        .build();

    let label_style = root_ui()
        .style_builder()
        .font(&font)
        .unwrap()
        .text_color(WHITE)
        .font_size(28)
        .build();

    Ok(Box::new(Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    }))
}

pub async fn initialize_menu(menu_skin: &Box<Skin>) {
    root_ui().push_skin(&menu_skin);
}

pub fn clear_ui_skin() {
    root_ui().pop_skin();
}

fn popup_item_count(
    selection: &MenuType,
    terrain_grid: &TerrainGrid,
    grid_tile: GridTile,
    has_factory: bool,
    has_airfield: bool,
    player_units: &UnitsContainer,
) -> usize {
    match selection {
        MenuType::Main => {
            let mut count = 2;
            if has_factory {
                count += 1;
            }
            if has_airfield {
                count += 1;
            }
            count
        }
        MenuType::Factory => {
            let allowed_units = terrain_grid.get_factory_allowed_units(grid_tile);
            if !allowed_units.is_empty() {
                allowed_units.len() + 1
            } else {
                1
            }
        }
        MenuType::Airfield => {
            let allowed_units = terrain_grid.get_airfield_allowed_units(grid_tile);
            if !allowed_units.is_empty() {
                allowed_units.len() + 1
            } else {
                1
            }
        }
        MenuType::Unit => match player_units.units_by_tile.get(&grid_tile) {
            Some(unit_stack) => {
                // one row per unit + one row per action + the Cancel button
                let mut count = 1;
                for unit in unit_stack.units.values() {
                    count += 1;
                    if let Some(actions) = &unit.actions {
                        count += actions.len();
                    }
                }
                count
            }
            // the lone "No units" button
            None => 1,
        },
    }
}

fn render_popup_menu_content(
    ui: &mut Ui,
    mouse: &mut MouseTracker,
    selection: &mut MenuType,
    terrain_grid: &mut TerrainGrid,
    grid_tile: GridTile,
    has_factory: bool,
    has_airfield: bool,
    player_units: &UnitsContainer,
    infr_container: &mut InfrastructureContainer,
) {
    match selection {
        MenuType::Main => render_popup_main_menu(ui, selection, mouse, has_factory, has_airfield),
        MenuType::Factory => {
            render_popup_factory_menu(ui, selection, mouse, terrain_grid, grid_tile)
        }
        MenuType::Airfield => {
            render_popup_airfield_menu(ui, selection, mouse, terrain_grid, grid_tile)
        }
        MenuType::Unit => render_popup_unit_menu(
            ui,
            selection,
            mouse,
            player_units,
            terrain_grid,
            infr_container,
            grid_tile,
        ),
    }
}

fn render_popup_main_menu(
    ui: &mut Ui,
    selection: &mut MenuType,
    mouse: &mut MouseTracker,
    has_factory: bool,
    has_airfield: bool,
) {
    let mut y_offset = 30.0;
    if has_factory {
        if ui.button(vec2(10.0, y_offset), "Factory") {
            *selection = MenuType::Factory;
        }
        y_offset += 20.0;
    }
    if has_airfield {
        if ui.button(vec2(10.0, y_offset), "Airfield") {
            *selection = MenuType::Airfield;
        }
        y_offset += 20.0;
    }
    if ui.button(vec2(10.0, y_offset), "Units") {
        *selection = MenuType::Unit;
    }
    y_offset += 20.0;
    if ui.button(vec2(10.0, y_offset), "Cancel") {
        mouse.set_popup_visible(false);
    }
}

fn render_popup_factory_menu(
    ui: &mut Ui,
    selection: &mut MenuType,
    mouse: &mut MouseTracker,
    terrain_grid: &mut TerrainGrid,
    grid_tile: GridTile,
) {
    let allowed_units = terrain_grid.get_factory_allowed_units(grid_tile);
    if !allowed_units.is_empty() {
        let mut y_offset = 30.0;
        for &unit_type in &allowed_units {
            if ui.button(vec2(10.0, y_offset), unit_type.to_string()) {
                terrain_grid.enqueue_unit_in_factory(grid_tile, unit_type);
                mouse.set_popup_visible(false);
                *selection = MenuType::Main;
            }
            y_offset += 20.0;
        }
        if ui.button(vec2(10.0, y_offset), "Cancel") {
            mouse.set_popup_visible(false);
            *selection = MenuType::Main;
        }
    } else if ui.button(vec2(10.0, 30.0), "Cancel") {
        mouse.set_popup_visible(false);
        *selection = MenuType::Main;
    }
}

fn render_popup_airfield_menu(
    ui: &mut Ui,
    selection: &mut MenuType,
    mouse: &mut MouseTracker,
    terrain_grid: &mut TerrainGrid,
    grid_tile: GridTile,
) {
    let allowed_units = terrain_grid.get_airfield_allowed_units(grid_tile);
    if !allowed_units.is_empty() {
        let mut y_offset = 30.0;
        for &unit_type in &allowed_units {
            if ui.button(vec2(10.0, y_offset), unit_type.to_string()) {
                terrain_grid.enqueue_unit_in_airfield(grid_tile, unit_type);
                mouse.set_popup_visible(false);
                *selection = MenuType::Main;
            }
            y_offset += 20.0;
        }
        if ui.button(vec2(10.0, y_offset), "Cancel") {
            mouse.set_popup_visible(false);
            *selection = MenuType::Main;
        }
    } else if ui.button(vec2(10.0, 30.0), "Cancel") {
        mouse.set_popup_visible(false);
        *selection = MenuType::Main;
    }
}

fn render_popup_unit_menu(
    ui: &mut Ui,
    selection: &mut MenuType,
    mouse: &mut MouseTracker,
    player_units: &UnitsContainer,
    terrain_grid: &mut TerrainGrid,
    infr_container: &mut InfrastructureContainer,
    grid_tile: GridTile,
) {
    if let Some(unit_stack) = player_units.units_by_tile.get(&grid_tile) {
        let mut y_offset = 30.0;
        for unit in unit_stack.units.values() {
            if ui.button(vec2(10.0, y_offset), unit.unit_name.as_str()) {
                // TODO: perhaps select the unit or something
                mouse.set_popup_visible(false);
                *selection = MenuType::Main;
            }
            y_offset += 20.0;
            // List this unit's available special actions as indented buttons.
            if let Some(actions) = &unit.actions {
                let mut sorted_actions: Vec<_> = actions.iter().copied().collect();
                sorted_actions.sort();
                for action in sorted_actions {
                    if ui.button(vec2(20.0, y_offset), action.to_string()) {
                        unit.perform_action(action, terrain_grid, infr_container, grid_tile);
                        mouse.set_popup_visible(false);
                        *selection = MenuType::Main;
                    }
                    y_offset += 20.0;
                }
            }
        }
        if ui.button(vec2(10.0, y_offset), "Cancel") {
            mouse.set_popup_visible(false);
            *selection = MenuType::Main;
        }
    } else {
        if ui.button(vec2(10.0, 30.0), "No units") {
            mouse.set_popup_visible(false);
            *selection = MenuType::Main;
        }
    }
}

pub async fn show_menu(game_state: &mut GameState) {
    clear_background(GRAY);
    let window_size = vec2(370.0, 320.0);

    root_ui().window(
        hash!(),
        vec2(
            screen_width() / 2.0 - window_size.x / 2.0,
            screen_height() / 2.0 - window_size.y / 2.0,
        ),
        window_size,
        |ui| {
            ui.label(vec2(80.0, -34.0), "Main Menu");
            if ui.button(vec2(65.0, 25.0), "Play") {
                *game_state = GameState::Game;
            }
            if ui.button(vec2(65.0, 125.0), "Quit") {
                *game_state = GameState::Exit;
            }
        },
    );
}

pub fn show_popup_menu(game_assets: &mut GameAssets, selection: &mut MenuType) {
    let GameAssets {
        mouse,
        map: terrain_grid,
        player_units_map: player_units,
        infr_container,
        ..
    } = game_assets;
    if mouse.is_popup_visible() {
        let location = vec2(mouse.popup_position().0, mouse.popup_position().1);
        if mouse.popup_id().is_none() {
            mouse.set_popup_id(hash!());
        }

        let grid_col = (mouse.popup_position().0 / TILE_SIZE.0) as u16;
        let grid_row = (mouse.popup_position().1 / TILE_SIZE.1) as u16;
        let grid_tile = GridTile::new(grid_row, grid_col);
        let has_factory = terrain_grid.has_infrastructure(grid_tile, InfrastructureEnum::Factory);
        let has_airfield = terrain_grid.has_infrastructure(grid_tile, InfrastructureEnum::Airfield);

        if mouse.popup_position_changed() {
            *selection = MenuType::Main;
            mouse.reset_popup_position_changed();
        }

        let item_count = popup_item_count(
            selection,
            terrain_grid,
            grid_tile,
            has_factory,
            has_airfield,
            player_units,
        );
        let window_size = vec2(100.0, 40.0 + item_count as f32 * 20.0 + 10.0);

        // Store popup bounds for click detection
        mouse.set_popup_bounds(location.x, location.y, window_size.x, window_size.y);

        root_ui().move_window(mouse.popup_id().unwrap(), location);
        root_ui().window(mouse.popup_id().unwrap(), location, window_size, |ui| {
            ui.label(vec2(0.0, 10.0), "Selection:");
            render_popup_menu_content(
                ui,
                mouse,
                selection,
                terrain_grid,
                grid_tile,
                has_factory,
                has_airfield,
                player_units,
                infr_container,
            );
        });
    }
}
