use crate::utils::read_lines;
use std::io;
use std::collections::VecDeque;
use std::collections::HashMap;

fn getting_input(input: &str) -> io::Result<Vec<String>>{

    let lines = read_lines(input).expect("File not found");
    let mut table: Vec<String> = Vec::new();
    
    for line in lines{
        let line = line?;
        for c in line.split(" "){
            table.push(c.to_string());
        }
    }
    
    Ok(table)

}

fn blink_brute_force(stone: &str) -> Vec<String>{
    let stone = stone.to_string();
    
    let mut stones: Vec<String> = Vec::new();

    if stone == "0".to_string(){
        stones.push("1".to_string());
    }
    else if stone.len() % 2 == 0{
        let s1 = &stone[0..stone.len()/2].parse::<u64>().unwrap();
        let s2 = &stone[stone.len()/2..].parse::<u64>().unwrap();
        stones.push(s1.to_string());
        stones.push(s2.to_string());
    }
    else{
        let s = stone.parse::<u64>().unwrap() * 2024;
        stones.push(s.to_string());
    }

    stones
}

fn solution_brute_force(input: &str, n_blinks: u8) -> io::Result<()>{
    println!("Running Day11, {}, {} blinks", input, n_blinks);

    let stones: Vec<String> = getting_input(input).expect("Failed to get input");
    
    let mut deque: VecDeque<(String, u8)> = VecDeque::new();
    for stone in stones{
        deque.push_back((stone.clone(), n_blinks));
    }
    
    while let Some(stone_tuple) = deque.pop_front(){
        if stone_tuple.1 == 0{
            deque.push_front(stone_tuple);
            break;
        }
        let new_stones = blink_brute_force(&stone_tuple.0);
        for stone in new_stones{
            deque.push_back((stone.clone(), stone_tuple.1-1));
        }
    }

    println!("{}", deque.len());

    Ok(())
}



struct DpTool{
    dp: HashMap<(String,u8), u64>,
}

impl DpTool{
    fn blink_dp(&mut self, stone: &str, depth: u8) -> u64{
        
        let stone = stone.to_string();

        if let Some(v) = self.dp.get(&(stone.clone(),depth)){
            return *v;
        }
        if depth == 0{
            return 1;
        }

         let mut stones: Vec<String> = Vec::new();

        if stone == "0".to_string(){
            stones.push("1".to_string());
        }
        else if stone.len() % 2 == 0{
            let s1 = &stone[0..stone.len()/2].parse::<u64>().unwrap();
            let s2 = &stone[stone.len()/2..].parse::<u64>().unwrap();
            stones.push(s1.to_string());
            stones.push(s2.to_string());
        }
        else{
            let s = stone.parse::<u64>().unwrap() * 2024;
            stones.push(s.to_string());
        }

        let result = if stones.len() == 2 {
        self.blink_dp(&stones[0], depth - 1)
            + self.blink_dp(&stones[1], depth - 1)
        } else {
            self.blink_dp(&stones[0], depth - 1)
        };
        self.dp.insert((stone.clone(), depth), result);

        if let Some(v) = self.dp.get(&(stone.clone(), depth)){
            return *v;
        }
        
        return 0;
    }
}


fn solution_dp(input: &str, n_blinks: u8) -> io::Result<()>{

    println!("Running Day11, {}, {} blinks", input, n_blinks);

    let stones: Vec<String> = getting_input(input).expect("Failed to get input");
    
    let mut solution = DpTool{
        dp: HashMap::new()
    };
    let mut answer: u64 = 0;
    for stone in stones{
        answer += solution.blink_dp(&stone, n_blinks);
    }

    println!("{answer}");

    Ok(())
}


pub fn run() {
    let _ = solution_brute_force("inputs/example11.txt", 25);
    let _ = solution_brute_force("inputs/input11.txt", 25);
    let _ = solution_dp("inputs/input11.txt", 75);
}