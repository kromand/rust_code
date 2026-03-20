pub mod infstrt {
    use crate::defines::*;
    use macroquad::prelude::*;
    
    pub struct InfrastructureTextures {
        factory_txtr: Vec<Texture2D>,
        factory_itr:  Box<dyn Iterator<Item=usize>>
    }

    impl InfrastructureTextures {
        pub fn get_repeat_seq_it(len:usize, repeat:usize) -> impl Iterator<Item=usize> {
            (0..len.to_owned()).flat_map(move |n| std::iter::repeat(n).take(repeat)).cycle()
        }
        pub fn get_unit_texture(self: &mut InfrastructureTextures, infr_type: InfrastructureEnum) -> &Texture2D {
            
            //calling unwrap here since any frame sequence should be cyclical, infinte
            //TODO: since death sequence will non-cyclical, need to implement that as well
            //TODO: the iterator might need to move to unit info since each animation will be separate per unit
            let frame_index = self.factory_itr.next().unwrap();

            match infr_type {
                InfrastructureEnum::Fatory => &self.factory_txtr[frame_index],
                _ => {
                    dbg!(infr_type);
                    unreachable!()},
            }
        }
    }

    pub async fn load_default_infra_textures(frame_repeat_rate: usize) -> Result<InfrastructureTextures, macroquad::Error> {
        let vct = vec![load_texture("assets/factory_base.png").await?,
            load_texture("assets/factory_1.png").await?,
            load_texture("assets/factory_2.png").await?,
            load_texture("assets/factory_3.png").await?,
            load_texture("assets/factory_4.png").await?];
        Ok(
            InfrastructureTextures {
                factory_itr: Box::new(InfrastructureTextures::get_repeat_seq_it(vct.len(),frame_repeat_rate)),
                factory_txtr: vct,
            }
        )
    }

    pub struct InfrObject {
        infr_type : InfrastructureEnum,
        count: usize,
        location: GridTile,
        owner: Entity,
        health: usize,

    }

    impl InfrObject {
        pub fn new(obj_type: InfrastructureEnum, loc: GridTile, own:Entity) -> InfrObject {
            InfrObject {
                infr_type: obj_type,
                count: 0,
                location: loc,
                owner:own,
                health:100,
            }
        }
    }
}