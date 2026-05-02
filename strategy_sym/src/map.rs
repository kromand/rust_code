pub mod terrain {
    use std::collections::HashMap;
    use std::collections::HashSet;

    use crate::defines::*;
    use crate::infrastructure::infstrt;
    use crate::random::random_nums;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::sync::{Arc, Mutex};

    use macroquad::prelude::*;
    struct TileInfo {
        terrain_type: TerrainTilesEnum,
        location: GridTile,
        visible_units: [HashSet<usize>; 2],
        hidden_units: [HashSet<usize>; 2],
        infrastruct: HashMap<InfrastructureEnum, Arc<Mutex<infstrt::InfrObject>>>,
    }
    pub struct TerrainGrid {
        map: Vec<Vec<TileInfo>>,
        probability_generation: random_nums::prob_gen,
        pub visible_units_per_tile: HashMap<GridTile, HashSet<usize>>,
    }
    impl TerrainGrid {
        pub fn new(map_file: &str) -> TerrainGrid {
            let map_file = File::open(map_file).unwrap();
            let reader = BufReader::new(map_file);
            let mut m = Vec::<Vec<TileInfo>>::new();

            for (y, line_result) in reader.lines().enumerate() {
                let row: Vec<TileInfo> = line_result
                    .unwrap()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| TileInfo {
                        terrain_type: TerrainGrid::char_code_terrain_enum(c),
                        location: (y as u16, x as u16),
                        visible_units: [HashSet::<usize>::new(), HashSet::<usize>::new()],
                        hidden_units: [HashSet::<usize>::new(), HashSet::<usize>::new()],
                        infrastruct: HashMap::<InfrastructureEnum, Arc<Mutex<infstrt::InfrObject>>>::new(),
                    })
                    .collect();
                m.push(row);
            }

            TerrainGrid {
                map: m,
                probability_generation: random_nums::prob_gen::new(100),
                visible_units_per_tile: HashMap::<GridTile, HashSet<usize>>::new(),
            }
        }
        pub fn get_hidden_units_count(self: &TerrainGrid, tile: GridTile, ent: Entity) -> usize {
            let mut size = 0;
            if (tile.1 as usize) < self.map.len()
                && (tile.0 as usize) < self.map[tile.1 as usize].len()
            {
                size = self.map[tile.1 as usize][tile.0 as usize].hidden_units[ent as usize].len();
            }
            size
        }

        pub fn get_owned_units_count(self: &TerrainGrid, tile: GridTile, ent: Entity) -> usize {
            let mut size = 0;
            if (tile.1 as usize) < self.map.len()
                && (tile.0 as usize) < self.map[tile.1 as usize].len()
            {
                size = self.map[tile.1 as usize][tile.0 as usize].visible_units[ent as usize].len();
            }
            size
        }

        pub fn get_title_for_cord(self: &mut TerrainGrid, tile: GridTile) -> Option<&mut TileInfo> {
            if (tile.1 as usize) < self.map.len()
                && (tile.0 as usize) < self.map[tile.1 as usize].len()
            {
                return Some(&mut self.map[tile.1 as usize][tile.0 as usize]);
            }
            None
        }

        pub fn get_titletype_for_cord(
            self: &mut TerrainGrid,
            tile: GridTile,
        ) -> Option<TerrainTilesEnum> {
            if let Some(tile_info) = self.get_title_for_cord(tile) {
                return Some(tile_info.terrain_type);
            }
            None
        }
        pub fn char_code_terrain_enum(c: char) -> TerrainTilesEnum {
            match c {
                'F' => TerrainTilesEnum::Forest,
                'O' => TerrainTilesEnum::Ocean,
                'L' => TerrainTilesEnum::Lake,
                'M' => TerrainTilesEnum::Mountain,
                'G' => TerrainTilesEnum::GrassTerrain,
                'U' => TerrainTilesEnum::Urban,
                _ => {
                    unreachable!("Inavlid map file char")
                }
            }
        }
        pub fn add_unit(unit_id: usize, tile_info: &mut TileInfo, ent: Entity) {
            //no enemy units - add as hidden
            if tile_info.hidden_units[Entity::get_opposite(ent) as usize].is_empty() {
                tile_info.hidden_units[ent as usize].insert(unit_id);
            } else
            //enemy units detected - make all visible
            {
                for u in tile_info.hidden_units[Entity::get_opposite(ent) as usize].iter() {
                    tile_info.visible_units[Entity::get_opposite(ent) as usize].insert(*u);
                }
                tile_info.hidden_units[Entity::get_opposite(ent) as usize].clear();
                tile_info.visible_units[ent as usize].insert(unit_id);
            }
        }
        pub fn add_hidden_unit(
            self: &mut TerrainGrid,
            unit_id: usize,
            tile: GridTile,
            ent: Entity,
        ) {
            if let Some(end_tile_info) = self.get_title_for_cord(tile) {
                TerrainGrid::add_unit(unit_id, end_tile_info, ent);
            }
        }
        pub fn remove_hidden_unit(unit_id: usize, tile_info: &mut TileInfo, ent: Entity) {
            tile_info.hidden_units[ent as usize].remove(&unit_id);
        }
        pub fn remove_unit(self: &mut TerrainGrid, unit_id: usize, tile: GridTile, ent: Entity) {
            if let Some(tile_info) = self.get_title_for_cord(tile) {
                tile_info.hidden_units[ent as usize].remove(&unit_id);
                tile_info.visible_units[ent as usize].remove(&unit_id);
            }
        }
        pub fn make_visible(self: &mut TerrainGrid, unit_id: usize, tile: GridTile, ent: Entity) {
            if let Some(t) = self.get_title_for_cord(tile) {
                t.hidden_units[ent as usize].remove(&unit_id);
                t.visible_units[ent as usize].insert(unit_id);
            }
        }
        pub fn has_mines(tile: &mut TileInfo) -> bool {
            if let Some(infr) = tile.infrastruct.get_mut(&InfrastructureEnum::Mines) {
                //infr.detected = true;
                return true;
            }
            false
        }
        pub fn move_unit_to_new_tile(
            self: &mut TerrainGrid,
            unit_id: usize,
            move_from_tile: GridTile,
            move_to_tile: GridTile,
            ent: Entity,
        ) -> (bool, bool) {
            let mut res = (false, false);
            if let Some(end_tile_info) = self.get_title_for_cord(move_to_tile) {
                TerrainGrid::add_unit(unit_id, end_tile_info, ent);
                res = (true, TerrainGrid::has_mines(end_tile_info));
            }
            if res.0 {
                if let Some(start_tile_info) = self.get_title_for_cord(move_from_tile) {
                    TerrainGrid::remove_hidden_unit(unit_id, start_tile_info, ent);
                } else {
                    res.0 = false;
                }
            }

            res
        }

        pub fn scan_square(
            self: &mut TerrainGrid,
            center: GridTile,
            detect_possiblity: usize,
            player_type: Entity,
        ) {
            let count: usize = self.get_hidden_units_count(center, player_type);
            let probs = self.probability_generation.ProbabilityRollVect(count);

            let mut detected_units = Vec::<usize>::new();
            if let Some(tile_data) = self.get_title_for_cord(center) {
                //run through hidden units and check which ones were detected
                detected_units = tile_data.hidden_units[player_type as usize]
                    .iter()
                    .zip(probs.iter())
                    .filter(|(_, prob)| **prob < detect_possiblity)
                    .map(|(unit, _)| *unit)
                    .collect::<Vec<usize>>();

                for unit_id in &detected_units {
                    dbg!(&unit_id);
                    tile_data.hidden_units[player_type as usize].remove(unit_id);
                    tile_data.visible_units[player_type as usize].insert(*unit_id);
                }
            }
            for unit_id in detected_units {
                let tile_entry = self
                    .visible_units_per_tile
                    .entry(center)
                    .or_insert(HashSet::<usize>::new());
                tile_entry.insert(unit_id);
            }
        }

        pub fn scan_around(
            self: &mut TerrainGrid,
            center: GridTile,
            distance: u16,
            detect_possiblity: usize,
            player_type: Entity,
        ) {
            let min_y = if distance <= center.1 {
                center.1 - distance
            } else {
                0
            };
            let max_y = if center.1 + distance >= (self.map.len() - 1) as u16 {
                (self.map.len() - 1) as u16
            } else {
                center.1 + distance
            };
            let min_x = if distance <= center.0 {
                center.0 - distance
            } else {
                0
            };
            let max_x = if center.0 + distance > (self.map[center.1 as usize].len() - 1) as u16 {
                (self.map.len() - 1) as u16
            } else {
                center.0 + distance
            };
            let detected_ent = Entity::get_opposite(player_type);

            // scan the square around the center tile and check for hidden units
            for x in min_x..=max_x {
                self.scan_square((x, max_y), detect_possiblity, detected_ent);
            }
            for x in min_x..=max_x {
                self.scan_square((x, min_y), detect_possiblity, detected_ent);
            }
            for y in min_y + 1..max_y {
                self.scan_square((max_x, y), detect_possiblity, detected_ent);
            }
            for y in min_y + 1..max_y {
                self.scan_square((min_x, y), detect_possiblity, detected_ent);
            }
        }
        pub fn unit_detection_chance(
            self: &mut TerrainGrid,
            tile: GridTile,
            radius: usize,
            detect_possiblity: usize,
            ent: Entity,
        ) {
            for r in 1..=radius {
                self.scan_around(tile, r as u16, detect_possiblity / r, ent);
            }
        }
        pub fn add_infr(self: &mut TerrainGrid, new_infr: Arc<Mutex<infstrt::InfrObject>>) {
            let tp = new_infr.lock().unwrap().infr_type;
            let loc = new_infr.lock().unwrap().location;
            
            if let Some(tile_info) = self.get_title_for_cord(loc) {
                tile_info.infrastruct.insert(tp, new_infr);
            }
        }
    }

    pub struct TerrainTiles {
        forrest_txtr: Texture2D,
        ocean_txtr: Texture2D,
        lake_txtr: Texture2D,
        mountain_txtr: Texture2D,
        grass_terrain_txtr: Texture2D,
        urban_txtr: Texture2D,
    }

    impl TerrainTiles {
        pub fn get_tile_texture(self: &TerrainTiles, terrain_type: TerrainTilesEnum) -> &Texture2D {
            match terrain_type {
                TerrainTilesEnum::Forest => &self.forrest_txtr,
                TerrainTilesEnum::Ocean => &self.ocean_txtr,
                TerrainTilesEnum::Lake => &self.lake_txtr,
                TerrainTilesEnum::Mountain => &self.mountain_txtr,
                TerrainTilesEnum::GrassTerrain => &self.grass_terrain_txtr,
                TerrainTilesEnum::Urban => &self.urban_txtr,
                TerrainTilesEnum::End => {
                    unreachable!("Invalid map texture: End")
                }
            }
        }
    }

    pub async fn load_terrain_textures() -> Result<Box<TerrainTiles>, macroquad::Error> {
        Ok(Box::new(TerrainTiles {
            forrest_txtr: load_texture("assets/forest.png").await?,
            ocean_txtr: load_texture("assets/ocean.png").await?,
            lake_txtr: load_texture("assets/lake.png").await?,
            mountain_txtr: load_texture("assets/mountain.png").await?,
            grass_terrain_txtr: load_texture("assets/grass_terrain.png").await?,
            urban_txtr: load_texture("assets/urban.png").await?,
        }))
    }
}
