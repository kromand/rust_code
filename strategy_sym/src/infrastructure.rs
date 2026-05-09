pub mod infstrt {
    use crate::defines::*;
    use crate::units::Units::{UnitId, UnitInfo};
    use macroquad::prelude::*;
    use std::collections::HashSet;
    use std::collections::VecDeque;
    use std::sync::Arc;
    use std::sync::Mutex;

    /***************************** Textures *****************************/
    pub struct TextureContainer {
        obj_txtr: Vec<Texture2D>,
        obj_itr: Box<dyn Iterator<Item = usize>>,
    }

    impl TextureContainer {
        pub fn get_repeat_seq_it(len: usize, repeat: usize) -> impl Iterator<Item = usize> {
            (0..len.to_owned())
                .flat_map(move |n| std::iter::repeat(n).take(repeat))
                .cycle()
        }

        pub fn get_first_frame_only() -> impl Iterator<Item = usize> {
            (0..1).cycle()
        }

        pub fn get_next_texture(
            self: &mut TextureContainer,
            infr_type: InfrastructureEnum,
        ) -> &Texture2D {
            //calling unwrap here since any frame sequence should be cyclical, infinte
            //TODO: since death sequence will non-cyclical, need to implement that as well
            //TODO: the iterator might need to move common file since units might use animation too
            let frame_index = self.obj_itr.next().unwrap();

            &self.obj_txtr[frame_index]
        }
    }

    pub struct InfrastuctureTextures {
        infra_textures: Vec<TextureContainer>,
    }
    impl InfrastuctureTextures {
        pub async fn new() -> Result<Box<InfrastuctureTextures>, macroquad::Error> {
            let vct = vec![
                InfrastuctureTextures::load_default_factory_textures(20).await?,
                InfrastuctureTextures::load_default_mines_textures().await?,
                InfrastuctureTextures::load_default_airfield_textures().await?,
            ];

            Ok(Box::new(InfrastuctureTextures {
                infra_textures: vct,
            }))
        }

        pub async fn load_default_factory_textures(
            frame_repeat_rate: usize,
        ) -> Result<TextureContainer, macroquad::Error> {
            let vct = vec![
                load_texture("assets/factory_base.png").await?,
                load_texture("assets/factory_1.png").await?,
                load_texture("assets/factory_2.png").await?,
                load_texture("assets/factory_3.png").await?,
                load_texture("assets/factory_4.png").await?,
            ];
            Ok(TextureContainer {
                obj_itr: Box::new(TextureContainer::get_repeat_seq_it(
                    vct.len(),
                    frame_repeat_rate,
                )),
                obj_txtr: vct,
            })
        }
        pub async fn load_default_mines_textures() -> Result<TextureContainer, macroquad::Error> {
            let vct = vec![load_texture("assets/mines.png").await?];
            Ok(TextureContainer {
                obj_itr: Box::new(TextureContainer::get_first_frame_only()),
                obj_txtr: vct,
            })
        }
        pub async fn load_default_airfield_textures() -> Result<TextureContainer, macroquad::Error>
        {
            let vct = vec![load_texture("assets/airport.png").await?];
            Ok(TextureContainer {
                obj_itr: Box::new(TextureContainer::get_first_frame_only()),
                obj_txtr: vct,
            })
        }

        pub fn get_infra_texture(
            self: &mut InfrastuctureTextures,
            i_type: InfrastructureEnum,
        ) -> &Texture2D {
            &self.infra_textures[i_type as usize].get_next_texture(i_type)
        }
    }

    /***************************** Objects *****************************/
    pub struct UnitProduction {
        pub production_queue: VecDeque<UnitTilesEnum>,
        pub allowed_units: HashSet<UnitTilesEnum>,
        elapse_time: usize,
    }

    impl UnitProduction {
        pub fn new(allowed_units: HashSet<UnitTilesEnum>) -> UnitProduction {
            UnitProduction {
                production_queue: VecDeque::<UnitTilesEnum>::new(),
                allowed_units,
                elapse_time: 0,
            }
        }
        pub fn add_to_queue(self: &mut UnitProduction, unit_type: UnitTilesEnum) {
            if self.allowed_units.contains(&unit_type) {
                if self.production_queue.is_empty() {
                    (_, self.elapse_time) = UnitProduction::get_cost_and_time(unit_type);
                }
                self.production_queue.push_back(unit_type);
            }
        }

        pub fn get_next_in_queue(self: &mut UnitProduction) -> Option<UnitTilesEnum> {
            //since first unit is being removed, get the cost and time for the next unit in queue, if exists, and update elapse time accordingly
            if self.production_queue.len() > 1 {
                (_, self.elapse_time) = UnitProduction::get_cost_and_time(self.production_queue[1]);
            } else {
                self.elapse_time = 0;
            }
            self.production_queue.pop_front()
        }

        pub fn remove_from_queue(
            self: &mut UnitProduction,
            remove_index: usize,
        ) -> Option<UnitTilesEnum> {
            self.production_queue.remove(remove_index)
        }

        pub fn get_cost_and_time(unit_type: UnitTilesEnum) -> (usize, usize) {
            match unit_type {
                UnitTilesEnum::Tank => (100, 500),
                UnitTilesEnum::Infantry => (50, 2),
                UnitTilesEnum::Scout => (75, 3),
                UnitTilesEnum::Engineers => (60, 4),
                UnitTilesEnum::APC => (120, 6),
                UnitTilesEnum::RocketArty => (150, 7),
                UnitTilesEnum::Artillery => (130, 6),
                UnitTilesEnum::AttackHeli => (200, 8),
                UnitTilesEnum::TransportHeli => (180, 7),
                _ => unreachable!("Unhandled unit type in get_cost_and_time"), // Default case for End or any undefined unit
            }
        }
        pub fn update_production(
            self: &mut UnitProduction,
            location: GridTile,
            owner: Entity,
        ) -> Option<Box<UnitInfo>> {
            if self.production_queue.is_empty() {
                return None;
            }
            if self.elapse_time > 0 {
                self.elapse_time -= 1;
                None
            } else {
                let unit_type = self.get_next_in_queue().unwrap();
                let mut id_gen = UnitId::new();

                Some(Box::new(UnitInfo::new(
                    unit_type,
                    &mut id_gen,
                    owner,
                    location,
                )))
            }
        }
    }
    pub struct InfrObject {
        pub infr_type: InfrastructureEnum,
        count: usize,
        pub location: GridTile,
        pub owner: Entity,
        health: usize,
        pub detected: bool,
        pub unit_production: Option<UnitProduction>,
    }

    impl InfrObject {
        pub fn new(obj_type: InfrastructureEnum, loc: GridTile, own: Entity) -> InfrObject {
            if obj_type == InfrastructureEnum::Fatory {
                InfrObject {
                    infr_type: obj_type,
                    count: 0,
                    location: loc,
                    owner: own,
                    health: 100,
                    detected: true,
                    unit_production: Some(UnitProduction::new(HashSet::from([
                        UnitTilesEnum::Tank,
                        UnitTilesEnum::Infantry,
                        UnitTilesEnum::Scout,
                        UnitTilesEnum::Engineers,
                        UnitTilesEnum::APC,
                        UnitTilesEnum::RocketArty,
                        UnitTilesEnum::Artillery,
                    ]))),
                }
            } else if obj_type == InfrastructureEnum::Airfield {
                InfrObject {
                    infr_type: obj_type,
                    count: 0,
                    location: loc,
                    owner: own,
                    health: 100,
                    detected: true,
                    unit_production: Some(UnitProduction::new(HashSet::from([
                        UnitTilesEnum::AttackHeli,
                        UnitTilesEnum::TransportHeli,
                    ]))),
                }
            } else {
                InfrObject {
                    infr_type: obj_type,
                    count: 0,
                    location: loc,
                    owner: own,
                    health: 100,
                    detected: true,
                    unit_production: None,
                }
            }
        }
    }

    /***************************** Container *****************************/
    pub struct InfrastructureContainer {
        pub infr_objects: Vec<Arc<Mutex<InfrObject>>>,
    }

    impl InfrastructureContainer {
        pub fn new() -> InfrastructureContainer {
            InfrastructureContainer {
                infr_objects: Vec::<Arc<Mutex<InfrObject>>>::new(),
            }
        }
        pub fn add_infr_objest(
            self: &mut InfrastructureContainer,
            new_infr: Arc<Mutex<InfrObject>>,
        ) {
            self.infr_objects.push(new_infr);
        }
        //add few test infra objects (mines, factory...)
        pub fn Init(self: &mut InfrastructureContainer) {
            //add start objects
            self.infr_objects.push(Arc::new(Mutex::new(InfrObject::new(
                InfrastructureEnum::Fatory,
                (18, 5),
                Entity::AI,
            ))));
            self.infr_objects.push(Arc::new(Mutex::new(InfrObject::new(
                InfrastructureEnum::Fatory,
                (2, 2),
                Entity::Player,
            ))));
            //test adding to production queue, since the first unit will be produced after 5 iterations, we can check if the iterate infrastructure function is working properly
            self.infr_objects
                .last()
                .unwrap()
                .lock()
                .unwrap()
                .unit_production
                .as_mut()
                .unwrap()
                .add_to_queue(UnitTilesEnum::Tank);

            self.infr_objects.push(Arc::new(Mutex::new(InfrObject::new(
                InfrastructureEnum::Airfield,
                (2, 4),
                Entity::Player,
            ))));

            self.infr_objects.push(Arc::new(Mutex::new(InfrObject::new(
                InfrastructureEnum::Mines,
                (19, 5),
                Entity::AI,
            ))));
        }
        pub fn iterate_infrastructure(self: &mut InfrastructureContainer) -> Vec<Box<UnitInfo>> {
            let mut produced_units = Vec::new();

            for infr_arc in self.infr_objects.iter_mut() {
                let mut infr = infr_arc.lock().unwrap();
                let loc = infr.location;
                let own = infr.owner;

                if let Some(unit_prod) = &mut infr.unit_production {
                    // Call update_production and handle the result as needed

                    let new_unit = unit_prod.update_production(loc, own);
                    if let Some(unit_info) = new_unit {
                        // Handle the newly produced unit (e.g., add it to the game state)
                        produced_units.push(unit_info);
                    }
                }
            }
            produced_units
        }
    }
}
