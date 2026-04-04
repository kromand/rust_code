pub mod infstrt {
    use crate::defines::*;
    use macroquad::prelude::*;
    use std::sync::Arc;

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


        pub fn get_infra_texture(
            self: &mut InfrastuctureTextures,
            i_type: InfrastructureEnum,
        ) -> &Texture2D {
            &self.infra_textures[i_type as usize].get_next_texture(i_type)
        }
    }

    pub struct InfrObject {
        pub infr_type: InfrastructureEnum,
        count: usize,
        pub location: GridTile,
        pub owner: Entity,
        health: usize,
        pub detected:bool,
    }

    impl InfrObject {
        pub fn new(obj_type: InfrastructureEnum, loc: GridTile, own: Entity) -> InfrObject {
            InfrObject {
                infr_type: obj_type,
                count: 0,
                location: loc,
                owner: own,
                health: 100,
                detected:false,
            }
        }
    }
    pub struct InfrastructureContainer {
        pub infr_objects: Vec<Arc<InfrObject>>,
    }

    impl InfrastructureContainer {
        pub fn new() -> InfrastructureContainer {
            InfrastructureContainer {
                infr_objects: Vec::<Arc<InfrObject>>::new(),   
            }
        }
        pub fn Add(self: &mut InfrastructureContainer, new_infr: Arc<InfrObject>) {
            self.infr_objects.push(new_infr);
        }
        //add few test infra objects (mines, factory...)
        pub fn Init(self: &mut InfrastructureContainer) {
            //add sample object
            self.infr_objects.push(Arc::new(InfrObject::new(InfrastructureEnum::Fatory, (18, 5), Entity::AI)));
            self.infr_objects.push(Arc::new(InfrObject::new(InfrastructureEnum::Mines, (19, 5), Entity::AI)));
        }
    }
}
