use std::time::Duration;

use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::{game_objects::game_object::LogicObject, model::results::LogicResult};

#[derive(Debug)]
pub struct LogicTest { 
    pub arr: Vec<Vec<u8>>,
    pub nmbr_of_tries_per_second: u32,
    time_for_next_execute: Duration,
    time_between_executions: Duration,
    length: u32,
    width: u32,
    id: u32,
    result_vec: Vec<u128>,
    exec_count: u64,
    ser_vec: SerializableResultVec,
    current_test_done: bool,
    pos_1: (u32, u32),
    pos_2: (u32, u32),
}

impl LogicObject for LogicTest{
    fn process_logic(&mut self, delta_time: std::time::Duration) -> LogicResult {
        let mut next_execute_ms: i128 = self.time_for_next_execute.as_micros() as i128 - delta_time.as_micros() as i128;
        let mut count = 0;
        let mut opt_res = None;
        while next_execute_ms <= 0{
            count += 1;
            opt_res = Some(self.execute());
            
            next_execute_ms =  self.time_between_executions.as_micros() as i128 + next_execute_ms ;
        }
        self.time_for_next_execute = Duration::from_micros(next_execute_ms as u64);
        if let Some(res) = opt_res{
            return res;
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

#[derive(Debug, Serialize, Deserialize)]
struct SerializableResultVec{
    pub result_vec: Vec<((u32, u32), (u32, u32), f64, f64)>,
    pub trial_count: u64,
}

impl SerializableResultVec{
    fn new(len: usize, wid: usize) -> Self{
        let top_level_vec = Vec::with_capacity(len*wid);

        SerializableResultVec { result_vec: top_level_vec, trial_count: 0 }
    }
}

struct Player{
    current_path: Vec<(u32, u32)>,
    nmbr_of_fields: u32,
    team_nmbr: u8,
    done: bool,
}

impl LogicTest {
    pub fn new(nmbr_of_tries_per_second: u32)-> Self{
        let time_between_executions = Duration::from_secs(1) / nmbr_of_tries_per_second;
        println!("time_between_executions: {:?}", time_between_executions);
        const LEN : usize = 20;
        const WID : usize = 20;
        //if a json file with the results exists, load it
        let path_str = "results".to_owned()+&WID.to_string() +"_"+&LEN.to_string()+".json";
        if std::path::Path::new(&path_str).exists(){
            let file = std::fs::File::open(path_str).unwrap();
            let reader = std::io::BufReader::new(file);
            let ser_vec: SerializableResultVec = serde_json::from_reader(reader).unwrap();
            println!("loaded results from file");
            return Self{
                arr: vec![vec![0; LEN]; WID],
                nmbr_of_tries_per_second,
                time_for_next_execute: Duration::from_millis(0),
                time_between_executions,
                length: LEN as u32,
                width: WID as u32,
                id: 0,
                result_vec: Vec::new(),
                exec_count: 0,
                ser_vec,
                current_test_done: true,
                pos_1: (0, 0),
                pos_2: (0, 0),
            }
        }
        Self{
            arr: vec![vec![0; LEN]; WID],
            nmbr_of_tries_per_second,
            time_for_next_execute: Duration::from_millis(0),
            time_between_executions,
            length: LEN as u32,
            width: WID as u32,
            id: 0,
            result_vec: Vec::new(),
            exec_count: 0,
            ser_vec: SerializableResultVec::new(LEN, WID),
            current_test_done: true,
            pos_1: (0, 0),
            pos_2: (0, 0),
        }
    }

    fn os_to_pos(&self, os: u64) -> (u32, u32){
        let x = os / self.width as u64;
        let y = os % self.width as u64;
        (x as u32, y as u32)
    }


    fn execute(&mut self) -> LogicResult{
        if self.current_test_done{
            self.current_test_done = false;
            let already_executed = self.ser_vec.trial_count;
            let pos_1_os = already_executed / (self.width as u64 * self.length as u64);
            let pos_2_os: u64 = already_executed % (self.width as u64 * self.length as u64);
            let pos_1 = self.os_to_pos(pos_1_os);
            let pos_2 = self.os_to_pos(pos_2_os);
            self.pos_1 = pos_1;
            self.pos_2 = pos_2;
            self.ser_vec.trial_count += 1;
            self.exec_count = 0;

            if self.ser_vec.trial_count >= self.width as u64 * (self.length as u64/2) * self.width as u64 * self.length as u64{ 
                println!("Finished Simulation. Exiting.");
                return LogicResult::DestroyLogicObjects { logic_objects: vec![self.id as u64] };
            }
        }
        let start_pos_1 = self.pos_1;
        let start_pos_2 = self.pos_2;

        //if start positions are the same, skip
        if start_pos_1 == start_pos_2{
            self.current_test_done = true;
            self.ser_vec.result_vec.push((start_pos_1, start_pos_2, 0.0, 0.0));
            return LogicResult::None;
        }
        for i in 0..self.length{
            for j in 0..self.width{
                self.arr[i as usize][j as usize] = 0;
            }
        }
        //create two players, both with an empty path and no fields
        let mut player_1 = Player{
            current_path: Vec::new(),
            nmbr_of_fields: 0,
            team_nmbr: 1,
            done: false,
        };

        let mut player_2 = Player{
            current_path: Vec::new(),
            nmbr_of_fields: 0,
            team_nmbr: 2,
            done: false,
        };

        while self.result_vec.len() < 2{
            self.result_vec.push(0);
        }

        //add the starting position to the path of both players
        player_1.current_path.push(start_pos_1);
        player_2.current_path.push(start_pos_2);

        //add the starting position to the field counter of both players
        player_1.nmbr_of_fields += 1;
        player_2.nmbr_of_fields += 1;
        self.arr[start_pos_1.0 as usize][start_pos_1.1 as usize] = player_1.team_nmbr;
        self.arr[start_pos_2.0 as usize][start_pos_2.1 as usize] = player_2.team_nmbr;

        //while both players are not done
        self.fill_field(&mut player_1, &mut player_2);

        self.result_vec[0] += player_1.nmbr_of_fields as u128;
        self.result_vec[1] += player_2.nmbr_of_fields as u128;
        self.exec_count += 1;
        if self.exec_count % 100000 == 0{
            println!("exec_count: {:?} for pos: {:?} : {:?}", self.exec_count, self.pos_1, self.pos_2);
            println!("Results --- p1: {:?} --- p2: {:?}", self.result_vec[0] as f64 / self.exec_count as f64  , self.result_vec[1] as f64 / self.exec_count as f64 );
        }

        if self.exec_count == 200_000{
            //Serialize to json and write to file
            self.ser_vec.result_vec.push((start_pos_1, start_pos_2, self.result_vec[0] as f64 / self.exec_count as f64, self.result_vec[1] as f64 / self.exec_count as f64));
            let path_str = "results".to_owned()+&self.width.to_string() +"_"+&self.length.to_string()+".json";

            let file = std::fs::File::create(path_str).unwrap();
            let writer = std::io::BufWriter::new(file);
            serde_json::to_writer(writer, &self.ser_vec).unwrap();
            self.current_test_done = true;
            self.result_vec.clear();

        }
        return LogicResult::None;



    }

    fn fill_field(&mut self, player_1: &mut Player, player_2: &mut Player){
        let mut run = true;
        let mut rng = rand::thread_rng();
        while run{

            let mut r1 = true;
            let mut r2 = true;
            if rng.gen_bool(0.5f64)  {
                r1 = self.proc(player_1);
                r2 = self.proc(player_2);
            }else{
                r2 = self.proc(player_2);
                r1 = self.proc(player_1);
            }

            run = r1 || r2;
        }
    }

    fn proc(&mut self, player: &mut Player) -> bool{
        let mut possible_moves = Vec::new();
        while !player.current_path.is_empty(){
            possible_moves = self.possible_moves(player);
            if possible_moves.is_empty(){
                player.current_path.pop();
            }else{
                break;
            }
        }
        if player.current_path.is_empty(){
            player.done = true;
            return false;

        }
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..possible_moves.len());
        let (x, y) = possible_moves[index];
        player.current_path.push((x, y));
        player.nmbr_of_fields += 1;
        self.arr[x as usize][y as usize] = player.team_nmbr;
        return true;
    }



    fn possible_moves(&self, player: &Player) -> Vec<(u32, u32)>{
        let field = player.current_path.last().unwrap();
        let (x, y) = field;

        let mut possible_moves = Vec::new();
        //check all 4 directions, if there is a field and it has value zero, add to possible moves
        if *x > 0 {
            if self.arr[*x as usize - 1][*y as usize] == 0 {
                possible_moves.push((*x - 1, *y));
            }
        }
        if *x < self.length - 1 {
            if self.arr[*x as usize + 1][*y as usize] == 0 {
                possible_moves.push((*x + 1, *y));
            }
        }
        if *y > 0 {
            if self.arr[*x as usize][*y as usize - 1] == 0 {
                possible_moves.push((*x, *y - 1));
            }
        }
        if *y < self.width - 1 {
            if self.arr[*x as usize][*y as usize + 1] == 0 {
                possible_moves.push((*x, *y + 1));
            }
        }


        possible_moves

    }



}