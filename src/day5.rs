use crate::utils::read_lines;
use std::io;
use std::collections::HashMap;

fn getting_input(input1: &str, input2: &str) -> io::Result<(Vec<Vec<i32>>, Vec<Vec<i32>>)>{

    let lines = read_lines(input1).expect("File not found");
    let mut ordering: Vec<Vec<i32>> = Vec::new();
    
    for line in lines{
        let line = line?;
        let numbers: Vec<&str> = line.split("|").collect();
        let tmp: Vec<i32> = vec![
            numbers[0].parse().expect("Problem in line"),
            numbers[1].parse().expect("Problem in line")
        ];
        ordering.push(tmp);
    }
    
    let lines = read_lines(input2).expect("File not found");
    let mut pages: Vec<Vec<i32>> = Vec::new();
    
    for line in lines{
        let line = line?;
        let numbers: Vec<&str> = line.split(",").collect();
        let mut tmp: Vec<i32> = Vec::new();
        for number in numbers{
            tmp.push(number.parse().expect("Problem in line"));
        }
        pages.push(tmp);
    }
    
    
    Ok((ordering, pages))

}


fn solution1(input1: &str, input2: &str) -> io::Result<()>{
    println!("Running Day5, sol1, {} and {}", input1, input2);

    let (ordering, pages) = getting_input(input1, input2).expect("Failed to get input");
    
    let mut rules_before: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut rules_after: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in ordering{
        rules_after.entry(rule[0]).or_insert(Vec::new()).push(rule[1]);
        rules_before.entry(rule[1]).or_insert(Vec::new()).push(rule[0]);
    }

    let mut answer: i32 = 0;

    for update in pages{
        
        let (i,j) = get_indexes_to_change(&update, &rules_after, &rules_before);


        if i == j{
            let middle_number: i32 = update[
                update.len() / 2
            ];
            answer += middle_number;
        }
    }

    println!("Result: {}", answer);
    
    Ok(())
}


fn solution2(input1: &str, input2: &str) -> io::Result<()>{
    println!("Running Day5, sol2, {} and {}", input1, input2);

    let (ordering, pages) = getting_input(input1, input2).expect("Failed to get input");
    
    let mut rules_before: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut rules_after: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in ordering{
        rules_after.entry(rule[0]).or_insert(Vec::new()).push(rule[1]);
        rules_before.entry(rule[1]).or_insert(Vec::new()).push(rule[0]);
    }

    let mut answer: i32 = 0;

    for mut update in pages{
        
        let (mut i, mut j) = get_indexes_to_change(&update, &rules_after, &rules_before);

        if i == j{
            continue;
        }

        while i != j{
            let tmp = update[i];
            update[i] = update[j];
            update[j] = tmp;
            (i,j) = get_indexes_to_change(&update, &rules_after, &rules_before);   
        }

        
        let middle_number: i32 = update[
            update.len() / 2
        ];
        answer += middle_number;

    }

    println!("Result: {}", answer);
    
    Ok(())
}

fn get_indexes_to_change(update: &Vec<i32>, rules_after: &HashMap<i32, Vec<i32>>, rules_before: &HashMap<i32, Vec<i32>>) -> (usize, usize){

    for i in 0..update.len(){
        let to_test: i32 = update[i];
        
        if let Some(after) = rules_after.get(&to_test){
                for j in 0..i{
                    if after.contains(&update[j]){
                        return (i,j);
                    }
                }
        }
        if let Some(before) = rules_before.get(&to_test){
                for j in i+1..update.len(){
                    if before.contains(&update[j]){
                        return (i,j);
                    }
                }
        }
        
    }

    return (0,0);

}


pub fn run() {
    let _ = solution1("inputs/example5_1.txt", "inputs/example5_2.txt");
    let _ = solution1("inputs/input5_1.txt", "inputs/input5_2.txt");
    let _ = solution2("inputs/example5_1.txt", "inputs/example5_2.txt");
    let _ = solution2("inputs/input5_1.txt", "inputs/input5_2.txt");
}
