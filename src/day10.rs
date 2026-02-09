use crate::utils::read_lines;
use std::io;

fn getting_input(input: &str) -> io::Result<Vec<Vec<u8>>>{

    let lines = read_lines(input).expect("File not found");
    let mut table: Vec<Vec<u8>> = Vec::new();
    
    for line in lines{
        let line = line?;
        let line: Vec<&str> = line.split("").collect();
        let mut tmp: Vec<u8> = Vec::new();
        for c in line{
            if c.len() < 1{
                continue;
            }
            tmp.push(c.parse::<u8>().expect("Problem in line"));
        }
        table.push(tmp);
    }
    
    Ok(table)

}


fn solve_trailhead(n: u8, row: usize, col: usize, map: &Vec<Vec<u8>>, trail_ends: &mut Vec<(usize, usize)>){

    if n == 9{
        trail_ends.push((row, col));
    }


    else if n < 9{

        if row > 0{
            if map[row-1][col] == n + 1{
                solve_trailhead(n+1, row-1, col, map, trail_ends);    
            }

        }
        if row < map.len()-1{
            if map[row+1][col] == n + 1{
                solve_trailhead(n+1, row+1, col, map, trail_ends);    
            }
        }
        if col > 0{
            if map[row][col-1] == n + 1{
                solve_trailhead(n+1, row, col-1, map, trail_ends);    
            }

        }
        if col < map[0].len()-1{
            if map[row][col+1] == n + 1{
                solve_trailhead(n+1, row, col+1, map, trail_ends);    
            }
        }


    }

}

fn size_unique_vector(vector: Vec<(usize, usize)>) -> usize{

    let mut set: Vec<(usize, usize)> = Vec::new();

    for v in vector{
        if !set.contains(&v){
            set.push(v);
        }
    }

    set.len()

}


fn solution1(input: &str) -> io::Result<()>{
    println!("Running Day10, sol1, {}", input);

    let map: Vec<Vec<u8>> = getting_input(input).expect("Failed to get input");
    let mut answer: u32 = 0;

    for row in 0..map.len(){
        for col in 0..map[0].len(){
            if map[row][col] == 0{
                let mut trail_ends: Vec<(usize, usize)> = Vec::new();
                solve_trailhead(0, row, col, &map, &mut trail_ends);
                //println!("{:?}", trail_ends);
                answer += size_unique_vector(trail_ends) as u32;
            }
        }
    }

    println!("{answer}");

    Ok(())
}

fn solution2(input: &str) -> io::Result<()>{
    println!("Running Day10, sol2, {}", input);

    let map: Vec<Vec<u8>> = getting_input(input).expect("Failed to get input");
    let mut answer: u32 = 0;

    for row in 0..map.len(){
        for col in 0..map[0].len(){
            if map[row][col] == 0{
                let mut trail_ends: Vec<(usize, usize)> = Vec::new();
                solve_trailhead(0, row, col, &map, &mut trail_ends);
                answer += trail_ends.len() as u32;
            }
        }
    }

    println!("{answer}");

    Ok(())
}



pub fn run() {
    let _ = solution1("inputs/example10.txt");
    let _ = solution1("inputs/input10.txt");
    let _ = solution2("inputs/example10.txt");
    let _ = solution2("inputs/input10.txt");
}
