use std::collections::HashSet;

use crate::defines::{Entity, GridTile};
use crate::draw::Textures;
use crate::game::init_player_units;
use crate::infrastructure::infstrt::InfrastructureContainer;
use crate::map::terrain::TerrainGrid;
use crate::units::unit::{UnitId, UnitInfo, UnitsContainer, init_enemy_units};

struct PlayerResources {
    pub cash: i32,
    pub monthly_income: i32,
    pub monthly_unit_cost: i32,
    pub monthly_infr_cost: i32,
}

impl PlayerResources {
    pub fn new() -> PlayerResources {
        PlayerResources {
            cash: 1000,
            monthly_income: 100,
            monthly_unit_cost: 0,
            monthly_infr_cost: 0,
        }
    }
}

/// All long-lived game state, bundled so it can be passed around as a single
/// reference instead of threading each field through every function.
pub struct GameAssets {
    pub map: TerrainGrid,
    pub player_units_map: UnitsContainer,
    pub enemy_units_map: UnitsContainer,
    pub infr_container: InfrastructureContainer,
    pub textures: Textures,
    pub destroyed_units: Vec<UnitInfo>,
    pub contested_tiles: HashSet<GridTile>,
    pub id_gen: UnitId,
}

impl GameAssets {
    /// Builds the terrain, units, infrastructure and textures, wiring units and
    /// infrastructure into the map the same way the original startup code did.
    pub async fn new() -> GameAssets {
        let mut id_gen = UnitId::new();
        let mut map = TerrainGrid::new("assets/terrain_map.txt");
        let player_units_map = init_player_units(&mut id_gen);

        let mut infr_container = InfrastructureContainer::new();
        infr_container.init();
        for obj in infr_container.infr_objects.iter() {
            map.add_infr(obj.clone());
        }

        let enemy_units_map = init_enemy_units(&mut id_gen);
        for (_, stack) in &enemy_units_map.units_by_tile {
            for (unit_id, unit) in &stack.units {
                map.add_hidden_unit(*unit_id, unit.location, Entity::Enemy);
            }
        }

        let textures = Textures::new().await.expect("Failed to load textures");

        GameAssets {
            map,
            player_units_map,
            enemy_units_map,
            infr_container,
            textures,
            destroyed_units: Vec::new(),
            contested_tiles: HashSet::new(),
            id_gen,
        }
    }
}
