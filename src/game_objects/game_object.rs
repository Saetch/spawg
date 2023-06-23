use crate::rendering::{vertex::Vertex, sprites::sprite_mapping::Sprite};


///!!!This is the trait that all drawable objects have to implement, the implementations here are defaults and should be overridden if necessary !!!
pub(crate) trait DrawableObject{
    

    fn get_position(&self) -> (f32, f32);
    fn get_size(&self) -> f32;
    fn get_texture(&self) -> Sprite;



    //Consider making this interior mutable, in order to speed up access to these 
    fn process_animation(&mut self, delta_time: f64);

    #[inline(always)]
    fn vertices(&self) -> Vec<Vertex>{
        vec![            
        Vertex { position: [0.5, -0.5], tex_coords: [1.0, 1.0]}, // A
        Vertex { position: [0.5, 0.5], tex_coords: [1.0, 0.0]}, // B
        Vertex { position: [-0.5, 0.5], tex_coords: [0.0, 0.0] }, // C
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 1.0] }, // D]
        ]
    }
}