use std::{ time::Duration, cell::{RefCell, Ref, RefMut}, rc::{Rc, Weak}, sync::{ Arc}};

use async_std::sync::RwLock;
use rand::Rng;

use crate::{game_objects::{game_object::{DrawableObject, LogicObject}, debug::line::Line, buildings::debug_house::DebugHouse}, model::results::{LogicResult, GameObjects}, controller::{controller::Direction, position::Position}, rendering::sprites::{vertex_configration::VertexConfigration, sprite_mapping::Sprite}};


const DISTANCE_BETWEEN_TILES: f32 = 0.48;
const TIME_BETWEEN_STEPS_IN_MS : u32 = 45;

#[derive(Debug)]
pub(crate) struct Maze{
    pub(crate) width: usize,
    pub(crate) height: usize,

    start_tile: Weak<RefCell<MazeTile>>,
    end_tile: Weak<RefCell<MazeTile>>,
    next_tile_ms: u32,
    maze: Vec<Vec<Rc<RefCell<MazeTile>>>>,
    current_path: Option<Vec<Weak<RefCell<MazeTile>>>>,
    id: u32,
}


impl LogicObject for Maze{
    fn process_logic(&mut self, delta_time: Duration) -> LogicResult{
        let millis = delta_time.as_millis() as u32;
        if self.next_tile_ms > millis{
            self.next_tile_ms -= millis;
            return LogicResult::None;
        }
        let overtime = millis - self.next_tile_ms;
        if overtime > TIME_BETWEEN_STEPS_IN_MS{
            self.next_tile_ms = 0;
        }else{
            self.next_tile_ms = TIME_BETWEEN_STEPS_IN_MS - (overtime);
        }


        //if we have no path, we need to find one
        let ret = self.find_path_step();
        ret
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}

impl Maze{

    pub fn new(width: usize, height: usize, position: (f32, f32))-> (Self, GameObjects){
        //if any of these values is 0, panic!
        assert!(width > 0 && height > 0);
        let mut maze: Vec<Vec<Rc<RefCell<MazeTile>>>> = Vec::with_capacity(width);
        


        //set the correct positions for the MazeTiles
        for i in 0..width{
            let column = Vec::with_capacity(height);
            maze.push(column);
            for j in 0..height{
                let tile = MazeTile{position_offset: position, position: (i, j),connected:(true,true,true,true),visited:false, underlying_objects: [None, None, None, None]};
                let refr = Rc::new(RefCell::new(tile));
                maze[i].push(refr);
            }
        }


        let start_tile= Rc::downgrade(&maze[0][0].clone());
        let end_tile= Rc::downgrade(&maze[width-1][height-1].clone());
        let mut maze = Maze { 
            id: 0,
            maze: maze,
            width: width,
            height: height,
            current_path: Some(Vec::new()),
            next_tile_ms: 2000,
            start_tile: start_tile,
            end_tile: end_tile,            
         };
       
        maze.set_outside_walls();
        
        //set left of 0,0 and right of width-1, height-1 to true, these are the start and end points
        RefCell::borrow_mut(&maze.maze[0][0]).connected.3 = true;

        RefCell::borrow_mut(&maze.maze[width-1][height-1]).connected.1 = true;

        let objects = maze.generate_maze_objects(); 

        (maze, objects)
    }

    fn generate_maze_objects(&self) -> GameObjects{
        let mut objects = GameObjects::new();
        for i in 0..self.width{
            for j in 0..self.height{
                let tile = self.maze[i][j].clone();
                let mut tile = tile.borrow_mut();
                let tile_objects = tile.update_underlying_objects();

                objects.extend(tile_objects.0);
            }
        }

        objects
    }       
   
   
    //set the outsides of the maze to false
    fn set_outside_walls(&mut self){
        for j in 0..self.height{
            RefCell::borrow_mut(&self.maze[0][j]).connected.3 = false;
            self.maze[self.width-1][j].borrow_mut().connected.1 = false;
        }
        for i in 0..self.width{
            self.maze[i][0].borrow_mut().connected.2 = false;
            self.maze[i][self.height-1].borrow_mut().connected.0 = false;
        }
    }



