use crate::utils::read_lines;
use std::io;
use itertools::Itertools;

fn getting_input(input: &str) -> io::Result<Vec<(i64, Vec<i64>)>>{

    let lines = read_lines(input).expect("File not found");
    let mut equations: Vec<(i64, Vec<i64>)> = Vec::new();
    
    for line in lines{
        let line = line?;
        let columns: Vec<&str> = line.split(": ").collect();
        let result: i64 = columns[0].parse::<i64>().expect("Problem in line");
        let numbers: Vec<&str> = columns[1].split(" ").collect();
        let mut tmp: Vec<i64> = Vec::new();
        for number in numbers{
            tmp.push(
                number.parse::<i64>().expect("Problem in line")
            );
        }
        equations.push((result, tmp));
    }
    
    Ok(equations)

}

fn create_operation_combinations(size: usize, operations: Vec<char>) -> Vec<Vec<char>>{

    let combinations: Vec<Vec<char>> = 
        std::iter::repeat(operations)
            .take(size)
            .multi_cartesian_product().
            collect();

    combinations
}

fn is_valid_equation_sol1(result: &i64, numbers: &Vec<i64>) -> bool{

    let n_numbers = numbers.len();
    let combinations = create_operation_combinations(n_numbers - 1, vec!['+', '*']);
    
    for operations in combinations{
        let mut candidate: i64 = numbers[0];
    
        for i in 1..n_numbers{
            if operations[i-1] == '+'{
                candidate += numbers[i];
            }
            else if operations[i-1] == '*'{
                candidate *= numbers[i];
            }
            if candidate > *result{
                break;
            }
        }
        if candidate == *result{
            //println!("{}, {:?}, {:?}", result, numbers, operations);
            return true;
        }    
    }
    
    return false;
}

fn is_valid_equation_sol2(result: &i64, numbers: &Vec<i64>) -> bool{

    let n_numbers = numbers.len();
    let combinations = create_operation_combinations(n_numbers - 1, vec!['+', '*', '|']);
    
    for operations in combinations{
        let mut candidate: i64 = numbers[0];
    
        for i in 1..n_numbers{
            if operations[i-1] == '+'{
                candidate += numbers[i];
            }
            else if operations[i-1] == '*'{
                candidate *= numbers[i];
            }
            else if operations[i-1] == '|'{
                candidate = format!("{}{}", candidate, numbers[i]).parse::<i64>().unwrap()
            }
            if candidate > *result{
                break;
            }
        }
        if candidate == *result{
            //println!("{}, {:?}, {:?}", result, numbers, operations);
            return true;
        }    
    }
    
    return false;
}



fn solution1(input: &str) -> io::Result<()>{
    println!("Running Day7, sol1, {}", input);

    let equations = getting_input(input).expect("Failed getting input");
    let mut answer: i64 = 0;
   
    for (result, numbers) in equations.into_iter(){
        if is_valid_equation_sol1(&result, &numbers){
            answer += result;
        }
    }
   
    println!("{}", answer);

    Ok(())
}


fn solution2(input: &str) -> io::Result<()>{
    println!("Running Day7, sol2, {}", input);

    let equations = getting_input(input).expect("Failed getting input");
    let mut answer: i64 = 0;
   
    for (result, numbers) in equations.into_iter(){
        if is_valid_equation_sol2(&result, &numbers){
            answer += result;
        }
    }
   
    println!("{}", answer);

    Ok(())
}



pub fn run() {
    let _ = solution1("inputs/example7.txt");
    let _ = solution1("inputs/input7.txt");
    let _ = solution2("inputs/example7.txt");
    let _ = solution2("inputs/input7.txt");
}
