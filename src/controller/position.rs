use std::{sync::{Arc}, collections::HashMap};


use tokio::sync::RwLock;

use crate::{model::strategy_test::map_chunk::MapChunk, game_objects::game_object::VisitableStructure};
#[derive(Debug, Clone, Copy)]

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct PathTrack{
    pub path: Vec<(f32, f32)>,
    pub previously_visited: Vec<(f32, f32)>,
}

struct PathBuildingInformation{
    optimal_path_length: f32,
    optimal_path: Vec<(f32, f32)>,
}

struct PathHelper{
    map: HashMap<(f32, f32), PathBuildingInformation>,
    current_best: Option<(f32, f32)>,
}
pub struct IntPosition{
    pub x: i32,
    pub y: i32,
}

pub fn direct_distance_between_positions(start: &Position, end: &Position) -> f32{
    let x = start.x - end.x;
    let y = start.y - end.y;
    (x * x + y * y).sqrt()
}

pub fn direct_distance_between(start: &(f32, f32), end: &(f32, f32)) -> f32{
    let x = start.0 - end.0;
    let y = start.1 - end.1;
    (x * x + y * y).sqrt()
}

pub fn find_all_eligible_neighbors(path_track: &PathTrack, blockers: &Vec<Box<dyn MapChunk>>, structures: &Vec<Arc<RwLock<dyn VisitableStructure>>>) -> Vec<(f32, f32)>{
    let mut end_point = path_track.path[path_track.path.len()-1];
    end_point = (end_point.0.floor(), end_point.1.floor());
    let mut ret = find_all_neighbors(&end_point);
    ret.retain(|x| 
        is_eligible(x, blockers, structures, path_track));
    ret
}

fn is_eligible(tile: &(f32, f32), blockers: &Vec<Box<dyn MapChunk>>, structures: &Vec<Arc<RwLock<dyn VisitableStructure>>>, path: &PathTrack) -> bool{
    if path.path.contains(tile){
        return false;
    }
    for blocker in blockers{
        if blocker.inf().contains(&Position::new(tile.0, tile.1)){
            return false;
        }
    }
    for structure in structures{
        if structure.blocking_read().get_blocking_chunk().contains(&Position::new(tile.0, tile.1)){
            return false;
        }
    }

    true
}

pub fn find_all_neighbors(tile: &(f32, f32)) -> Vec<(f32, f32)>{
    let mut ret = Vec::new();
    ret.push(((tile.0 + 1.0).floor(), tile.1.floor()));
    ret.push(((tile.0 + 1.0).floor(), (tile.1 + 1.0).floor()));
    ret.push(((tile.0 + 1.0).floor(), (tile.1 - 1.0).floor()));
    ret.push(((tile.0 - 1.0).floor(), (tile.1 + 1.0).floor()));
    ret.push(((tile.0 - 1.0).floor(), tile.1.floor()));
    ret.push(((tile.0 - 1.0).floor(), (tile.1 - 1.0).floor()));
    ret.push((tile.0.floor(), (tile.1 + 1.0).floor()));
    ret.push((tile.0.floor(), (tile.1 - 1.0).floor()));
    ret
}



fn find_smallest_dist(start: &(f32, f32), neighbors: &Vec<(f32, f32)>) -> (f32, f32){
    let mut ret = neighbors[0];
    let mut smallest_dist = direct_distance_between(start, &ret);
    for neighbor in neighbors{
        let dist = direct_distance_between(start, neighbor);
        if dist < smallest_dist{
            smallest_dist = dist;
            ret = *neighbor;
        }
    }
    ret
}

#[allow(unused)]
impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position {
            x: x,
            y: y,
        }
    }

    pub fn distance_to_position(&self, other: &Position) -> f32 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        (x * x + y * y).sqrt()
    }

    pub fn direction_to(&self, other: &(f32, f32)) -> (f32, f32) {
        let x = other.0 - self.x;
        let y = other.1 - self.y;
        let dist = (x * x + y * y).sqrt();
        (x / dist, y / dist)
    }

    pub fn distance(&self, other: &(f32, f32)) -> f32{
        let x = self.x - other.0;
        let y = self.y - other.1;
        (x * x + y * y).sqrt()
    }
    pub fn find_path_to(&self, other: &(f32, f32), blockers: &Vec<Box<dyn MapChunk>>, structures: &Vec<Arc<RwLock<dyn VisitableStructure>>>) -> Option<Vec<(f32, f32)>>{
        let mut path_track = PathTrack{
            path: Vec::new(),
            previously_visited: Vec::new(),
        };
        path_track.path.push(*other);
        path_track.path.push((other.0.floor(), other.1.floor()));
        let mut ret = None;
        loop  {
            if path_track.path.len() == 0{
                break;
                
            }
            if self.x.floor() == path_track.path.last().unwrap().0.floor() && self.y.floor() == path_track.path.last().unwrap().1.floor(){
                path_track.path.reverse();
                ret = Some(path_track.path);
                break;

            }
            
            let eligible_neighbors = find_all_eligible_neighbors(&path_track, blockers, structures);
            if eligible_neighbors.len() == 0{
                path_track.previously_visited.push(path_track.path.pop().unwrap());
                continue;
            }
            let next_step = find_smallest_dist(&(self.x, self.y), &eligible_neighbors);
            path_track.path.push(next_step);

        }

        ret
                
    }

    pub fn find_optimal_path_to(&self, other: &(f32, f32), blockers: &Vec<Box<dyn MapChunk>>, structures: &Vec<Arc<RwLock<dyn VisitableStructure>>>) -> Option<Vec<(f32, f32)>>{
        let mut helper: PathHelper = PathHelper{
            map: HashMap::new(),
            current_best: None,
        };
                
    }

    pub fn find_path_to_position(&self, other: &Position, blockers: &Vec<Box<dyn MapChunk>>, structures: &Vec<Arc<RwLock<dyn VisitableStructure>>>) -> Option<Vec<(f32, f32)>>{
        self.find_path_to(&(other.x, other.y), blockers, structures)
    }
    pub fn get_x_y_values(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn get_x(&self) -> f32 {
       self.x
    }

    pub fn set_x(&mut self, value: f32) {
        self.x = value;
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn set_y(&mut self, value: f32) {
        self.y = value;
    }
}


struct Line{
    start: Position,
    end: Position,
}

impl Line{
    fn new(start: Position, end: Position) -> Self{
        Self{
            start,
            end,
        }
    }

    fn get_length(&self) -> f32{
        self.start.distance_to_position(&self.end)
    }

    fn get_angle(&self) -> f32{
        let x = self.end.x - self.start.x;
        let y = self.end.y - self.start.y;
        y.atan2(x)
    }

    fn get_x_y_values(&self) -> ((f32, f32), (f32, f32)){
        ((self.start.x, self.start.y), (self.end.x, self.end.y))
    }


}