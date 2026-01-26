use crate::utils::{read_lines};
use std::io;

fn getting_input(input: &str) -> io::Result<Vec<Vec<i32>>>{

    let mut v: Vec<Vec<i32>> = Vec::new();
    
    let lines = read_lines(input).expect("File not found");
    for line in lines{
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();
        let mut tmp: Vec<i32> = vec![];
        for number in numbers{
            tmp.push(
                number.parse().expect("Problem in line")
            );
        }
        v.push(tmp);
    }

    Ok(v)

}

fn is_vector_safe(v: &Vec<i32>) -> i32{

    let decreasing: bool = v[1] < v[0];
    let mut failed_index: i32 = -1;    
    for i in 1..=v.len()-1{
        if (v[i] - v[i-1]).abs() > 3 || v[i] == v[i-1] {
            failed_index = i as i32 ;
            break;
        }
        else if v[i] > v[i-1]{
            if decreasing{
                failed_index = i as i32 ;
                break;
            }
        }
        else if v[i] < v[i-1]{
            if !decreasing{
                failed_index = i as i32 ;
                break;
            }
            
        }  
    }

    failed_index
}

fn solution1(input: &str) -> io::Result<()>{
    println!("Running Day2, sol1, {}", input);

    let vinputs = getting_input(input).expect("Failed to get input vectors");

    let mut answer: i32 = 0;
    for v in vinputs{
        if v.len() < 2{
            continue;
        }

        if is_vector_safe(&v) == -1{
            answer += 1;
        }

    }

    println!("Result: {}", answer);

    Ok(())
}

fn solution2(input: &str) -> io::Result<()>{
    println!("Running Day2, sol2, {}", input);

    let vinputs = getting_input(input).expect("Failed to get input vectors");

    let mut answer: i32 = 0;
    for v in vinputs{
        if v.len() < 2{
            continue;
        }

        let failed_index = is_vector_safe(&v);
        if failed_index == -1{
            answer += 1;
        }
        else{
            for i in 0..=failed_index{
                let mut trial = v.clone();
                trial.remove(i as usize);
                if is_vector_safe(&trial) == -1{
                    answer += 1;
                    break;
                }    
            }
        }
        
    }

    println!("Result: {}", answer);

    Ok(())
}




pub fn run(){
    let _ = solution1("inputs/example2.txt");
    let _ = solution1("inputs/input2.txt");
    let _ = solution2("inputs/example2.txt");
    let _ = solution2("inputs/input2.txt");
}