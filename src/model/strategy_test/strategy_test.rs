use std::{time::Duration, fmt::Debug, sync::{Arc, Weak}};

use tokio::sync::RwLock;
use futures::lock;
use rand::Rng;

use crate::{game_objects::{game_object::{VisitableStructure, LogicObject}, buildings::start_obj::StartObj}, model::results::{LogicResult, GameObjects}, controller::position::{self, Position}};

use super::{map_chunk::{MapChunk, ChunkInfo}, worker::Worker};


//the idea is to hold information about the current level, this includes the chunks that are currently loaded, the structures that are currently loaded and the borders of the level. This is supposed to be used for stuff like pathfinding and collision detection
#[derive(Debug)]
pub struct StratLevel{
    pub(crate) blocking_chunks: Vec<Box<dyn MapChunk>>,
    pub(crate) non_blocking_chunks: Vec<Box<dyn MapChunk>>,
    pub(crate) structures: Vec<Box<dyn VisitableStructure>>,
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

    pub(crate) async fn add_logic_object(&mut self, logic_object: Arc<RwLock<dyn StrategyLogicObject>>){
        logic_object.write().await.set_id(self.logic_objects_id_counter);
        self.logic_objects.push(logic_object);
        self.logic_objects_id_counter += 1;
    }

    pub(crate) async fn initialize(&mut self) -> GameObjects{
        let mut ret : GameObjects = Vec::new();
        let mut rand = rand::thread_rng();

        let start_base = StartObj::new(Position::new(0.0, 0.0), self.logic_objects_id_counter);

        let arxed_base = Arc::new(RwLock::new(start_base));
        self.add_logic_object(arxed_base.clone()).await;
        ret.push(arxed_base.clone());

       for i in 0..6{
            //generate two random values between 0 and 1
            let x = rand.gen_range(0.0.. 1.0);
            let y = rand.gen_range(0.0.. 1.0);
            let worker = Worker::new(Some(arxed_base.clone()), Position::new(i as f32 , y), self.logic_objects_id_counter);
            let arxed = Arc::new(RwLock::new(worker));
            ret.push(arxed.clone());
            self.add_logic_object(arxed).await;
        }
        let worker = Worker::new(Some(arxed_base.clone()), Position::new(0.0, -1.1), self.logic_objects_id_counter);
        let arxed = Arc::new(RwLock::new(worker));
        ret.push(arxed.clone());
        self.add_logic_object(arxed).await; 
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
    fn process_logic(&mut self, delta_time: Duration, blockers: &mut Vec<Box<dyn MapChunk>>, structures: &mut Vec<Box<dyn VisitableStructure>>) -> LogicResult;
    fn set_id(&mut self, id: u64);
    fn get_id(&self) -> u64;
}