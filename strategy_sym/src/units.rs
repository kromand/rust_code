pub mod Units {
    use crate::defines::*;
    use macroquad::prelude::*;
    use std::{
        collections::HashMap,
        sync::atomic::{AtomicUsize, Ordering},
    };

    #[derive(Debug)]
    pub enum TextureType {
        Default,
        Moving,
        Damage,
        Destruction,
        End,
    }
    pub fn health_to_texture_type(health_ratio: f32) -> TextureType {
        if health_ratio > 0.5 {
            TextureType::Default
        } else {
            TextureType::Damage
        }
    }
    pub struct AnimateUnit {
        default: UnitTileTextures,
        movement: UnitTileTextures,
        damage: UnitTileTextures,
        destruction: UnitTileTextures,
    }
    impl AnimateUnit {
        pub async fn load_default_textures(
            frame_count: usize,
            frame_repeat_rate: usize,
        ) -> Result<UnitTileTextures, macroquad::Error> {
            let mut vct = Vec::<Box<dyn Iterator<Item = usize>>>::new();

            for _ in 0..(UnitTilesEnum::End as usize) {
                vct.push(Box::new(UnitTileTextures::get_repeat_seq_it(
                    frame_count,
                    frame_repeat_rate,
                )));
            }

            Ok(UnitTileTextures {
                tank_txtr: vec![load_texture("assets/tank_pix.png").await?],
                rocket_arty_txtr: vec![load_texture("assets/himars.png").await?],
                artillery_txtr: vec![load_texture("assets/ai_arty.png").await?],
                apc_txtr: vec![load_texture("assets/apc_pix.png").await?],
                attack_heli_txtr: vec![load_texture("assets/ai_heli.png").await?],
                transport_heli_txtr: vec![load_texture("assets/transport_heli.png").await?],
                plane_txtr: vec![load_texture("assets/plane.png").await?],
                sam_txtr: vec![load_texture("assets/sam.png").await?],
                infantry_txtr: vec![load_texture("assets/infantry_pix.png").await?],
                scout_txtr: vec![load_texture("assets/scouts.png").await?],
                frame_itr: vct,
            })
        }
        pub async fn load_damage_textures(
            frame_repeat_rate: usize,
        ) -> Result<UnitTileTextures, macroquad::Error> {
            // Load textures first
            let tank_txtr = vec![load_texture("assets/tank_pix.png").await?];
            let rocket_arty_txtr = vec![load_texture("assets/himars.png").await?];
            let artillery_txtr = vec![load_texture("assets/ai_arty.png").await?];
            let apc_txtr = vec![load_texture("assets/apc_dmg_1.png").await?,
                load_texture("assets/apc_dmg_2.png").await?,
                load_texture("assets/apc_dmg_3.png").await?,
                load_texture("assets/apc_dmg_4.png").await?,];

            let attack_heli_txtr = vec![load_texture("assets/ai_heli.png").await?];
            let transport_heli_txtr = vec![load_texture("assets/transport_heli.png").await?];
            let plane_txtr = vec![load_texture("assets/plane.png").await?];
            let sam_txtr = vec![load_texture("assets/sam.png").await?];
            let infantry_txtr = vec![load_texture("assets/infantry_pix.png").await?];
            let scout_txtr = vec![load_texture("assets/scouts.png").await?];

            // Initialize frame_itr with vector sizes
            let mut vct = Vec::<Box<dyn Iterator<Item = usize>>>::new();
            vct.push(Box::new(UnitTileTextures::get_repeat_seq_it(tank_txtr.len(), frame_repeat_rate)));
            vct.push(Box::new(UnitTileTextures::get_repeat_seq_it(rocket_arty_txtr.len(), frame_repeat_rate)));
            vct.push(Box::new(UnitTileTextures::get_repeat_seq_it(artillery_txtr.len(), frame_repeat_rate)));
            vct.push(Box::new(UnitTileTextures::get_repeat_seq_it(artillery_txtr.len(), frame_repeat_rate)));
            vct.push(Box::new(UnitTileTextures::get_repeat_seq_it(apc_txtr.len(), frame_repeat_rate)));
            vct.push(Box::new(UnitTileTextures::get_repeat_seq_it(attack_heli_txtr.len(), frame_repeat_rate)));
            vct.push(Box::new(UnitTileTextures::get_repeat_seq_it(transport_heli_txtr.len(), frame_repeat_rate)));
            vct.push(Box::new(UnitTileTextures::get_repeat_seq_it(plane_txtr.len(), frame_repeat_rate)));
            vct.push(Box::new(UnitTileTextures::get_repeat_seq_it(sam_txtr.len(), frame_repeat_rate)));
            vct.push(Box::new(UnitTileTextures::get_repeat_seq_it(infantry_txtr.len(), frame_repeat_rate)));
            vct.push(Box::new(UnitTileTextures::get_repeat_seq_it(scout_txtr.len(), frame_repeat_rate)));

            Ok(UnitTileTextures {
                tank_txtr,
                rocket_arty_txtr,
                artillery_txtr,
                apc_txtr,
                attack_heli_txtr,
                transport_heli_txtr,
                plane_txtr,
                sam_txtr,
                infantry_txtr,
                scout_txtr,
                frame_itr: vct,
            })
        }
        pub async fn load_movement_textures(
            frame_count: usize,
            frame_repeat_rate: usize,
        ) -> Result<UnitTileTextures, macroquad::Error> {
            let mut vct = Vec::<Box<dyn Iterator<Item = usize>>>::new();

            vct.push(Box::new(UnitTileTextures::get_repeat_seq_it(
                    frame_count,
                    frame_repeat_rate,
                )));

            Ok(UnitTileTextures {
                tank_txtr: vec![load_texture("assets/tank_pix.png").await?],
                rocket_arty_txtr: vec![load_texture("assets/himars.png").await?],
                artillery_txtr: vec![load_texture("assets/ai_arty.png").await?],
                apc_txtr: vec![load_texture("assets/apc_pix.png").await?],
                attack_heli_txtr: vec![load_texture("assets/attack_heli.png").await?],
                transport_heli_txtr: vec![load_texture("assets/transport_heli.png").await?],
                plane_txtr: vec![load_texture("assets/plane.png").await?],
                sam_txtr: vec![load_texture("assets/sam.png").await?],
                infantry_txtr: vec![load_texture("assets/infantry_pix.png").await?],
                scout_txtr: vec![load_texture("assets/scouts.png").await?],
                frame_itr: vct,
            })
        }
        pub async fn load_destruction_textures(
            frame_count: usize,
            frame_repeat_rate: usize,
        ) -> Result<UnitTileTextures, macroquad::Error> {
            let mut vct = Vec::<Box<dyn Iterator<Item = usize>>>::new();

            for _ in 0..(UnitTilesEnum::End as usize) {
                vct.push(Box::new(UnitTileTextures::get_repeat_seq_it(
                    frame_count,
                    frame_repeat_rate,
                )));
            }

            Ok(UnitTileTextures {
                tank_txtr: vec![load_texture("assets/tank_pix.png").await?],
                rocket_arty_txtr: vec![load_texture("assets/himars.png").await?],
                artillery_txtr: vec![load_texture("assets/ai_arty.png").await?],
                apc_txtr: vec![load_texture("assets/apc_pix.png").await?],
                attack_heli_txtr: vec![load_texture("assets/attack_heli.png").await?],
                transport_heli_txtr: vec![load_texture("assets/transport_heli.png").await?],
                plane_txtr: vec![load_texture("assets/plane.png").await?],
                sam_txtr: vec![load_texture("assets/sam.png").await?],
                infantry_txtr: vec![load_texture("assets/infantry_pix.png").await?],
                scout_txtr: vec![load_texture("assets/scouts.png").await?],
                frame_itr: vct,
            })
        }
        pub async fn new() -> Result<Box<AnimateUnit>, macroquad::Error> {
            Ok(Box::new(AnimateUnit {
                default: AnimateUnit::load_default_textures(1, 1).await?,
                movement: AnimateUnit::load_movement_textures(1, 1).await?,
                damage: AnimateUnit::load_damage_textures( 20).await?,
                destruction: AnimateUnit::load_destruction_textures(1, 1).await?,
            }))
        }
        pub fn get_texture(
            self: &mut AnimateUnit,
            unit_type: UnitTilesEnum,
            texture_type: TextureType,
        ) -> &Texture2D {
            match texture_type {
                TextureType::Default => self.default.get_unit_texture(unit_type),
                TextureType::Moving => &self.movement.get_unit_texture(unit_type),
                TextureType::Damage => &self.damage.get_unit_texture(unit_type),
                _ => {
                    dbg!(texture_type);
                    unreachable!()
                }
            }
        }
    }

    pub struct UnitTileTextures {
        tank_txtr: Vec<Texture2D>,
        rocket_arty_txtr: Vec<Texture2D>,
        artillery_txtr: Vec<Texture2D>,
        apc_txtr: Vec<Texture2D>,
        attack_heli_txtr: Vec<Texture2D>,
        transport_heli_txtr: Vec<Texture2D>,
        plane_txtr: Vec<Texture2D>,
        sam_txtr: Vec<Texture2D>,
        infantry_txtr: Vec<Texture2D>,
        scout_txtr: Vec<Texture2D>,
        frame_itr: Vec<Box<dyn Iterator<Item = usize>>>,
    }
    impl UnitTileTextures {
        pub fn get_repeat_seq_it(len: usize, repeat: usize) -> impl Iterator<Item = usize> {
            //to collect into a vector:
            // (0..len).flat_map(|n| std::iter::repeat(n).take(repeat)).collect::<Vec<usize>>()
            (0..len.to_owned())
                .flat_map(move |n| std::iter::repeat(n).take(repeat))
                .cycle()
        }

        pub fn get_unit_texture(
            self: &mut UnitTileTextures,
            unit_type: UnitTilesEnum,
        ) -> &Texture2D {
            //calling unwrap here since any frame sequence should be cyclical, infinte
            //TODO: since death sequence will non-cyclical, need to implement that as well
            //TODO: the iterator might need to move to unit info since each animation will be separate per unit
            let frame_index = self.frame_itr[unit_type as usize].next().unwrap();

            match unit_type {
                UnitTilesEnum::Tank => &self.tank_txtr[frame_index],
                UnitTilesEnum::Infantry => &self.infantry_txtr[frame_index],
                UnitTilesEnum::Scout => &self.scout_txtr[frame_index],
                UnitTilesEnum::RocketArty => &self.rocket_arty_txtr[frame_index],
                UnitTilesEnum::Artillery => &self.artillery_txtr[frame_index],
                UnitTilesEnum::APC => &self.apc_txtr[frame_index],
                UnitTilesEnum::TransportHeli => &self.transport_heli_txtr[frame_index],
                UnitTilesEnum::Plane => &self.plane_txtr[frame_index],
                UnitTilesEnum::SAM => &self.sam_txtr[frame_index],
                UnitTilesEnum::AttackHeli => &self.attack_heli_txtr[frame_index],
                _ => {
                    dbg!(unit_type);
                    unreachable!()
                }
            }
        }
    }

    pub struct UnitId {
        next_id: AtomicUsize,
        unit_type_counts: HashMap<UnitTilesEnum, usize>,
    }
    impl UnitId {
        pub fn new() -> UnitId {
            UnitId {
                next_id: AtomicUsize::new(0),
                unit_type_counts: HashMap::new(),
            }
        }
        pub fn get_new(self: &mut UnitId) -> usize {
            self.next_id.fetch_add(1, Ordering::Relaxed)
        }
        pub fn get_next_name(&mut self, tp: UnitTilesEnum) -> String {
            let count = self.unit_type_counts.entry(tp).or_insert(0);
            *count += 1;
            let ordinal = match *count % 10 {
                1 if *count % 100 != 11 => "st",
                2 if *count % 100 != 12 => "nd",
                3 if *count % 100 != 13 => "rd",
                _ => "th",
            };
            format!("{}{}_{}", count, ordinal, tp)
        }
    }
    pub struct UnitInfo {
        pub unit_id: usize,
        pub player_id: Entity,
        pub unit_name: String,
        pub unit_type: UnitTilesEnum,
        pub max_health: f32,
        pub health: f32,
        movement_rate: f32,
        pub location: GridTile,
        pub allowed_terrains: [bool; TerrainTilesEnum::End as usize],
        pub visibility_range: usize,
        pub prob_to_detect_units: usize,
    }
    impl Default for UnitInfo {
        fn default() -> Self {
            UnitInfo {
                unit_id: 1,
                player_id: Entity::AI,
                unit_name: "martian riders".to_owned(),
                unit_type: UnitTilesEnum::End,
                max_health: 100.0,
                health: 100.0,
                movement_rate: 2.0,
                location: (0, 0),
                allowed_terrains: [false; TerrainTilesEnum::End as usize],
                visibility_range: 1,
                prob_to_detect_units: 50,
            }
        }
    }
    impl UnitInfo {
        pub fn new(
            tp: UnitTilesEnum,
            id_gen: &mut UnitId,
            p_id: Entity,
            loc: GridTile,
        ) -> UnitInfo {
            match tp {
                UnitTilesEnum::Tank => UnitInfo {
                    unit_id: id_gen.get_new(),
                    player_id: p_id,
                    unit_name: id_gen.get_next_name(tp),
                    unit_type: tp,
                    max_health: 200.0,
                    health: 200.0,
                    movement_rate: 2.0,
                    location: loc,
                    allowed_terrains: [true, false, false, false, true, true],
                    visibility_range: 1,
                    prob_to_detect_units: 50,
                },
                UnitTilesEnum::APC => UnitInfo {
                    unit_id: id_gen.get_new(),
                    player_id: p_id,
                    unit_name: id_gen.get_next_name(tp),
                    unit_type: tp,
                    max_health: 200.0,
                    health: 200.0,
                    movement_rate: 2.0,
                    location: loc,
                    allowed_terrains: [true, false, true, false, true, true],
                    visibility_range: 1,
                    prob_to_detect_units: 50,
                },
                UnitTilesEnum::Artillery => UnitInfo {
                    unit_id: id_gen.get_new(),
                    player_id: p_id,
                    unit_name: id_gen.get_next_name(tp),
                    unit_type: tp,
                    max_health: 160.0,
                    health: 160.0,
                    movement_rate: 2.0,
                    location: loc,
                    allowed_terrains: [true, false, true, false, true, true],
                    visibility_range: 2,
                    prob_to_detect_units: 40,
                },
                UnitTilesEnum::RocketArty => UnitInfo {
                    unit_id: id_gen.get_new(),
                    player_id: p_id,
                    unit_name: id_gen.get_next_name(tp),
                    unit_type: tp,
                    max_health: 180.0,
                    health: 180.0,
                    movement_rate: 2.0,
                    location: loc,
                    allowed_terrains: [true, false, true, false, true, true],
                    visibility_range: 3,
                    prob_to_detect_units: 45,
                },
                UnitTilesEnum::Engineers => UnitInfo {
                    unit_id: id_gen.get_new(),
                    player_id: p_id,
                    unit_name: id_gen.get_next_name(tp),
                    unit_type: tp,
                    max_health: 80.0,
                    health: 80.0,
                    movement_rate: 1.0,
                    location: loc,
                    allowed_terrains: [true, false, false, true, true, true],
                    visibility_range: 1,
                    prob_to_detect_units: 60,
                },
                UnitTilesEnum::AttackHeli => UnitInfo {
                    unit_id: id_gen.get_new(),
                    player_id: p_id,
                    unit_name: id_gen.get_next_name(tp),
                    unit_type: tp,
                    max_health: 200.0,
                    health: 200.0,
                    movement_rate: 4.0,
                    location: loc,
                    allowed_terrains: [true, true, true, true, true, true],
                    visibility_range: 3,
                    prob_to_detect_units: 90,
                },
                UnitTilesEnum::TransportHeli => UnitInfo {
                    unit_id: id_gen.get_new(),
                    player_id: p_id,
                    unit_name: id_gen.get_next_name(tp),
                    unit_type: tp,
                    max_health: 180.0,
                    health: 180.0,
                    movement_rate: 4.0,
                    location: loc,
                    allowed_terrains: [true, true, true, true, true, true],
                    visibility_range: 2,
                    prob_to_detect_units: 80,
                },
                UnitTilesEnum::Plane => UnitInfo {
                    unit_id: id_gen.get_new(),
                    player_id: p_id,
                    unit_name: id_gen.get_next_name(tp),
                    unit_type: tp,
                    max_health: 220.0,
                    health: 220.0,
                    movement_rate: 5.0,
                    location: loc,
                    allowed_terrains: [true, true, true, true, true, true],
                    visibility_range: 4,
                    prob_to_detect_units: 85,
                },
                UnitTilesEnum::SAM => UnitInfo {
                    unit_id: id_gen.get_new(),
                    player_id: p_id,
                    unit_name: id_gen.get_next_name(tp),
                    unit_type: tp,
                    max_health: 200.0,
                    health: 200.0,
                    movement_rate: 4.0,
                    location: loc,
                    allowed_terrains: [true, false, false, false, true, true],
                    visibility_range: 0,
                    prob_to_detect_units: 5,
                },
                UnitTilesEnum::Infantry => UnitInfo {
                    unit_id: id_gen.get_new(),
                    player_id: p_id,
                    unit_name: id_gen.get_next_name(tp),
                    unit_type: tp,
                    max_health: 100.0,
                    health: 100.0,
                    movement_rate: 1.0,
                    location: loc,
                    allowed_terrains: [true, false, false, true, true, true],
                    visibility_range: 1,
                    prob_to_detect_units: 70,
                },
                UnitTilesEnum::Scout => UnitInfo {
                    unit_id: id_gen.get_new(),
                    player_id: p_id,
                    unit_name: id_gen.get_next_name(tp),
                    unit_type: tp,
                    max_health: 50.0,
                    health: 50.0,
                    movement_rate: 1.0,
                    location: loc,
                    allowed_terrains: [true, false, true, true, true, true],
                    visibility_range: 2,
                    prob_to_detect_units: 70,
                },
                _ => UnitInfo::default(),
            }
        }
        pub fn allowed_move(self: &UnitInfo, new_title: TerrainTilesEnum) -> bool {
            self.allowed_terrains[new_title as usize]
        }
        pub fn takes_damage(self: &mut UnitInfo, dmg: f32) -> bool {
            if dmg < self.health {
                self.health -= dmg;
                false
            } else {
                self.health = 0.0;
                true
            }
        }
        pub fn get_health_bar(self: &UnitInfo) -> f32 {
            self.health / self.max_health
        }
        pub fn assess_damage(self: &mut UnitInfo, prob: usize) -> bool {
            if prob <= 85 {
                self.takes_damage(0.3 * self.max_health);
            }
            self.health <= 0.0
        }
    }

    pub struct AI_units {
        pub units: HashMap<usize, UnitInfo>,
    }
    impl AI_units {
        pub fn new() -> AI_units {
            AI_units {
                units: HashMap::<usize, UnitInfo>::new(),
            }
        }
        pub fn add_test_units(self: &mut AI_units, id_gen: &mut UnitId) {
            //add sample AI infantry
            let mut new_unit = UnitInfo::new(UnitTilesEnum::Scout, id_gen, Entity::AI, (15, 7));
            self.units.insert(new_unit.unit_id, new_unit);

            // add AI tank
            new_unit = UnitInfo::new(UnitTilesEnum::Tank, id_gen, Entity::AI, (16, 6));
            self.units.insert(new_unit.unit_id, new_unit);

            // add AI SAM
            new_unit = UnitInfo::new(UnitTilesEnum::SAM, id_gen, Entity::AI, (17, 5));
            self.units.insert(new_unit.unit_id, new_unit);
        }
    }

    #[derive(Default)]
    pub struct UnitStack {
        pub units: HashMap<usize, UnitInfo>,
        pub top: Option<usize>,
    }
    impl UnitStack {
        pub fn new() -> UnitStack {
            UnitStack {
                units: HashMap::<usize, UnitInfo>::new(),
                top: None,
            }
        }
    }
    pub struct PlayerUnits {
        pub units_by_tile: HashMap<GridTile, UnitStack>,
    }

    impl PlayerUnits {
        pub fn new() -> PlayerUnits {
            PlayerUnits {
                units_by_tile: HashMap::<GridTile, UnitStack>::new(),
            }
        }

        pub fn add_unit(&mut self, unit: UnitInfo) {
            let mut unit_stack = self.units_by_tile.entry(unit.location).or_default();
            if unit_stack.units.is_empty() {
                unit_stack.top = Some(unit.unit_id);
            }
            unit_stack.units.insert(unit.unit_id, unit);
        }

        pub fn add_unit_at(
            &mut self,
            unit_type: UnitTilesEnum,
            id_gen: &mut UnitId,
            player_id: Entity,
            location: GridTile,
        ) -> usize {
            let unit = UnitInfo::new(unit_type, id_gen, player_id, location);
            let id = unit.unit_id;
            self.add_unit(unit);
            id
        }

        pub fn move_unit(
            &mut self,
            start_tile: GridTile,
            unit_id: usize,
            new_tile: GridTile,
        ) -> bool {
            if let Some(units_at_tile) = self.units_by_tile.get_mut(&start_tile) {
                if let Some(mut unit) = units_at_tile.units.remove(&unit_id) {
                    unit.location = new_tile;
                    self.units_by_tile
                        .entry(new_tile)
                        .or_default()
                        .units
                        .insert(unit_id, unit);

                    return true;
                }
            }

            false
        }

        pub fn get_units_at(&self, tile: GridTile) -> Option<&HashMap<usize, UnitInfo>> {
            self.units_by_tile.get(&tile).map(|stack| &stack.units)
        }

        pub fn remove_unit(&mut self, tile: GridTile, unit_id: usize) -> bool {
            if let Some(units_at_tile) = self.units_by_tile.get_mut(&tile) {
                if units_at_tile.units.remove(&unit_id).is_some() {
                    return true;
                }
            }
            false
        }
    }

    pub struct DamageAssessment {
        damage_matrix: Vec<Vec<f32>>,
    }

    impl DamageAssessment {
        pub fn new() -> DamageAssessment {
            let mut dmg_vec = Vec::<Vec<f32>>::new();
            dmg_vec.reserve(UnitTilesEnum::End as usize);

            dmg_vec[UnitTilesEnum::Tank as usize] =
                vec![1.0, 2.0, 4.0, 2.0, 1.5, 100.0, 100.0, 0.05, 0.2, 100.0];
            dmg_vec[UnitTilesEnum::Infantry as usize] =
                vec![0.3, 2.0, 4.0, 2.0, 1.5, 100.0, 100.0, 0.05, 0.2, 100.0];
            dmg_vec[UnitTilesEnum::Scout as usize] =
                vec![1.0, 2.0, 4.0, 2.0, 1.5, 100.0, 100.0, 0.05, 0.2, 100.0];
            dmg_vec[UnitTilesEnum::Engineers as usize] =
                vec![1.0, 2.0, 4.0, 2.0, 1.5, 100.0, 100.0, 0.05, 0.2, 100.0];

            dmg_vec[UnitTilesEnum::APC as usize] =
                vec![1.0, 2.0, 4.0, 2.0, 1.5, 100.0, 100.0, 0.05, 0.2, 100.0];
            dmg_vec[UnitTilesEnum::RocketArty as usize] =
                vec![1.0, 2.0, 4.0, 2.0, 1.5, 100.0, 100.0, 0.05, 0.2, 100.0];
            dmg_vec[UnitTilesEnum::Artillery as usize] =
                vec![1.0, 2.0, 4.0, 2.0, 1.5, 100.0, 100.0, 0.05, 0.2, 100.0];
            dmg_vec[UnitTilesEnum::AttackHeli as usize] =
                vec![1.0, 2.0, 4.0, 2.0, 1.5, 100.0, 100.0, 0.05, 0.2, 100.0];

            dmg_vec[UnitTilesEnum::TransportHeli as usize] =
                vec![1.0, 2.0, 4.0, 2.0, 1.5, 100.0, 100.0, 0.05, 0.2, 100.0];
            dmg_vec[UnitTilesEnum::SAM as usize] =
                vec![1.0, 2.0, 4.0, 2.0, 1.5, 100.0, 100.0, 0.05, 0.2, 100.0];

            DamageAssessment {
                damage_matrix: dmg_vec,
            }
        }

        pub fn damage_multiplier(&self, source: UnitTilesEnum, target: UnitTilesEnum) -> f32 {
            // Return the multiplier from the damage matrix for given source and target unit types.
            // Assumes `damage_matrix` has been initialized with length >= UnitTilesEnum::End.
            self.damage_matrix[source as usize][target as usize]
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn add_unit_at_inserts_unit_into_tile_map() {
            let mut id_gen = UnitId::new();
            let mut player_units = PlayerUnits::new();

            let unit_id =
                player_units.add_unit_at(UnitTilesEnum::Tank, &mut id_gen, Entity::Player, (2, 3));

            let tile_units = player_units
                .get_units_at((2, 3))
                .expect("Tile should exist");
            assert_eq!(tile_units.len(), 1);
            assert!(tile_units.contains_key(&unit_id));
        }

        #[test]
        fn move_unit_moves_unit_between_tiles() {
            let mut id_gen = UnitId::new();
            let mut player_units = PlayerUnits::new();

            let unit_id =
                player_units.add_unit_at(UnitTilesEnum::Tank, &mut id_gen, Entity::Player, (2, 3));
            let moved = player_units.move_unit((2, 3), unit_id, (3, 4));

            assert!(
                moved,
                "move_unit should succeed when start_tile contains the unit"
            );
            let source_units = player_units
                .get_units_at((2, 3))
                .expect("source tile should exist");
            assert!(
                source_units.is_empty(),
                "old tile should be empty after move"
            );

            let target_units = player_units
                .get_units_at((3, 4))
                .expect("Target tile should exist");
            assert_eq!(target_units.len(), 1);
            let moved_unit = target_units
                .get(&unit_id)
                .expect("Moved unit should be present");
            assert_eq!(moved_unit.location, (3, 4));
        }

        #[test]
        fn move_unit_fails_if_start_tile_is_wrong() {
            let mut id_gen = UnitId::new();
            let mut player_units = PlayerUnits::new();

            let unit_id =
                player_units.add_unit_at(UnitTilesEnum::Tank, &mut id_gen, Entity::Player, (2, 3));
            let moved = player_units.move_unit((1, 1), unit_id, (3, 4));

            assert!(
                !moved,
                "move_unit should fail when the unit is not present at start_tile"
            );
            let original_units = player_units
                .get_units_at((2, 3))
                .expect("Original tile should still exist");
            assert!(original_units.contains_key(&unit_id));
        }
    }
}
