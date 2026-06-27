use std::collections::HashSet;

use crate::defines::{Entity, GridTile};
use crate::draw::Textures;
use crate::game::init_player_units;
use crate::infrastructure::infstrt::InfrastructureContainer;
use crate::map::terrain::TerrainGrid;
use crate::mouse::MouseTracker;
use crate::units::unit::{UnitId, UnitInfo, UnitsContainer, init_enemy_units};

/// All long-lived game state, bundled so it can be passed around as a single
/// reference instead of threading each field through every function.
pub struct GameAssets {
    pub map: TerrainGrid,
    pub player_units_map: UnitsContainer,
    pub enemy_units_map: UnitsContainer,
    pub infr_container: InfrastructureContainer,
    pub textures: Textures,
    pub mouse: MouseTracker,
    pub destroyed_units: Vec<UnitInfo>,
    pub contested_tiles: HashSet<GridTile>,
}

impl GameAssets {
    /// Builds the terrain, units, infrastructure and textures, wiring units and
    /// infrastructure into the map the same way the original startup code did.
    pub async fn new(id_gen: &mut UnitId) -> GameAssets {
        let mut map = TerrainGrid::new("assets/terrain_map.txt");
        let player_units_map = init_player_units(id_gen);

        let mut infr_container = InfrastructureContainer::new();
        infr_container.init();
        for obj in infr_container.infr_objects.iter() {
            map.add_infr(obj.clone());
        }

        let enemy_units_map = init_enemy_units(id_gen);
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
            mouse: MouseTracker::new(),
            destroyed_units: Vec::new(),
            contested_tiles: HashSet::new(),
        }
    }
}
