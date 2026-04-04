pub const TILE_SIZE: (f32, f32) = (40.0, 40.0);
pub type GridTile = (i16, i16);

#[derive(Clone, Copy)]
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
    pub fn get_opposite(p: Entity) -> Entity
    {
        match p {
           Entity::AI => Entity::Player, 
           Entity::Player => Entity::AI,   
        }
    }
}

#[derive(Clone, Copy,Eq, PartialEq, Hash,Debug)]
pub enum InfrastructureEnum {
    Fatory,
    Mines,
    Bunkers,
    DefensiveObstacles,
    Road,
    Airfield,
    End,
} 

#[derive(Clone, Copy,Debug)]
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
    SAM,
    End,
} 