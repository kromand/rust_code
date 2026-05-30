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
#[derive(Clone, Copy)]
pub enum Entity {
    Player,
    AI,
}

impl Entity {
    pub fn get_opposite(p: Entity) -> Entity {
        match p {
            Entity::AI => Entity::Player,
            Entity::Player => Entity::AI,
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
