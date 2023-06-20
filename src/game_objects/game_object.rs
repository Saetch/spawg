use crate::rendering::vertex::Vertex;


///!!!This is the trait that all drawable objects have to implement, the implementations here are defaults and should be overridden if necessary !!!
pub(crate) trait DrawableObject{
    
/* This is the output for an object that fills out the top right corner of the screen @(0,5/0,5) and uses its whole texture.
    Vertex { position: [0.0, 0.0], tex_i: 0, tex_coords: [0.0, 1.0], },
    Vertex { position: [1.0, 1.0],  tex_i: 0, tex_coords: [1.0, 0.0], }, 
    Vertex { position: [0.0, 1.0],  tex_i: 0, tex_coords: [0.0, 0.0], }, 
    Vertex { position: [0.0, 0.0], tex_i: 0, tex_coords: [0.0, 1.0], }, 
    Vertex { position: [1.0, 0.0],  tex_i: 0, tex_coords: [1.0, 1.0], },
    Vertex { position: [1.0, 1.0],  tex_i: 0, tex_coords: [1.0, 0.0], }, 
    */

    /*
     * Default implementation for a somewhat rectangle-shaped object
     */
    #[inline(always)]
    fn construct_vertices(&self, camera_position: (f32, f32), window_dimensions_ingame: (f64, f64)) -> Vec<Vertex>{
        let x = (( self.get_position().0 - camera_position.0 ) as f32) /window_dimensions_ingame.0 as f32;
        let y = (( self.get_position().1 - camera_position.1 ) as f32) /window_dimensions_ingame.1 as f32;
        let size_x = self.get_size() / (window_dimensions_ingame.0 as f32);
        let size_y = self.get_size() / (window_dimensions_ingame.1 as f32);
        let tex_i = self.get_tex_i();
        //in wgpu: -1 bottom, 1 top, so this needs to be switched around, since in the game logic, a higher y is lower on the screen, just like actual screen coordinates. I don't know why wgpu is doing this, but they want to be an actual math graph.
        let pre_set: f32 = -1.0;
        //in wgpu, the vertex faces need to be specified in counter-clockwise order
        vec!(
            Vertex{         //TOP RIGHT CORNER
                position: [x+size_x, pre_set* (y-size_y)],
                texture_id: tex_i,
                tex_coords: self.top_right_coords(),
            },
            Vertex{         //TOP LEFT CORNER
                position: [x-size_x,  pre_set* (y-size_y) ],
                texture_id: tex_i,
                tex_coords: self.top_left_coords(),
            },
            Vertex{         //BOTTOM LEFT CORNER
                position: [x-size_x, pre_set* (y+size_y)],
                texture_id: tex_i,
                tex_coords: self.bottom_left_coords(),
            },
            Vertex{         //BOTTOM RIGHT CORNER
                 position: [x+size_x, pre_set* (y+size_y)],
                 texture_id: tex_i,
                 tex_coords: self.bottom_right_coords(),
             },

            
        )
    }


    fn get_position(&self) -> (f32, f32);
    fn get_size(&self) -> f32;
    fn get_tex_i(&self) -> u32;



    //Consider making this interior mutable, in order to speed up access to these 
    fn process_animation(&mut self, delta_time: f64);


    /**
     * the relative position of the top right vertex if overlayed on top of the image that is to be drawn
     */
    #[inline(always)]
    fn top_right_coords(&self) -> [f32; 2]{
        //const expressions are evaluated at compile time and thus can be used to explicitly tell the compiler to optimize this. Would most likely happen anyway.
        const RET: [f32; 2] = [1.0, 0.0];
        return RET;
    }
    
    /**
     * the relative position of the top left vertex if overlayed on top of the image that is to be drawn
     */
    #[inline(always)]
    fn top_left_coords(&self) -> [f32; 2]{
        const RET: [f32; 2] = [0.0, 0.0];
        return RET;
    }
    
    /**
     * the relative position of the bottom right vertex if overlayed on top of the image that is to be drawn
     */
    #[inline(always)]
    fn bottom_right_coords(&self) -> [f32; 2]{
        const RET: [f32; 2] = [1.0, 1.0];
        return RET;
    }
    
    /**
     * the relative position of the bottom left vertex if overlayed on top of the image that is to be drawn
     */
    #[inline(always)]
    fn bottom_left_coords(&self) -> [f32; 2]{
        const RET: [f32; 2] = [0.0, 1.0];
        return RET;
    }
}