    fn find_path_step(&mut self) -> LogicResult{
        if let Some(path) = &mut self.current_path{
            if path.len() == 0{
                let rc = self.maze[0][0].clone();
                let weak = Rc::downgrade(&rc);
                path.push(weak.clone());
                let possible_neighbors = self.visit_tile(&weak);
                let (to_add, to_remove) = weak.upgrade().unwrap().borrow_mut().update_underlying_objects_with_prev_ref(possible_neighbors);
                return LogicResult::CreateAndDestroyGameObjects { game_objects_to_create: to_add, game_objects_to_destroy: to_remove };
            }
            else{
                self.extend_path();
                if let Some(path) = self.current_path.clone().as_ref(){
                    if let Some(tile) = path.last(){
                        let possible_neighbors =  self.visit_tile(tile);
                        let (to_add, to_remove) = tile.upgrade().unwrap().borrow_mut().update_underlying_objects_with_prev_ref(possible_neighbors);
                        return LogicResult::CreateAndDestroyGameObjects { game_objects_to_create: to_add, game_objects_to_destroy: to_remove };
                    }else{
                        unreachable!("Path is empty, but current_path is not");
                    }
                }else{
                    let mut return_vec: Vec<u64> = Vec::with_capacity(1);
                    return_vec.push(self.id as u64);
                    return LogicResult::DestroyLogicObjects { logic_objects: return_vec };
                }
            }

        }
        unreachable!("None Path in find_path_step! This should never happen! The maze should have been destroyed!");

    }


    fn extend_path(&mut self){
        let path = self.current_path.as_ref().unwrap();
        let mut point ;
        let mut i = path.len()-1;
        let mut next_tile: Weak<RefCell<MazeTile>> = Weak::new();
        loop{
            
            point = path.get(i).unwrap();
            let upgrade = point.upgrade().unwrap();
            let tile = upgrade.borrow_mut();
            let neighbors = self.check_directions(&tile);
            let mut possible_directions = Vec::new();
            for j in 0..4{
                if let Some(actual_tile) = &neighbors[j]{
                    let tile = actual_tile.upgrade().unwrap();
                    if !tile.borrow().visited{
                        possible_directions.push(actual_tile);
                    }
                }
            }
            if possible_directions.len() == 0{
                if i == 0{
                    self.current_path = None;

                    return;
                }
                i = i -1;
                continue;
            }

            next_tile = possible_directions[rand::thread_rng().gen_range(0..possible_directions.len())].clone();
            break;

        }

        let to_pop = path.len() - i - 1;
        for _ in 0..to_pop{
            self.current_path.as_mut().unwrap().pop();
        }

        self.current_path.as_mut().unwrap().push(next_tile);
        
    }

    //the new tile is the end point of our current path, which means we can set everything to false, except for where we just came from and the end/start point
    fn visit_tile(&mut self, tile_weak: &Weak<RefCell<MazeTile>>) -> [Option<Weak<RefCell<MazeTile>>>; 4]{


        let upgrade = tile_weak.upgrade().unwrap();
        let mut tile = upgrade.borrow_mut();
        tile.visited = true;
        let possible_directions : [Option<Weak<RefCell<MazeTile>>>; 4] = self.check_directions(&tile);
        self.set_connection(&mut tile, &possible_directions);

        if tile_weak.ptr_eq(&self.start_tile){
            tile.connected.3 = true;   
        }
        if tile_weak.ptr_eq(&self.end_tile){
            tile.connected.1 = true;   
        }
        return possible_directions;
    }

