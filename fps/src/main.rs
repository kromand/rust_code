use macroquad::prelude::*;
// use glam::vec3;

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;
const EYE_HEIGHT: f32 = 1.0;
const PLAYER_RADIUS: f32 = 0.3;

const GRID_SLICES: u32 = 20;
const GRID_SPACING: f32 = 1.0;
/// `draw_grid` is centred on the origin, so it spans ±GRID_HALF on X and Z.
const GRID_HALF: f32 = GRID_SLICES as f32 * GRID_SPACING / 2.0;
const FLOOR_Y: f32 = -0.001;

const CUBE_SIZE: Vec3 = vec3(2., 2., 2.);
const CUBE_CENTERS: [Vec3; 3] = [vec3(0., 1., -6.), vec3(0., 1., 6.), vec3(2., 1., 2.)];

/// Walls are built from cubes this size so the bricks match the scale on the
/// free-standing cubes instead of one texture stretched along a whole side.
const WALL_SEGMENT: f32 = 2.0;

/// The poster hangs on the inner face of the +Z wall, nudged off it slightly
/// so it doesn't z-fight with the bricks. Width follows the image's aspect.
const POSTER_CENTER: Vec3 = vec3(-3.0, 1.1, GRID_HALF - 0.01);
const POSTER_HEIGHT: f32 = 1.2;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

async fn load_linear_texture(path: &str) -> Texture2D {
    let tex = load_texture(path)
        .await
        .unwrap_or_else(|e| panic!("failed to load {path}: {e}"));
    tex.set_filter(FilterMode::Linear);
    tex
}

fn draw_brick_cubes(tex: &Texture2D) {
    for center in CUBE_CENTERS {
        draw_cube(center, CUBE_SIZE, Some(tex), WHITE);
    }
}

/// Centres of the wall cubes ringing the grid. Their inner faces line up with
/// the grid edge; the north/south rows also fill the four corners.
fn wall_centers() -> Vec<Vec3> {
    let offset = GRID_HALF + WALL_SEGMENT / 2.0;
    let y = WALL_SEGMENT / 2.0;
    let count = (2.0 * offset / WALL_SEGMENT).round() as usize + 1;
    let mut centers = Vec::with_capacity(count * 4);
    for i in 0..count {
        let t = -offset + i as f32 * WALL_SEGMENT;
        centers.push(vec3(t, y, -offset));
        centers.push(vec3(t, y, offset));
        if i > 0 && i < count - 1 {
            centers.push(vec3(-offset, y, t));
            centers.push(vec3(offset, y, t));
        }
    }
    centers
}

/// Upright textured quad for the poster, facing into the room (towards -Z).
/// Seen from inside, +X is on the viewer's left, so the image's left edge
/// (u = 0) goes at the larger X.
fn poster_mesh(tex: Texture2D) -> Mesh {
    let half_w = POSTER_HEIGHT * tex.width() / tex.height() / 2.0;
    let half_h = POSTER_HEIGHT / 2.0;
    let (x, y, z) = (POSTER_CENTER.x, POSTER_CENTER.y, POSTER_CENTER.z);
    Mesh {
        vertices: vec![
            Vertex::new(x + half_w, y + half_h, z, 0.0, 0.0, WHITE),
            Vertex::new(x - half_w, y + half_h, z, 1.0, 0.0, WHITE),
            Vertex::new(x - half_w, y - half_h, z, 1.0, 1.0, WHITE),
            Vertex::new(x + half_w, y - half_h, z, 0.0, 1.0, WHITE),
        ],
        indices: vec![0, 1, 2, 0, 2, 3],
        texture: Some(tex),
    }
}

fn draw_walls(tex: &Texture2D, centers: &[Vec3]) {
    let size = Vec3::splat(WALL_SEGMENT);
    for &center in centers {
        draw_cube(center, size, Some(tex), WHITE);
    }
}

/// Marble floor covering the whole grid. Sits just below y = 0 so the grid
/// lines draw on top of it instead of z-fighting with it.
fn draw_floor(tex: &Texture2D) {
    draw_plane(
        vec3(0.0, FLOOR_Y, 0.0),
        vec2(GRID_HALF, GRID_HALF),
        Some(tex),
        WHITE,
    );
}

/// True if either of the two keys bound to the same action is held.
fn either_down(a: KeyCode, b: KeyCode) -> bool {
    is_key_down(a) || is_key_down(b)
}

/// Desired horizontal movement from the arrow keys or WASD. `front` is flattened
/// onto the ground plane so looking up or down never moves the player vertically.
fn movement_input(front: Vec3, right: Vec3) -> Vec3 {
    let forward = vec3(front.x, 0.0, front.z).normalize_or_zero();
    let strafe = vec3(right.x, 0.0, right.z).normalize_or_zero();
    let mut dir = Vec3::ZERO;
    if either_down(KeyCode::Up, KeyCode::W) {
        dir += forward;
    }
    if either_down(KeyCode::Down, KeyCode::S) {
        dir -= forward;
    }
    if either_down(KeyCode::Left, KeyCode::A) {
        dir -= strafe;
    }
    if either_down(KeyCode::Right, KeyCode::D) {
        dir += strafe;
    }
    dir.normalize_or_zero() * MOVE_SPEED
}

/// True if a player standing at `pos` (radius PLAYER_RADIUS) overlaps any cube
/// in the XZ plane.
fn collides(pos: Vec3) -> bool {
    let half = CUBE_SIZE / 2.0 + Vec3::splat(PLAYER_RADIUS);
    CUBE_CENTERS.iter().any(|c| {
        (pos.x - c.x).abs() < half.x && (pos.z - c.z).abs() < half.z
    })
}

