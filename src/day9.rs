use crate::utils::read_lines;
use std::io;

fn getting_input(input: &str) -> io::Result<Vec<u8>>{

    let lines = read_lines(input).expect("File not found");
    let mut table: Vec<u8> = Vec::new();
    
    for line in lines{
        let line = line?;
        let line: Vec<&str> = line.split("").collect();
        for c in line{
            if c.len() < 1{
                continue;
            }
            table.push(c.parse::<u8>().expect("Problem in line"));
        }
    }
    
    Ok(table)

}


fn solution1(input: &str) -> io::Result<()>{
    println!("Running Day9, sol1, {}", input);

    let disk_map: Vec<u8> = getting_input(input).expect("Failed to get input");
    let mut blocks: Vec<String> = Vec::new();
    let empty_space: String = String::from(".");
    
    
    let mut id: u32 = 0;
    for i in 0..disk_map.len(){
        let item: String;
        if i % 2 == 0{
            item = id.to_string();
            id += 1;
        }
        else{
            item = empty_space.clone();
        }
        for _ in 0..disk_map[i]{
            blocks.push(item.clone());    
        }
        
    }
    //println!("{}", blocks.join(""));


    let mut begin: usize = 0;
    let mut end: usize = blocks.len() - 1;

    while begin < end-2{
        while blocks[begin] != empty_space{
            begin += 1;
        }
        while blocks[end] == empty_space{
            end -= 1;
        }
        blocks[begin] = blocks[end].clone();
        blocks[end] = empty_space.clone();
        
    }
    //println!("{}", blocks.join(""));
    
    let mut answer: u64 = 0;
    for i in 0..blocks.len(){
        if blocks[i] == empty_space{
            break;
        }
        let n: u64 = blocks[i].parse::<u64>().unwrap();
        let i = i as u64;
        answer += i * n;  
    }

    println!("{}", answer);

    Ok(())
}


fn solution2(input: &str) -> io::Result<()>{
    println!("Running Day9, sol2, {}", input);

    let disk_map: Vec<u8> = getting_input(input).expect("Failed to get input");
    let mut blocks: Vec<String> = Vec::new();
    let empty_space: String = String::from(".");
    
    
    let mut id: u32 = 0;
    for i in 0..disk_map.len(){
        let item: String;
        if i % 2 == 0{
            item = id.to_string();
            id += 1;
        }
        else{
            item = empty_space.clone();
        }
        for _ in 0..disk_map[i]{
            blocks.push(item.clone());    
        }
        
    }
    //println!("{}", blocks.join(""));


    let mut end: usize = blocks.len() - 1;
    let mut begin: usize = end-0;

    while begin > 0{
        while blocks[end] == empty_space{
            end -= 1;
            begin -= 1;
            if begin == 0{
                break;
            }
        }
        while blocks[begin] == blocks[end]{
            begin -= 1;
            if begin == 0{
                break;
            }
        }
    
        let size: usize = end - begin;

        let mut size_free: usize = 0;
        for i in 0..=begin{
            if blocks[i] == empty_space{
                size_free += 1;
            }
            else{
                size_free = 0;
            }
            if size_free == size{
                for j in 0..size_free{
                    blocks[i-size_free+j+1] = blocks[end-j].clone();
                    blocks[end-j] = empty_space.clone();
                }
            }
        }
        end = begin;
        //println!("{}", blocks.join(""));
    
    }
    
    



    let mut answer: u64 = 0;
    for i in 0..blocks.len(){
        if blocks[i] == empty_space{
            continue;
        }
        let n: u64 = blocks[i].parse::<u64>().unwrap();
        let i = i as u64;
        answer += i * n;  
    }

    println!("{}", answer);

    Ok(())
}


pub fn run() {
    let _ = solution1("inputs/example9.txt");
    let _ = solution1("inputs/input9.txt");
    let _ = solution2("inputs/example9.txt");
    let _ = solution2("inputs/input9.txt");
}
