use macroquad::prelude::*;

use crate::defines::*;
use crate::units::unit::*;
use crate::utils;

pub struct MouseTracker {
    start_cursor_position: Option<PixelOffset>,
    end_cursor_position: Option<PixelOffset>,
    unitid: usize,
    show_popup: bool,
    popup_position: PixelOffset,
    popup_position_changed: bool,
    popup_id: Option<u64>,
    popup_bounds: Option<(f32, f32, f32, f32)>, // (x, y, width, height)
}

impl MouseTracker {
    pub fn new() -> MouseTracker {
        MouseTracker {
            end_cursor_position: None,
            start_cursor_position: None,
            unitid: 0,
            show_popup: false,
            popup_position: (0.0, 0.0),
            popup_position_changed: false,
            popup_id: None,
            popup_bounds: None,
        }
    }

    pub fn process_mouse_action(self: &mut MouseTracker, units: &UnitsContainer) {
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_tile = self.get_cursor_pointed_tile();

        if is_mouse_button_down(MouseButton::Left) {
            // Check if click is outside popup bounds and hide it
            if self.show_popup {
                if let Some((px, py, pw, ph)) = self.popup_bounds {
                    if mouse_x < px || mouse_x > px + pw || mouse_y < py || mouse_y > py + ph {
                        self.show_popup = false;
                    }
                }
            }

            if let Some(u) = units.get_units_at(mouse_tile) {
                if !u.is_empty() && self.start_cursor_position.is_none() {
                    //get first unit id from the tile and set it as selected, also set start cursor position for dragging
                    self.unitid = *u.iter().next().unwrap().0;
                    self.end_cursor_position = None;
                    self.start_cursor_position = Some((mouse_x, mouse_y));
                }
            }
        } else {
            if self.end_cursor_position.is_none() {
                self.end_cursor_position = Some((mouse_x, mouse_y));
            } else if self.end_cursor_position.is_some() {
                self.end_cursor_position = None;
                self.start_cursor_position = None;
            }
        }

        if is_mouse_button_down(MouseButton::Right) {
            let new_popup_position = (mouse_x, mouse_y);
            self.popup_position_changed =
                !self.show_popup || self.popup_position != new_popup_position;
            self.popup_position = new_popup_position;
            self.show_popup = true;
        }
        if is_mouse_button_down(MouseButton::Middle) {}
    }

    pub fn get_selected_unit_id(self: &MouseTracker) -> usize {
        self.unitid
    }

    pub fn is_popup_visible(self: &MouseTracker) -> bool {
        self.show_popup
    }

    pub fn popup_position(self: &MouseTracker) -> PixelOffset {
        self.popup_position
    }

    pub fn popup_id(self: &MouseTracker) -> Option<u64> {
        self.popup_id
    }

    pub fn set_popup_id(self: &mut MouseTracker, id: u64) {
        self.popup_id = Some(id);
    }

    pub fn popup_position_changed(self: &MouseTracker) -> bool {
        self.popup_position_changed
    }

    pub fn reset_popup_position_changed(self: &mut MouseTracker) {
        self.popup_position_changed = false;
    }

    pub fn set_popup_visible(self: &mut MouseTracker, visible: bool) {
        self.show_popup = visible;
    }

    pub fn set_popup_bounds(self: &mut MouseTracker, x: f32, y: f32, width: f32, height: f32) {
        self.popup_bounds = Some((x, y, width, height));
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

    pub fn get_start_cursor_tile(self: &MouseTracker) -> Option<GridTile> {
        if let Some(pos) = self.start_cursor_position {
            return Some(utils::conv::pixel_offset_to_grid(pos));
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