    fn set_connection(&self, tile: &mut RefMut<MazeTile>, directions: &[Option<Weak<RefCell<MazeTile>>>; 4]){

        for dir in 0..4{
            if let Some(neighbor) = &directions[dir]{
                let upgrade = neighbor.upgrade().unwrap();
                let pos = upgrade.borrow().position;
            }
        }
        let mut previous_tile_position = None;
        let path = self.current_path.as_ref().unwrap();
        let len = path.len();
        let mut previous_tile = None;
        if len > 1{
            let previous_id = len -2;
            previous_tile = path.get(previous_id);
            previous_tile_position = Some(previous_tile.unwrap().upgrade().unwrap().borrow().position);
        }
        

        tile.connected = (false, false, false, false);

        if let Some(prev) = previous_tile_position{
            if !(tile.position.0 == self.width) && prev.0 == tile.position.0 + 1{
                tile.connected.1 = true;
            }
            if !(tile.position.0 == 0) && prev.0 == tile.position.0 - 1{
                tile.connected.3 = true;
            }
            if !(tile.position.1 == self.height) && prev.1 == tile.position.1 + 1{
                tile.connected.0 = true;
            }
            if !(tile.position.1 == 0) && prev.1 == tile.position.1 - 1{
                tile.connected.2 = true;
            }

        }
        
    }


    fn check_directions(&self, tile: &RefMut<MazeTile>) ->  [Option<Weak<RefCell<MazeTile>>>; 4]{
        let up = self.check_direction(tile, 0);
        let right = self.check_direction(tile, 1);
        let down = self.check_direction(tile, 2);
        let left = self.check_direction(tile, 3);
        [up, right, down, left]
    }

