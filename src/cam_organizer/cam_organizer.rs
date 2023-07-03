
use std::{sync::{Arc, atomic::AtomicBool}, time::{Duration, Instant}, cell::RefCell, rc::Rc, pin::Pin};

use async_std::{sync::RwLock, future};
use bytemuck::{Pod, Zeroable};
use flume::{Sender, r#async};
use futures::{join, future::{join_all, BoxFuture}, Future};

use crate::{model::model::GameObjectList, rendering::{wgpurenderer::RenderChunk, sprite_instance::SpriteInstance, sprites::vertex_configration::VertexConfigrationTrait}, controller::controller::{SharablePosition, Direction}, game_objects::game_object::{self, DrawableObject}};

const CAMERA_SPEED: f32 = 15.0;
pub(crate) struct CamOrganizer{
    state: u32,
    pub(crate) game_objects: GameObjectList,
    cam_pos: SharablePosition,
    cam_proportions: Arc<RwLock<(f32, f32)>>,
    cam_directions: Arc<RwLock<(Direction, Direction)>>,
    sender: Sender<(Vec<RenderChunk>, CamState)>,
    pub(crate) running: Arc<AtomicBool>,  //<-- this is used to indicate whether the program should exit or not
    
}

impl CamOrganizer{

    pub(crate) fn new(game_objects : GameObjectList, cam_pos: SharablePosition, sender: Sender<(Vec<RenderChunk>, CamState)>, cam_proportions: Arc<RwLock<(f32, f32)>>, cam_directions: Arc<RwLock<(Direction, Direction)>>, running: Arc<AtomicBool>) -> CamOrganizer{
        CamOrganizer{
            state: 0,
            game_objects: game_objects,
            cam_pos: cam_pos,
            cam_proportions: cam_proportions,
            cam_directions,
            sender: sender,
            running: running,  //<-- this is used to indicate whether the program should exit or not

        }
    }

    pub(crate) async fn run(&self){
        let mut loop_helper = spin_sleep::LoopHelper::builder()
        .report_interval_s(1.0) // report every half a second
        .build_with_target_rate(144.0);
        let mut current_fps = None;

        while self.running.load(std::sync::atomic::Ordering::Relaxed) {     
            loop_helper.loop_sleep();
            let delta = loop_helper.loop_start();
            if let Some(fps) = loop_helper.report_rate() {
                current_fps = Some(fps);
                println!("FPS: {}", fps);
            }
            let render_ops: Vec<RenderChunk> = Vec::with_capacity(10);
            let cell = Rc::new(RefCell::new(render_ops));
            let lock = self.game_objects.as_ref().read().await;
            let mut futures_vec = Vec::new();
            for obj in lock.iter(){
            
                let fut =  Self::process_object(obj, cell.clone()) ;
                futures_vec.push(fut);
            }

            let fut = self.compute_camera(delta.as_micros() as f32);

            
            let vec_join = join_all(futures_vec);
            let (_ , cam_state) = futures::join!(vec_join, fut);
            drop(lock);

            let res = self.sender.send((Rc::try_unwrap(cell).unwrap().into_inner(), cam_state));
            if let Err(e) = res{ 
                println!("Could not send rendering info to renderer thread: {}", e);
            }
            
        }


    


    }


#[inline(always)]
    async fn process_object(obj: &Arc<RwLock<dyn DrawableObject + Send + Sync>>, render_ops: Rc<RefCell<Vec<RenderChunk>>>){
        let obj_lock = obj.read().await;
                let texture_id = *obj_lock.get_texture() as u32;
                let position = obj_lock.get_position();
            
                let vertex_configration = obj_lock.get_vertex_configuration();
                let mut borrow = render_ops.borrow_mut();
                let already_queued = borrow.iter_mut().find(|chunk| chunk.vertex_conf as u32 == *vertex_configration as u32);
                if let Some(queue) = already_queued{
                    queue.instance_buffer.push(SpriteInstance {
                        position: [position.x, position.y],
                        texture_id: texture_id,
                    });
                }else{
                    let instance_buffer = 
                        vec![SpriteInstance {
                            position: [position.x, position.y],
                            texture_id: texture_id,
                        }];
                    let render_chunk = RenderChunk{
                        vertex_conf: *vertex_configration,
                        instance_buffer: instance_buffer,   //this is because a sprite consists of 2 triangles at the moment. If this changes and can be dynamically set, this should be updated
                    };
                    borrow.push(render_chunk);
                }

    }


#[inline(always)]
    async fn compute_camera(&self, delta_us: f32) -> CamState {
        let cam_directions = self.cam_directions.read();
        let cam_pos = self.cam_pos.write();
        let cam_size = self.cam_proportions.read();
        
        let(cam_directions, mut cam_pos, cam_size) = join!(cam_directions, cam_pos, cam_size);

        //compute x direction
        let x_direction = match cam_directions.0{
            Direction::Positive => CAMERA_SPEED*1.0,
            Direction::Negative => CAMERA_SPEED*-1.0,
            _ => 0.0,
        
        };
        cam_pos.x += x_direction * delta_us / 1_000_000.0;
        //compute y direction
        let y_direction = match cam_directions.1{
            Direction::Positive => CAMERA_SPEED*1.0,
            Direction::Negative => CAMERA_SPEED*-1.0,
            _ => 0.0,
        };
        cam_pos.y += y_direction * delta_us / 1_000_000.0;

        CamState{
            cam_size: [cam_size.0, cam_size.1],
            cam_pos: [cam_pos.x, cam_pos.y],
        }



    }

}


#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub(crate) struct CamState{
    pub(crate) cam_size: [f32; 2],
    pub(crate) cam_pos: [f32; 2],
}