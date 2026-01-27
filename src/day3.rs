use regex::Regex;
use crate::utils::read_lines;
use std::io;
use std::cmp;


fn getting_input(input: &str) -> io::Result<String>{

    let lines = read_lines(input).expect("File not found");
    let mut command = String::from("");
    for line in lines{
        let line = line?;
        command += &line;
    }

    Ok(command)

}


fn solution1(input: &str) -> io::Result<()>{
    println!("Running Day3, sol1, {}", input);

    let command: String = getting_input(input).expect("Failed to get input");
    
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut answer: u32 = 0;
    for cap in re.captures_iter(&command) {
        let x: u32 = cap[1].parse().unwrap();
        let y: u32 = cap[2].parse().unwrap();

        answer += x * y;
        //println!("Found mul({}, {})", x, y);
    }

    println!("Result: {}", answer);
    
    Ok(())
}

fn solution2(input: &str) -> io::Result<()>{

    println!("Running Day3, sol1, {}", input);

    let command: String = getting_input(input).expect("Failed to get input");
    let size_command: usize = command.len();

    let dont_string: &str = "don't()";
    let size_dont_string: usize = dont_string.len();
    let do_string: &str = "do()";
    let size_do_string: usize = do_string.len();
    let max_mul_command: usize = "mul(XXX,YYY)".len();
    let mut answer: u32 = 0;


    let mut i: usize = 0;
    let mut is_open: bool = true;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    while i < size_command{
        if &command[i..i+1] == "d"{
            if i+size_dont_string < size_command{
                if &command[i..i+size_dont_string] == dont_string{
                    is_open = false;
                    i = i + size_dont_string;
                    continue;
                }
            }
            if i+size_do_string < size_command{
                if &command[i..i+size_do_string] == do_string{
                    is_open = true;
                    i = i + size_do_string;
                    continue;
                }
            }        
        }
        else if (&command[i..i+1] == "m") && is_open{
            let max = cmp::min(i+max_mul_command, size_command);
            for cap in re.captures_iter(&command[i..max]) {
                let x: u32 = cap[1].parse().unwrap();
                let y: u32 = cap[2].parse().unwrap();

                answer += x * y;
                //println!("Found mul({}, {})", x, y);
            }

        }
        
        i += 1;
    }



    println!("Result {}", answer);

    Ok(())
}


pub fn run() {
    let _ = solution1("inputs/example3.txt");
    let _ = solution1("inputs/input3.txt");
    let _ = solution2("inputs/example3.txt");
    let _ = solution2("inputs/input3.txt");
}