    fn check_direction(&self, tile: &RefMut<MazeTile>, direction: usize) -> Option<Weak<RefCell<MazeTile>>>{
        if direction > 3 {
            panic!("Direction must be between 0 and 3");
        }
        
        let (x, y) = tile.position;
        if direction == 0 && x == (self.maze.len() ){
            return None;
        }
        if direction == 1 && y == (self.maze[0].len() ){
            return None;
        }
        if direction == 3 && x == 0{
            return None;
        }
        if direction == 2 && y == 0{
            return None;
        }
        let (updated_x, updated_y) = match direction{
            0 => (x, y+1),
            1 => (x+1, y),
            2 => (x, y-1),
            3 => (x-1, y),
            _ => panic!("Direction must be between 0 and 3"),
        };
        let row_check = self.maze.get(updated_x);
        if let Some(row) = row_check{
            let column_check = row.get(updated_y);
            if let Some(tile) = column_check{
                let weak = Rc::downgrade(&tile);
                return Some(weak);
            }
        }
        None

    }




}
fn show_pos_string(s: Vec<Vec<Rc<RefCell<MazeTile>>>>) -> String{
    let mut string = String::new();
    for row in s.iter(){
        for tile in row.iter(){
            string.push_str(&format!("{:?} ", tile.borrow().position));
        }
        string.push_str("\n");
    }
    string
}


#[derive(Debug, Clone)]
struct MazeTile{
    position_offset: (f32, f32),
    position: (usize, usize),
    connected: (bool, bool, bool, bool),
    visited: bool,
    underlying_objects: [Option<Arc<RwLock<dyn DrawableObject + Send + Sync>>>; 4],
}

impl MazeTile{
    fn update_underlying_objects(&mut self) -> (GameObjects, Vec<u64>){
        let mut to_add: Vec<Arc<RwLock<dyn DrawableObject + Send + Sync>>> = Vec::new();
        let mut to_remove: Vec<u64> = Vec::new();
        //process top
        let current_object = self.underlying_objects[0].clone();
        let actual_x = self.position.0 as f32 * DISTANCE_BETWEEN_TILES + self.position_offset.0;
        let actual_y = self.position.1 as f32 * DISTANCE_BETWEEN_TILES + self.position_offset.1;

        if let Some(object) = current_object{
            if self.connected.0{
                let obj_id = object.try_read().unwrap().get_id();
                to_remove.push(obj_id);
                self.underlying_objects[0] = None;
            }
        }else{
            if !self.connected.0{
                let object = Arc::new(RwLock::new(Line::Horizontal { position: Position::new(actual_x, actual_y+ DISTANCE_BETWEEN_TILES /2.0 ), id: 0 }));
                //let object = Arc::new(RwLock::new(DebugHouse { position: Position::new(self.position.0 as f32, self.position.1 as f32), texture: Sprite::DwarfBaseHouse, vertices: VertexConfigration::SQUARE_SMALL_1 }));
                to_add.push(object.clone());
                self.underlying_objects[0] = Some(object);
            }
        }


        let current_object = self.underlying_objects[1].clone();
       //process right
        if let Some(object) = current_object{
            if self.connected.1{
                let obj_id = object.try_read().unwrap().get_id();
                to_remove.push(obj_id);                self.underlying_objects[1] = None;
            }
        }else{
            if !self.connected.1{
                let object = Arc::new(RwLock::new(Line::Vertical { position: Position::new(actual_x + DISTANCE_BETWEEN_TILES/2.0, actual_y), id: 0 }));
                to_add.push(object.clone());
                self.underlying_objects[1] = Some(object);
            }
        }
        let current_object = self.underlying_objects[3].clone();
        //process bottom
        if let Some(object) = current_object{
            if self.connected.2{
                let obj_id = object.try_read().unwrap().get_id();
                to_remove.push(obj_id);                self.underlying_objects[2] = None;
            }
        }else{
            if !self.connected.2{
                let object = Arc::new(RwLock::new(Line::Horizontal { position: Position::new(actual_x, actual_y - DISTANCE_BETWEEN_TILES/2.0), id: 0 }));
                to_add.push(object.clone());
                self.underlying_objects[2] = Some(object);
            }
        }
        //process left
        let current_object = self.underlying_objects[3].clone();
        if let Some(object) = current_object{
            if self.connected.3{
                let obj_id = object.try_read().unwrap().get_id();
                to_remove.push(obj_id);                self.underlying_objects[3] = None;
            }
        }else{
            if !self.connected.3{
                let object = Arc::new(RwLock::new(Line::Vertical { position: Position::new(actual_x - DISTANCE_BETWEEN_TILES/2.0, actual_y), id: 0 }));
                to_add.push(object.clone());
                self.underlying_objects[3] = Some(object);
            }
        }
        
        (to_add, to_remove)
    }


