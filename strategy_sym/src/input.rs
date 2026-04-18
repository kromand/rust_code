

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