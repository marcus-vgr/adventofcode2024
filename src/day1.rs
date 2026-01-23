use crate::utils::{read_lines};
use std::io;
use std::collections::HashMap;


fn getting_input(input: &str) -> io::Result<(Vec<i32>, Vec<i32>)>{

    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    
    let lines = read_lines(input).expect("File not found");
    for line in lines{
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();
        //println!("{}", line);

        let n1: i32 = numbers[0].parse().expect("Problem in column 1");
        let n2: i32 = numbers[1].parse().expect("Problem in column 2");

        v1.push(n1);
        v2.push(n2);
    }

    Ok((v1, v2))

}

fn solution1(input: &str) -> io::Result<()> {
    println!("Running Day1, sol1, {}", input);

    let (mut v1, mut v2) = getting_input(input).expect("Failed to get input vectors");
    v1.sort();
    v2.sort();

    let mut answer: i32 = 0;
    let mut diff: i32;

    for i in 0..=v1.len()-1{
        diff = (v1[i] - v2[i]).abs();
        answer = answer + diff;
    }

    println!("Result: {}", answer);
    
    Ok(())
}

fn solution2(input: &str) -> io::Result<()> {
    println!("Running Day1, sol2, {}", input);

    let (v1, v2) = getting_input(input).expect("Failed to get input vectors");
    
    let mut frequencies: HashMap<i32,i32> = HashMap::new();
    for v in v2{
        *frequencies.entry(v).or_insert(0) += 1;
    }

    let mut answer: i32 = 0;
    for v in v1{
        if let Some(f) = frequencies.get(&v){
            answer += f * v;
        }
    }    

    println!("Result: {}", answer);
    
    Ok(())
}



pub fn run(){
    let _ = solution1("inputs/example1.txt");
    let _ = solution1("inputs/input1.txt");
    let _ = solution2("inputs/example1.txt");
    let _ = solution2("inputs/input1.txt");
}