/// True if a player standing at `pos` (radius PLAYER_RADIUS) stays fully on
/// the floor grid.
fn inside_grid(pos: Vec3) -> bool {
    let limit = GRID_HALF - PLAYER_RADIUS;
    pos.x.abs() <= limit && pos.z.abs() <= limit
}

/// True if the player can't stand at `pos`: it hits a cube or leaves the grid.
fn blocked(pos: Vec3) -> bool {
    collides(pos) || !inside_grid(pos)
}

/// Applies `step` one axis at a time so the player slides along walls instead
/// of stopping dead, and pins the eye at a fixed height above the floor.
fn move_player(position: Vec3, step: Vec3) -> Vec3 {
    let mut pos = position;
    let try_x = vec3(pos.x + step.x, pos.y, pos.z);
    if !blocked(try_x) {
        pos = try_x;
    }
    let try_z = vec3(pos.x, pos.y, pos.z + step.z);
    if !blocked(try_z) {
        pos = try_z;
    }
    pos.y = EYE_HEIGHT;
    pos
}

/// Locks and hides the cursor for mouse-look, or releases and shows it.
fn apply_cursor_grab(grabbed: bool) {
    set_cursor_grab(grabbed);
    show_mouse(!grabbed);
}

/// Toggles the cursor grab on <TAB> and returns the new grab state.
fn update_cursor_grab(grabbed: bool) -> bool {
    if !is_key_pressed(KeyCode::Tab) {
        return grabbed;
    }
    let grabbed = !grabbed;
    apply_cursor_grab(grabbed);
    grabbed
}

/// Turns the mouse movement into new yaw/pitch angles. Pitch is clamped short
/// of ±90° so `front` never becomes parallel to the world up vector.
fn update_look_angles(yaw: f32, pitch: f32, mouse_delta: Vec2, delta: f32) -> (f32, f32) {
    let yaw = yaw + mouse_delta.x * delta * LOOK_SPEED;
    let pitch = (pitch + mouse_delta.y * delta * -LOOK_SPEED).clamp(-1.5, 1.5);
    (yaw, pitch)
}

/// Camera `(front, right, up)` vectors for the given yaw/pitch angles.
fn camera_basis(yaw: f32, pitch: f32, world_up: Vec3) -> (Vec3, Vec3, Vec3) {
    let front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let right = front.cross(world_up).normalize();
    let up = right.cross(front).normalize();
    (front, right, up)
}

#[macroquad::main(conf)]
async fn main() {
    let brick_tex = load_linear_texture("assets/brick_wall_albedo.png").await;
    let marble_tex = load_linear_texture("assets/white_marble_albedo.png").await;
    let poster = poster_mesh(load_linear_texture("assets/horror_poster_albedo.png").await);
    let walls = wall_centers();

    let world_up = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let (mut front, mut right, mut up) = camera_basis(yaw, pitch, world_up);

    let mut position = vec3(0.0, EYE_HEIGHT, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = true;
    apply_cursor_grab(grabbed);

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        grabbed = update_cursor_grab(grabbed);

        position = move_player(position, movement_input(front, right));

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;

        last_mouse_position = mouse_position;

        if grabbed {
            (yaw, pitch) = update_look_angles(yaw, pitch, mouse_delta, delta);
            (front, right, up) = camera_basis(yaw, pitch, world_up);
        }

        clear_background(LIGHTGRAY);

        // Going 3d!

        set_camera(&Camera3D {
            position,
            up,
            target: position + front,
            ..Default::default()
        });

        draw_floor(&marble_tex);
        draw_grid(GRID_SLICES, GRID_SPACING, BLACK, GRAY);

        draw_brick_cubes(&brick_tex);
        draw_walls(&brick_tex, &walls);
        draw_mesh(&poster);

        // Back to screen space
        set_default_camera();

        next_frame().await
    }
}

/// Animated yellow line from macroquad's first-person example. Currently unused;
/// to show it, keep a `RayDemo` in `main`, call `update()` while the mouse is
/// grabbed and `draw()` after `set_camera`.
#[allow(dead_code)]
struct RayDemo {
    x: f32,
    switch: bool,
}

#[allow(dead_code)]
impl RayDemo {
    const BOUNDS: f32 = 8.0;
    const STEP: f32 = 0.04;

    fn new() -> Self {
        Self { x: 0.0, switch: false }
    }

    /// Slides the floor end of the line back and forth between ±BOUNDS.
    fn update(&mut self) {
        self.x += if self.switch { Self::STEP } else { -Self::STEP };
        if self.x >= Self::BOUNDS || self.x <= -Self::BOUNDS {
            self.switch = !self.switch;
        }
    }

    fn draw(&self) {
        draw_line_3d(
            vec3(self.x, 0.0, self.x),
            vec3(5.0, 5.0, 5.0),
            Color::new(1.0, 1.0, 0.0, 1.0),
        );
    }
}

/// On-screen debug text (title, mouse position, grab state). Currently unused;
/// call it after `set_default_camera()` to show it.
#[allow(dead_code)]
fn draw_debug_text(mouse_position: Vec2, grabbed: bool) {
    draw_text("First Person Camera", 10.0, 20.0, 30.0, BLACK);
    draw_text(
        format!("X: {} Y: {}", mouse_position.x, mouse_position.y).as_str(),
        10.0,
        48.0 + 18.0,
        30.0,
        BLACK,
    );
    draw_text(
        format!("Press <TAB> to toggle mouse grab: {grabbed}").as_str(),
        10.0,
        48.0 + 42.0,
        30.0,
        BLACK,
    );
}
