use std::{time::Duration, fmt::Debug, sync::{Arc, Weak}, io::Write, f32::NAN};

use tokio::sync::RwLock;
use futures::lock;
use rand::Rng;

use crate::{game_objects::{game_object::{VisitableStructure, LogicObject}, buildings::start_obj::StartObj}, model::results::{LogicResult, GameObjects}, controller::position::{self, Position}, rendering::sprites::sprite_mapping::Sprite};

use super::{map_chunk::{MapChunk, ChunkInfo}, worker::Worker};


//the idea is to hold information about the current level, this includes the chunks that are currently loaded, the structures that are currently loaded and the borders of the level. This is supposed to be used for stuff like pathfinding and collision detection
#[derive(Debug)]
pub struct StratLevel{
    pub(crate) blocking_chunks: Vec<Box<dyn MapChunk>>,
    pub(crate) non_blocking_chunks: Vec<Box<dyn MapChunk>>,
    pub(crate) structures: Vec<Arc<RwLock<dyn VisitableStructure>>>,
    pub(crate) logic_objects: Vec<Arc<RwLock<dyn StrategyLogicObject>>>,
    pub(crate) logic_objects_id_counter: u64,
    pub(crate) borders: ChunkInfo,
    pub(crate) id: u32,
}

impl StratLevel{
    pub(crate) fn new(borders: ChunkInfo) -> Self{
        StratLevel{
            blocking_chunks: Vec::new(),
            non_blocking_chunks: Vec::new(),
            structures: Vec::new(),
            logic_objects: Vec::new(),
            logic_objects_id_counter: 0,
            borders,
            id: 0,
        }
    }

    pub(crate) async fn initialize_initial_units(&mut self){
        for logic_object in &mut self.logic_objects{
            let mut lock = logic_object.write().await;
            lock.initialize_behavior(&self.blocking_chunks, &self.structures);
        }
    }

    pub(crate) async fn add_logic_object(&mut self, logic_object: Arc<RwLock<dyn StrategyLogicObject>>){
        logic_object.write().await.set_id(self.logic_objects_id_counter);
        self.logic_objects.push(logic_object);
        self.logic_objects_id_counter += 1;
    }

    pub(crate) async fn initialize(&mut self) -> GameObjects{
        println!("initializing level");
        let mut ret : GameObjects = Vec::new();
        let mut rand = rand::thread_rng();
        let other_base = StartObj::new(Position::new(4.0, -2.0), self.logic_objects_id_counter);
        let third_base = StartObj::new(Position::new(-2.0, 2.0), self.logic_objects_id_counter);
        let start_base = StartObj::new(Position::new(0.0, 0.0), self.logic_objects_id_counter);
        let third_base_arxed = Arc::new(RwLock::new(third_base));
        self.add_logic_object(third_base_arxed.clone()).await;
        self.structures.push(third_base_arxed.clone());
        ret.push(third_base_arxed.clone());
        let other_arxed_base = Arc::new(RwLock::new(other_base));
        self.add_logic_object(other_arxed_base.clone()).await;
        self.structures.push(other_arxed_base.clone());
        ret.push(other_arxed_base.clone());
        let arxed_base = Arc::new(RwLock::new(start_base));
        self.add_logic_object(arxed_base.clone()).await;
        self.structures.push(arxed_base.clone());
        ret.push(arxed_base.clone());
       for i in 0..2000{
            let dist_from_base: f32 = rand.gen_range(15.0..60.0);
            let mut x = rand.gen_range(-dist_from_base.sqrt()..=dist_from_base.sqrt());
            let mut y = (dist_from_base - f32::powi(x, 2)).sqrt();
            if rand.gen_bool(0.5){
                y *= -1.0;
            }
            loop{
                let mut eligible = true;
                for structure in &self.structures{
                    if structure.read().await.get_blocking_chunk().contains(&Position::new(x, y)){
                        eligible = false;
                    }
                }
                if eligible{
                    break;
                }
                x = rand.gen_range(-dist_from_base.sqrt()..=dist_from_base.sqrt());
                y = (dist_from_base - f32::powi(x, 2)).sqrt();
                if y == NAN || y == -NAN{
                    println!("Dist: {dist_from_base} x: {x} y: {y}");
                }
                if rand.gen_bool(0.5){
                    y *= -1.0;
                }
                //genera
            }

            let worker = Worker::new(Some(arxed_base.clone()), Position::new(x, y), self.logic_objects_id_counter);
            let arxed = Arc::new(RwLock::new(worker));
            ret.push(arxed.clone());
            self.add_logic_object(arxed).await;
        }
    /*    let worker = Worker::new(Some(arxed_base.clone()), Position::new(0.98684025, 4.901647), self.logic_objects_id_counter);
        let arxed = Arc::new(RwLock::new(worker));
        ret.push(arxed.clone());
        self.add_logic_object(arxed).await;
        */
        self.initialize_initial_units().await; 
        println!("finished initializing level");
        ret
    
    }


}

impl LogicObject for StratLevel{
    fn process_logic(&mut self, delta_time: Duration) -> LogicResult {
        if self.logic_objects.len() == 0{
            return LogicResult::None;
        }
        for logic_object in &mut self.logic_objects{
            let mut lock = logic_object.blocking_write();
            lock.process_logic(delta_time, &mut self.blocking_chunks, &mut self.structures);
        }
        LogicResult::None
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn get_id(&self) -> u32 {
        self.id
    }
    
}

pub(crate) trait StrategyLogicObject : Debug{
    fn process_logic(&mut self, delta_time: Duration, blockers: &mut Vec<Box<dyn MapChunk>>, structures: &mut Vec<Arc<RwLock<dyn VisitableStructure>>>) -> LogicResult;
    fn initialize_behavior(&mut self, blockers: &Vec<Box<dyn MapChunk>>, structures: &Vec<Arc<RwLock<dyn VisitableStructure>>>); //this is supposed to be called after the object has been added to the level, possibly needs all necessary chunks for pathfinding
    fn set_id(&mut self, id: u64);
    fn get_id(&self) -> u64;
}