    fn update_underlying_objects_with_prev_ref(&mut self, neighbours : [Option<Weak<RefCell<MazeTile>>>;4]) -> (GameObjects, Vec<u64>){
        let mut to_add: Vec<Arc<RwLock<dyn DrawableObject + Send + Sync>>> = Vec::new();
        let mut to_remove: Vec<u64> = Vec::new();

        //process top
        let current_object = self.underlying_objects[0].clone();
        let actual_x = self.position.0 as f32 * DISTANCE_BETWEEN_TILES + self.position_offset.0;
        let actual_y = self.position.1 as f32 * DISTANCE_BETWEEN_TILES + self.position_offset.1;

        
        let background_square = DebugHouse::new(Sprite::Green, Position { x: actual_x, y: actual_y}, VertexConfigration::NEARLY_SQUARE_RECTANGLE_0);
        to_add.push(Arc::new(RwLock::new(background_square)));
        if let Some(object) = current_object{

        }else{
            if let Some(neighbor) = neighbours[0].as_ref(){
                let upg = neighbor.upgrade().unwrap();
                if !self.connected.0 && !upg.borrow().visited{
                let object = Arc::new(RwLock::new(Line::Horizontal { position: Position::new(actual_x, actual_y+ DISTANCE_BETWEEN_TILES /2.0 ), id: 0 }));
                //let object = Arc::new(RwLock::new(DebugHouse { position: Position::new(self.position.0 as f32, self.position.1 as f32), texture: Sprite::DwarfBaseHouse, vertices: VertexConfigration::SQUARE_SMALL_1 }));
                to_add.push(object.clone());
                self.underlying_objects[0] = Some(object);
            }
            }

        }


        let current_object = self.underlying_objects[1].clone();
       //process right
        if let Some(object) = current_object{

        }else{
            if let Some(neighbor) = neighbours[1].as_ref(){
            let upg = neighbor.upgrade().unwrap();
                if !self.connected.1 && !upg.borrow().visited{
                    let object = Arc::new(RwLock::new(Line::Vertical { position: Position::new(actual_x + DISTANCE_BETWEEN_TILES/2.0, actual_y), id: 0 }));
                    to_add.push(object.clone());
                    self.underlying_objects[1] = Some(object);
                }
            }
        }
        let current_object = self.underlying_objects[2].clone();
        //process bottom
        if let Some(object) = current_object{

        }else{
            if let Some(neighbor) = neighbours[2].as_ref(){
            let upg = neighbor.upgrade().unwrap();
                if !self.connected.2 && !upg.borrow().visited{
                    
                    let object = Arc::new(RwLock::new(Line::Horizontal { position: Position::new(actual_x, actual_y - DISTANCE_BETWEEN_TILES/2.0), id: 0 }));
                    to_add.push(object.clone());
                    self.underlying_objects[2] = Some(object);
                }
        }
        }
        //process left
        let current_object = self.underlying_objects[3].clone();
        if let Some(object) = current_object{

        }else{
            if let Some(neighbor) = neighbours[3].as_ref(){
            let upg = neighbor.upgrade().unwrap();
            if !self.connected.3 && !upg.borrow().visited{
                    let object = Arc::new(RwLock::new(Line::Vertical { position: Position::new(actual_x - DISTANCE_BETWEEN_TILES/2.0, actual_y), id: 0 }));
                    to_add.push(object.clone());
                    self.underlying_objects[3] = Some(object);
                }
            }
        }
        if self.connected.0 {
            if let Some(tile) = neighbours[0].as_ref(){
                let tile = tile.upgrade().unwrap();
                let tile = tile.try_borrow().unwrap();
                let obj_id = tile.get_side_object_id(2);
                if let Some(obj_id) = obj_id{
                    to_remove.push(obj_id);
                }
            }
        }
        if self.connected.1 {
            if let Some(tile) = neighbours[1].as_ref(){
                let tile = tile.upgrade().unwrap();
                let tile = tile.try_borrow().unwrap();
                let obj_id = tile.get_side_object_id(3);
                if let Some(obj_id) = obj_id{
                    to_remove.push(obj_id);

                }
            }
        }
        if self.connected.2 {
            if let Some(tile) = neighbours[2].as_ref(){
                let tile = tile.upgrade().unwrap();
                let tile = tile.try_borrow().unwrap();
                let obj_id = tile.get_side_object_id(0);
                if let Some(obj_id) = obj_id{
                    to_remove.push(obj_id);

                }
            }
        }
        if self.connected.3 {
            if let Some(tile) = neighbours[3].as_ref(){
                let tile = tile.upgrade().unwrap();
                let tile = tile.try_borrow().unwrap();
                let obj_id = tile.get_side_object_id(1);
                if let Some(obj_id) = obj_id{
                    to_remove.push(obj_id);

                }
            }
        }


        

        (to_add, to_remove)
    }


    fn get_side_object_id(&self, direction: usize) -> Option<u64>{
        let dum = self.underlying_objects[direction].as_ref();
        if dum.is_none(){
            return None;
        }
        let dum = dum.unwrap();
        loop {
            if let Some(obj) = dum.try_read(){
                return Some(obj.get_id());
            }
        }
    }
}