use macroquad::prelude::*;
use macroquad::ui::{Skin, hash, root_ui, widgets};

pub enum GameState {
    Menu,
    Game,
    Exit,
}
pub struct MenuObjectPaths {
    window_background: &'static str,
    button_font: &'static str,
    button_background: &'static str,
    button_hover: &'static str,
    button_pressed: &'static str,
}

impl MenuObjectPaths {
    pub fn new() -> MenuObjectPaths
    {
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
