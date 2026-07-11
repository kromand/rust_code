pub const TILE_SIZE: (f32, f32) = (40.0, 40.0);
pub type PixelOffset = (f32, f32);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct GridTile {
    pub row: u16,
    pub col: u16,
}

impl GridTile {
    pub fn new(row: u16, col: u16) -> Self {
        GridTile { row, col }
    }
}

use strum_macros::Display;

#[derive(Debug, Display, Clone, Copy)]
pub enum TerrainTilesEnum {
    Forest,
    Ocean,
    Lake,
    Mountain,
    GrassTerrain,
    Urban,
    End,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Entity {
    Player,
    Enemy,
}

impl Entity {
    pub fn get_opposite(p: Entity) -> Entity {
        match p {
            Entity::Enemy => Entity::Player,
            Entity::Player => Entity::Enemy,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Display)]
pub enum InfrastructureEnum {
    Factory,
    Mines,
    Airfield,
    Bunker,
    DefensiveObstacles,
    Road,
    End,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Display)]
pub enum UnitTilesEnum {
    Tank,
    Infantry,
    Scout,
    Engineers,
    APC,
    RocketArty,
    Artillery,
    AttackHeli,
    TransportHeli,
    Plane,
    SAM,
    End,
}

impl UnitTilesEnum {
    /// Parses a unit type from its variant name (case-insensitive). Returns
    /// `None` for unknown names and for the `End` sentinel.
    pub fn from_name(name: &str) -> Option<UnitTilesEnum> {
        use UnitTilesEnum::*;
        match name.to_ascii_lowercase().as_str() {
            "tank" => Some(Tank),
            "infantry" => Some(Infantry),
            "scout" => Some(Scout),
            "engineers" => Some(Engineers),
            "apc" => Some(APC),
            "rocketarty" => Some(RocketArty),
            "artillery" => Some(Artillery),
            "attackheli" => Some(AttackHeli),
            "transportheli" => Some(TransportHeli),
            "plane" => Some(Plane),
            "sam" => Some(SAM),
            _ => None,
        }
    }
}

/// Special actions a unit may perform beyond moving and fighting.
/// Which actions a unit has is declared per unit type in `UnitInfo::new`.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display)]
pub enum UnitAction {
    AddMines,
    BuildBunker,
    BuildBridge,
    RangedAttack,
}

#[derive(Clone, Copy, Debug)]
pub enum MoveResult {
    Success,
    InvalidMove,
    UnitDestroyed,
}

pub const AIR_UNITS: [UnitTilesEnum; 3] = [
    UnitTilesEnum::AttackHeli,
    UnitTilesEnum::TransportHeli,
    UnitTilesEnum::Plane,
];

pub const LAND_UNITS: [UnitTilesEnum; 7] = [
    UnitTilesEnum::Tank,
    UnitTilesEnum::Infantry,
    UnitTilesEnum::Scout,
    UnitTilesEnum::Engineers,
    UnitTilesEnum::APC,
    UnitTilesEnum::RocketArty,
    UnitTilesEnum::Artillery,
];

pub fn is_air_unit(unit_type: UnitTilesEnum) -> bool {
    AIR_UNITS.contains(&unit_type)
}
