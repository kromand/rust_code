pub mod unit {
    use crate::defines::*;
    use macroquad::prelude::*;
    use std::{
        collections::HashMap,
        sync::atomic::{AtomicUsize, Ordering},
    };

    const DEFAULT_TANK_TEXTURE_FILE: &str = "assets/tank_pix.png";
    const DEFAULT_TANK_DMG_1_TEXTURE_FILE: &str = "assets/tank_dmg_1.png";
    const DEFAULT_TANK_DMG_2_TEXTURE_FILE: &str = "assets/tank_dmg_2.png";
    const DEFAULT_TANK_DMG_3_TEXTURE_FILE: &str = "assets/tank_dmg_3.png";
    const DEFAULT_TANK_DMG_4_TEXTURE_FILE: &str = "assets/tank_dmg_4.png";
    const DEFAULT_TANK_DEST_1_TEXTURE_FILE: &str = "assets/tank_dest_1.png";
    const DEFAULT_TANK_DEST_2_TEXTURE_FILE: &str = "assets/tank_dest_2.png";
    const DEFAULT_TANK_DEST_3_TEXTURE_FILE: &str = "assets/tank_dest_3.png";
    const DEFAULT_TANK_DEST_4_TEXTURE_FILE: &str = "assets/tank_dest_4.png";
    const DEFAULT_ROCKETARTY_TEXTURE_FILE: &str = "assets/rocket_arty.png";
    const DEFAULT_ARTILLERY_TEXTURE_FILE: &str = "assets/ai_arty.png";
    const DEFAULT_APC_TEXTURE_FILE: &str = "assets/apc_pix.png";
    const DEFAULT_APC_DMG_1_TEXTURE_FILE: &str = "assets/apc_dmg_1.png";
    const DEFAULT_APC_DMG_2_TEXTURE_FILE: &str = "assets/apc_dmg_2.png";
    const DEFAULT_APC_DMG_3_TEXTURE_FILE: &str = "assets/apc_dmg_3.png";
    const DEFAULT_APC_DMG_4_TEXTURE_FILE: &str = "assets/apc_dmg_4.png";
    const DEFAULT_ATTACK_HELI_TEXTURE_FILE: &str = "assets/ai_heli.png";
    const DEFAULT_TRANSPORT_HELI_TEXTURE_FILE: &str = "assets/transport_heli.png";
    const DEFAULT_PLANE_TEXTURE_FILE: &str = "assets/plane.png";
    const DEFAULT_SAM_TEXTURE_FILE: &str = "assets/sam.png";
    const DEFAULT_INFANTRY_TEXTURE_FILE: &str = "assets/infantry_pix.png";
    const DEFAULT_SCOUT_TEXTURE_FILE: &str = "assets/scouts.png";

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
        pub async fn load_default_textures() -> Result<UnitTileTextures, macroquad::Error> {
            Ok(UnitTileTextures {
                tank_txtr: vec![load_texture(DEFAULT_TANK_TEXTURE_FILE).await?],
                rocket_arty_txtr: vec![load_texture(DEFAULT_ROCKETARTY_TEXTURE_FILE).await?],
                artillery_txtr: vec![load_texture(DEFAULT_ARTILLERY_TEXTURE_FILE).await?],
                apc_txtr: vec![load_texture(DEFAULT_APC_TEXTURE_FILE).await?],
                attack_heli_txtr: vec![load_texture(DEFAULT_ATTACK_HELI_TEXTURE_FILE).await?],
                transport_heli_txtr: vec![load_texture(DEFAULT_TRANSPORT_HELI_TEXTURE_FILE).await?],
                plane_txtr: vec![load_texture(DEFAULT_PLANE_TEXTURE_FILE).await?],
                sam_txtr: vec![load_texture(DEFAULT_SAM_TEXTURE_FILE).await?],
                infantry_txtr: vec![load_texture(DEFAULT_INFANTRY_TEXTURE_FILE).await?],
                scout_txtr: vec![load_texture(DEFAULT_SCOUT_TEXTURE_FILE).await?],
            })
        }
        pub async fn load_damage_textures() -> Result<UnitTileTextures, macroquad::Error> {
            Ok(UnitTileTextures {
                tank_txtr: vec![
                    load_texture(DEFAULT_TANK_DMG_1_TEXTURE_FILE).await?,
                    load_texture(DEFAULT_TANK_DMG_2_TEXTURE_FILE).await?,
                    load_texture(DEFAULT_TANK_DMG_3_TEXTURE_FILE).await?,
                    load_texture(DEFAULT_TANK_DMG_4_TEXTURE_FILE).await?,
                ],
                rocket_arty_txtr: vec![load_texture(DEFAULT_ROCKETARTY_TEXTURE_FILE).await?],
                artillery_txtr: vec![load_texture(DEFAULT_ARTILLERY_TEXTURE_FILE).await?],
                apc_txtr: vec![
                    load_texture(DEFAULT_APC_DMG_1_TEXTURE_FILE).await?,
                    load_texture(DEFAULT_APC_DMG_2_TEXTURE_FILE).await?,
                    load_texture(DEFAULT_APC_DMG_3_TEXTURE_FILE).await?,
                    load_texture(DEFAULT_APC_DMG_4_TEXTURE_FILE).await?,
                ],
                attack_heli_txtr: vec![load_texture(DEFAULT_ATTACK_HELI_TEXTURE_FILE).await?],
                transport_heli_txtr: vec![load_texture(DEFAULT_TRANSPORT_HELI_TEXTURE_FILE).await?],
                plane_txtr: vec![load_texture(DEFAULT_PLANE_TEXTURE_FILE).await?],
                sam_txtr: vec![load_texture(DEFAULT_SAM_TEXTURE_FILE).await?],
                infantry_txtr: vec![load_texture(DEFAULT_INFANTRY_TEXTURE_FILE).await?],
                scout_txtr: vec![load_texture(DEFAULT_SCOUT_TEXTURE_FILE).await?],
            })
        }
        pub async fn load_movement_textures() -> Result<UnitTileTextures, macroquad::Error> {
            Ok(UnitTileTextures {
                tank_txtr: vec![load_texture(DEFAULT_TANK_TEXTURE_FILE).await?],
                rocket_arty_txtr: vec![load_texture(DEFAULT_ROCKETARTY_TEXTURE_FILE).await?],
                artillery_txtr: vec![load_texture(DEFAULT_ARTILLERY_TEXTURE_FILE).await?],
                apc_txtr: vec![load_texture(DEFAULT_APC_TEXTURE_FILE).await?],
                attack_heli_txtr: vec![load_texture(DEFAULT_ATTACK_HELI_TEXTURE_FILE).await?],
                transport_heli_txtr: vec![load_texture(DEFAULT_TRANSPORT_HELI_TEXTURE_FILE).await?],
                plane_txtr: vec![load_texture(DEFAULT_PLANE_TEXTURE_FILE).await?],
                sam_txtr: vec![load_texture(DEFAULT_SAM_TEXTURE_FILE).await?],
                infantry_txtr: vec![load_texture(DEFAULT_INFANTRY_TEXTURE_FILE).await?],
                scout_txtr: vec![load_texture(DEFAULT_SCOUT_TEXTURE_FILE).await?],
            })
        }
        pub async fn load_destruction_textures() -> Result<UnitTileTextures, macroquad::Error> {
            Ok(UnitTileTextures {
                tank_txtr: vec![
                    load_texture(DEFAULT_TANK_DEST_1_TEXTURE_FILE).await?,
                    load_texture(DEFAULT_TANK_DEST_2_TEXTURE_FILE).await?,
                    load_texture(DEFAULT_TANK_DEST_3_TEXTURE_FILE).await?,
                    load_texture(DEFAULT_TANK_DEST_4_TEXTURE_FILE).await?,
                ],
                rocket_arty_txtr: vec![load_texture(DEFAULT_ROCKETARTY_TEXTURE_FILE).await?],
                artillery_txtr: vec![load_texture(DEFAULT_ARTILLERY_TEXTURE_FILE).await?],
                apc_txtr: vec![load_texture(DEFAULT_APC_TEXTURE_FILE).await?],
                attack_heli_txtr: vec![load_texture(DEFAULT_ATTACK_HELI_TEXTURE_FILE).await?],
                transport_heli_txtr: vec![load_texture(DEFAULT_TRANSPORT_HELI_TEXTURE_FILE).await?],
                plane_txtr: vec![load_texture(DEFAULT_PLANE_TEXTURE_FILE).await?],
                sam_txtr: vec![load_texture(DEFAULT_SAM_TEXTURE_FILE).await?],
                infantry_txtr: vec![load_texture(DEFAULT_INFANTRY_TEXTURE_FILE).await?],
                scout_txtr: vec![load_texture(DEFAULT_SCOUT_TEXTURE_FILE).await?],
            })
        }
        pub async fn new() -> Result<Box<AnimateUnit>, macroquad::Error> {
            Ok(Box::new(AnimateUnit {
                default: AnimateUnit::load_default_textures().await?,
                movement: AnimateUnit::load_movement_textures().await?,
                damage: AnimateUnit::load_damage_textures().await?,
                destruction: AnimateUnit::load_destruction_textures().await?,
            }))
        }
        pub fn get_destruction_texture(&self, unit_type: UnitTilesEnum, frame: usize) -> &Texture2D {
            self.destruction.get_unit_texture(unit_type, frame)
        }

        pub fn get_texture(
            &mut self,
            unit_type: UnitTilesEnum,
            texture_type: TextureType,
            frame_itr: &mut Box<dyn Iterator<Item = usize>>,
        ) -> &Texture2D {
            let frame = frame_itr.next().unwrap();
            match texture_type {
                TextureType::Default => self.default.get_unit_texture(unit_type, frame),
                TextureType::Moving => self.movement.get_unit_texture(unit_type, frame),
                TextureType::Damage => self.damage.get_unit_texture(unit_type, frame),
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
    }
    impl UnitTileTextures {
        pub fn get_repeat_seq_it(len: usize, repeat: usize) -> impl Iterator<Item = usize> {
            (0..len.to_owned())
                .flat_map(move |n| std::iter::repeat(n).take(repeat))
                .cycle()
        }

        pub fn get_oneshot_seq_it(len: usize, repeat: usize) -> impl Iterator<Item = usize> {
            (0..len).flat_map(move |n| std::iter::repeat(n).take(repeat))
        }

        pub fn get_unit_texture(&self, unit_type: UnitTilesEnum, frame: usize) -> &Texture2D {
            match unit_type {
                UnitTilesEnum::Tank => &self.tank_txtr[frame % self.tank_txtr.len()],
                UnitTilesEnum::Infantry => &self.infantry_txtr[frame % self.infantry_txtr.len()],
                UnitTilesEnum::Scout => &self.scout_txtr[frame % self.scout_txtr.len()],
                UnitTilesEnum::RocketArty => &self.rocket_arty_txtr[frame % self.rocket_arty_txtr.len()],
                UnitTilesEnum::Artillery => &self.artillery_txtr[frame % self.artillery_txtr.len()],
                UnitTilesEnum::APC => &self.apc_txtr[frame % self.apc_txtr.len()],
                UnitTilesEnum::TransportHeli => &self.transport_heli_txtr[frame % self.transport_heli_txtr.len()],
                UnitTilesEnum::Plane => &self.plane_txtr[frame % self.plane_txtr.len()],
                UnitTilesEnum::SAM => &self.sam_txtr[frame % self.sam_txtr.len()],
                UnitTilesEnum::AttackHeli => &self.attack_heli_txtr[frame % self.attack_heli_txtr.len()],
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
        pub movement_rate: f32,
        pub location: GridTile,
        pub allowed_terrains: [bool; TerrainTilesEnum::End as usize],
        pub visibility_range: usize,
        pub prob_to_detect_units: usize,
        pub frame_itr: Box<dyn Iterator<Item = usize>>,
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
                location: GridTile::new(0, 0),
                allowed_terrains: [false; TerrainTilesEnum::End as usize],
                visibility_range: 1,
                prob_to_detect_units: 50,
                frame_itr: Box::new(std::iter::repeat(0usize)),
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
            let frame_itr_1 = || -> Box<dyn Iterator<Item = usize>> {
                Box::new(UnitTileTextures::get_repeat_seq_it(1, 20))
            };
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
                    frame_itr: Box::new(UnitTileTextures::get_repeat_seq_it(4, 20)),
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
                    frame_itr: Box::new(UnitTileTextures::get_repeat_seq_it(4, 20)),
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
                    frame_itr: frame_itr_1(),
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
                    frame_itr: frame_itr_1(),
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
                    frame_itr: frame_itr_1(),
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
                    frame_itr: frame_itr_1(),
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
                    frame_itr: frame_itr_1(),
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
                    frame_itr: frame_itr_1(),
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
                    frame_itr: frame_itr_1(),
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
                    frame_itr: frame_itr_1(),
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
                    frame_itr: frame_itr_1(),
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

    pub struct AiUnits {
        pub units: HashMap<usize, UnitInfo>,
    }
    impl AiUnits {
        pub fn new() -> AiUnits {
            AiUnits {
                units: HashMap::<usize, UnitInfo>::new(),
            }
        }
        pub fn add_test_units(self: &mut AiUnits, id_gen: &mut UnitId) {
            //add sample AI infantry
            let mut new_unit = UnitInfo::new(
                UnitTilesEnum::Scout,
                id_gen,
                Entity::AI,
                GridTile::new(7, 15),
            );
            self.units.insert(new_unit.unit_id, new_unit);

            // add AI tank
            new_unit = UnitInfo::new(
                UnitTilesEnum::Tank,
                id_gen,
                Entity::AI,
                GridTile::new(6, 16),
            );
            self.units.insert(new_unit.unit_id, new_unit);

            // add AI SAM
            new_unit = UnitInfo::new(UnitTilesEnum::SAM, id_gen, Entity::AI, GridTile::new(5, 17));
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

        pub fn find_unit_tile(&self, unit_id: usize) -> Option<GridTile> {
            self.units_by_tile
                .iter()
                .find(|(_, stack)| stack.units.contains_key(&unit_id))
                .map(|(tile, _)| *tile)
        }

        pub fn remove_unit(&mut self, tile: GridTile, unit_id: usize) -> bool {
            if let Some(units_at_tile) = self.units_by_tile.get_mut(&tile) {
                if units_at_tile.units.remove(&unit_id).is_some() {
                    return true;
                }
            }
            false
        }

        pub fn pop_unit(&mut self, tile: GridTile, unit_id: usize) -> Option<UnitInfo> {
            self.units_by_tile
                .get_mut(&tile)
                .and_then(|stack| stack.units.remove(&unit_id))
        }
    }

    pub struct DestroyedUnit {
        pub unit_type: UnitTilesEnum,
        pub location: GridTile,
        frame_itr: Box<dyn Iterator<Item = usize>>,
    }

    impl DestroyedUnit {
        pub fn new(unit_type: UnitTilesEnum, location: GridTile) -> Self {
            DestroyedUnit {
                unit_type,
                location,
                frame_itr: Box::new(UnitTileTextures::get_oneshot_seq_it(4, 20)),
            }
        }

        pub fn next_frame(&mut self) -> Option<usize> {
            self.frame_itr.next()
        }
    }

    pub fn unit_has_destruction_animation(unit_type: UnitTilesEnum) -> bool {
        matches!(unit_type, UnitTilesEnum::Tank)
    }

    pub struct DamageAssessment {
        damage_matrix: Vec<Vec<f32>>,
    }

    impl DamageAssessment {
        // Columns: Tank=0, Infantry=1, Scout=2, Engineers=3, APC=4,
        //          RocketArty=5, Artillery=6, AttackHeli=7, TransportHeli=8, Plane=9, SAM=10
        pub fn new() -> DamageAssessment {
            let n = UnitTilesEnum::End as usize;
            let mut m = vec![vec![1.0f32; n]; n];

            m[UnitTilesEnum::Tank as usize] =
                vec![1.0, 0.7, 1.5, 1.2, 1.3, 2.0, 2.0, 0.1, 0.1, 0.0, 1.5];
            m[UnitTilesEnum::Infantry as usize] =
                vec![0.3, 1.0, 1.2, 1.5, 0.5, 1.5, 1.5, 0.3, 0.3, 0.1, 0.5];
            m[UnitTilesEnum::Scout as usize] =
                vec![0.2, 0.8, 1.0, 1.0, 0.6, 1.0, 1.0, 0.1, 0.2, 0.0, 0.5];
            m[UnitTilesEnum::Engineers as usize] =
                vec![0.4, 1.2, 1.0, 1.0, 0.7, 1.0, 1.0, 0.1, 0.2, 0.0, 0.5];
            m[UnitTilesEnum::APC as usize] =
                vec![0.5, 1.5, 2.0, 1.5, 1.0, 1.5, 1.5, 0.2, 0.3, 0.0, 0.8];
            m[UnitTilesEnum::RocketArty as usize] =
                vec![1.5, 2.0, 2.0, 2.0, 1.5, 1.0, 1.0, 0.3, 0.4, 0.1, 1.5];
            m[UnitTilesEnum::Artillery as usize] =
                vec![1.5, 2.5, 2.0, 2.0, 1.5, 1.0, 1.0, 0.2, 0.3, 0.0, 1.5];
            m[UnitTilesEnum::AttackHeli as usize] =
                vec![2.5, 1.5, 2.0, 1.5, 2.0, 2.0, 2.0, 1.0, 2.0, 0.5, 1.5];
            m[UnitTilesEnum::TransportHeli as usize] =
                vec![0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.0, 0.1];
            m[UnitTilesEnum::Plane as usize] =
                vec![2.0, 1.5, 2.0, 1.5, 2.0, 3.0, 3.0, 1.0, 2.0, 1.0, 0.5];
            m[UnitTilesEnum::SAM as usize] =
                vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, 3.0, 2.5, 0.1];

            DamageAssessment { damage_matrix: m }
        }

        // Combined-arms unit types that qualify for the variety bonus.
        fn variety_bonus(units: &[&UnitInfo]) -> f32 {
            let combined_arms = [
                UnitTilesEnum::Tank,
                UnitTilesEnum::APC,
                UnitTilesEnum::Infantry,
                UnitTilesEnum::AttackHeli,
            ];
            let count = combined_arms
                .iter()
                .filter(|&&vt| units.iter().any(|u| u.unit_type == vt))
                .count();
            match count {
                0 | 1 => 1.0,
                2 => 1.15,
                3 => 1.30,
                _ => 1.50,
            }
        }

        // Logarithmic coordination bonus when attackers outnumber defenders.
        // No penalty for being outnumbered; the smaller raw sum already reflects that.
        fn numerical_bonus(attacker_count: usize, defender_count: usize) -> f32 {
            let ratio = attacker_count as f32 / defender_count.max(1) as f32;
            (1.0 + 0.2 * ratio.ln()).max(1.0)
        }

        /// Total damage dealt by `attackers` to the `defenders` group.
        ///
        /// In a 1v1 this equals exactly `damage_matrix[attacker][defender]`.
        /// Each additional attacker adds their proportional damage; the variety
        /// and numerical coordination bonuses then scale the final total.
        pub fn attack_damage(&self, attackers: &[&UnitInfo], defenders: &[&UnitInfo]) -> f32 {
            if attackers.is_empty() || defenders.is_empty() {
                return 0.0;
            }
            let n_def = defenders.len() as f32;
            // Each attacker contributes its average effectiveness across all defender types.
            let raw: f32 = attackers
                .iter()
                .map(|a| {
                    defenders
                        .iter()
                        .map(|d| self.damage_matrix[a.unit_type as usize][d.unit_type as usize])
                        .sum::<f32>()
                        / n_def
                })
                .sum();

            raw * Self::variety_bonus(attackers)
                * Self::numerical_bonus(attackers.len(), defenders.len())
        }

        /// Resolve simultaneous combat between two sides.
        /// Returns `(damage dealt to side_a, damage dealt to side_b)`.
        pub fn resolve_combat(&self, side_a: &[&UnitInfo], side_b: &[&UnitInfo]) -> (f32, f32) {
            (
                self.attack_damage(side_b, side_a),
                self.attack_damage(side_a, side_b),
            )
        }

        pub fn damage_multiplier(&self, source: UnitTilesEnum, target: UnitTilesEnum) -> f32 {
            self.damage_matrix[source as usize][target as usize]
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        fn make_unit(tp: UnitTilesEnum) -> UnitInfo {
            UnitInfo::new(tp, &mut UnitId::new(), Entity::Player, GridTile::new(0, 0))
        }

        #[test]
        fn one_v_one_equals_matrix_value() {
            let da = DamageAssessment::new();
            let tank = make_unit(UnitTilesEnum::Tank);
            let inf = make_unit(UnitTilesEnum::Infantry);
            let damage = da.attack_damage(&[&tank], &[&inf]);
            let expected = da.damage_multiplier(UnitTilesEnum::Tank, UnitTilesEnum::Infantry);
            assert!(
                (damage - expected).abs() < 1e-5,
                "1v1 damage {damage} should equal matrix value {expected}"
            );
        }

        #[test]
        fn combined_arms_beats_mono_type_same_size() {
            let da = DamageAssessment::new();
            let enemy = make_unit(UnitTilesEnum::Infantry);
            let t = make_unit(UnitTilesEnum::Tank);
            let a = make_unit(UnitTilesEnum::APC);
            let i = make_unit(UnitTilesEnum::Infantry);
            let h = make_unit(UnitTilesEnum::AttackHeli);
            let t2 = make_unit(UnitTilesEnum::Tank);
            let t3 = make_unit(UnitTilesEnum::Tank);
            let t4 = make_unit(UnitTilesEnum::Tank);

            let combined = da.attack_damage(&[&t, &a, &i, &h], &[&enemy]);
            let mono = da.attack_damage(&[&t, &t2, &t3, &t4], &[&enemy]);
            assert!(
                combined > mono,
                "combined arms ({combined}) should outperform mono-type ({mono})"
            );
        }

        #[test]
        fn numerical_bonus_applies_when_outnumbering() {
            let da = DamageAssessment::new();
            let enemy = make_unit(UnitTilesEnum::Infantry);
            let t1 = make_unit(UnitTilesEnum::Tank);
            let t2 = make_unit(UnitTilesEnum::Tank);
            let t3 = make_unit(UnitTilesEnum::Tank);
            let lone = make_unit(UnitTilesEnum::Tank);

            let dmg_3v1 = da.attack_damage(&[&t1, &t2, &t3], &[&enemy]);
            let dmg_1v1 = da.attack_damage(&[&lone], &[&enemy]);
            // 3 attackers stack raw damage AND gain a coordination bonus,
            // so the result must exceed a naive 3× multiple.
            assert!(
                dmg_3v1 > 3.0 * dmg_1v1,
                "3v1 ({dmg_3v1}) should exceed 3x 1v1 ({} ) due to numerical bonus",
                3.0 * dmg_1v1
            );
        }

        #[test]
        fn resolve_combat_returns_symmetric_for_equal_forces() {
            let da = DamageAssessment::new();
            let a1 = make_unit(UnitTilesEnum::Tank);
            let b1 = make_unit(UnitTilesEnum::Tank);
            let (dmg_a, dmg_b) = da.resolve_combat(&[&a1], &[&b1]);
            assert!(
                (dmg_a - dmg_b).abs() < 1e-5,
                "equal 1v1 forces should deal identical damage to each other"
            );
        }

        #[test]
        fn add_unit_at_inserts_unit_into_tile_map() {
            let mut id_gen = UnitId::new();
            let mut player_units = PlayerUnits::new();

            let unit_id = player_units.add_unit_at(
                UnitTilesEnum::Tank,
                &mut id_gen,
                Entity::Player,
                GridTile::new(3, 2),
            );

            let tile_units = player_units
                .get_units_at(GridTile::new(3, 2))
                .expect("Tile should exist");
            assert_eq!(tile_units.len(), 1);
            assert!(tile_units.contains_key(&unit_id));
        }

        #[test]
        fn move_unit_moves_unit_between_tiles() {
            let mut id_gen = UnitId::new();
            let mut player_units = PlayerUnits::new();

            let unit_id = player_units.add_unit_at(
                UnitTilesEnum::Tank,
                &mut id_gen,
                Entity::Player,
                GridTile::new(3, 2),
            );
            let moved = player_units.move_unit(GridTile::new(3, 2), unit_id, GridTile::new(4, 3));

            assert!(
                moved,
                "move_unit should succeed when start_tile contains the unit"
            );
            let source_units = player_units
                .get_units_at(GridTile::new(3, 2))
                .expect("source tile should exist");
            assert!(
                source_units.is_empty(),
                "old tile should be empty after move"
            );

            let target_units = player_units
                .get_units_at(GridTile::new(4, 3))
                .expect("Target tile should exist");
            assert_eq!(target_units.len(), 1);
            let moved_unit = target_units
                .get(&unit_id)
                .expect("Moved unit should be present");
            assert_eq!(moved_unit.location, GridTile::new(4, 3));
        }

        #[test]
        fn move_unit_fails_if_start_tile_is_wrong() {
            let mut id_gen = UnitId::new();
            let mut player_units = PlayerUnits::new();

            let unit_id = player_units.add_unit_at(
                UnitTilesEnum::Tank,
                &mut id_gen,
                Entity::Player,
                GridTile::new(3, 2),
            );
            let moved = player_units.move_unit(GridTile::new(1, 1), unit_id, GridTile::new(4, 3));

            assert!(
                !moved,
                "move_unit should fail when the unit is not present at start_tile"
            );
            let original_units = player_units
                .get_units_at(GridTile::new(3, 2))
                .expect("Original tile should still exist");
            assert!(original_units.contains_key(&unit_id));
        }
    }
}
