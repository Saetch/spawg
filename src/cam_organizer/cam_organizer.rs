
use std::{sync::{Arc, atomic::AtomicBool}, time::Duration};

use async_std::sync::RwLock;
use flume::Sender;

use crate::{model::model::GameObjectList, rendering::{wgpurenderer::RenderChunk, sprite_instance::SpriteInstance, sprites::vertex_configration::VertexConfigrationTrait}, controller::controller::{SharablePosition, Direction}, game_objects::game_object};


const CAMERA_SPEED: f32 = 1.0;
pub(crate) struct CamOrganizer{
    state: u32,
    pub(crate) game_objects: GameObjectList,
    cam_pos: SharablePosition,
    cam_proportions: Arc<RwLock<(f32, f32)>>,
    cam_directions: Arc<RwLock<(Direction, Direction)>>,
    sender: Sender<Vec<RenderChunk>>,
    pub(crate) running: Arc<AtomicBool>,  //<-- this is used to indicate whether the program should exit or not
    
}

impl CamOrganizer{

    pub(crate) fn new(game_objects : GameObjectList, cam_pos: SharablePosition, sender: Sender<Vec<RenderChunk>>, cam_proportions: Arc<RwLock<(f32, f32)>>, cam_directions: Arc<RwLock<(Direction, Direction)>>, running: Arc<AtomicBool>) -> CamOrganizer{
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
        .report_interval_s(0.5) // report every half a second
        .build_with_target_rate(144.0);
        let mut last_time = std::time::Instant::now();
        let mut current_fps = None;

        let mut dummy_counter = 0;
        while self.running.load(std::sync::atomic::Ordering::Relaxed) {     


            let delta = loop_helper.loop_start(); // or .loop_start_s() for f64 seconds.  This is just here to show and lock fps
            if let Some(fps) = loop_helper.report_rate() {
                current_fps = Some(fps.round());
                dummy_counter += 1;
                if dummy_counter > 3{
                    println!("FPS: {}", current_fps.unwrap());
                    dummy_counter = 0;
                }
            }



            self.compute_camera(delta).await;


            let mut render_ops: Vec<RenderChunk> = Vec::new();
            let x_os = self.cam_pos.read().await.x;
            let y_os = self.cam_pos.read().await.y;
            let lock = self.game_objects.as_ref().read().await;
            
            lock.iter().for_each(|obj| {
                let texture_id = *obj.get_texture() as u32;
                let position = obj.get_position();
            
                let vertex_configration = obj.get_vertex_configuration();
            
                let already_queued = render_ops.iter_mut().find(|chunk| chunk.vertex_conf as u32 == *vertex_configration as u32);
                if let Some(queue) = already_queued{
                    queue.instance_buffer.push(SpriteInstance {
                        position: [position.x/12.0-x_os, position.y/7.0-y_os],
                        texture_id: texture_id,
                    });
                }else{
                    let vertex_buffer = Vec::from(vertex_configration.get_vertices((24, 14)));
                    let instance_buffer = 
                        vec![SpriteInstance {
                            position: [position.x-x_os, position.y-y_os],
                            texture_id: texture_id,
                        }];
                    let render_chunk = RenderChunk{
                        vertex_conf: *vertex_configration,
                        vertex_buffer: vertex_buffer,
                        instance_buffer: instance_buffer,   //this is because a sprite consists of 2 triangles at the moment. If this changes and can be dynamically set, this should be updated
                    };
                    render_ops.push(render_chunk);
                }
            
            });
            drop(lock);

            let res = self.sender.send(render_ops);
            if let Err(e) = res{
                println!("Could not send rendering info to renderer thread: {}", e);
            }
            loop_helper.loop_sleep(); // sleeps to acheive the target rate
        }


    


    }


    async fn compute_camera(&self, delta_ms: Duration){
        let mut cam_pos = self.cam_pos.write().await;
        let cam_directions = self.cam_directions.read().await;

        
        //compute x direction
        let x_direction = match cam_directions.0{
            Direction::Positive => CAMERA_SPEED*1.0,
            Direction::Negative => CAMERA_SPEED*-1.0,
            Direction::None => 0.0,
        };
        cam_pos.x += x_direction * delta_ms.as_millis() as f32 / 1000.0;
        //compute y direction
        let y_direction = match cam_directions.1{
            Direction::Positive => CAMERA_SPEED*1.0,
            Direction::Negative => CAMERA_SPEED*-1.0,
            Direction::None => 0.0,
        };
        cam_pos.y += y_direction * delta_ms.as_millis() as f32 / 1000.0;


    }

}