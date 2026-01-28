use crate::utils::read_lines;
use std::io;

fn getting_input(input: &str) -> io::Result<Vec<Vec<char>>>{

    let lines = read_lines(input).expect("File not found");
    let mut table: Vec<Vec<char>> = Vec::new();
    
    for line in lines{
        let line = line?;
        table.push(line.chars().collect());
    }

    Ok(table)

}


fn solution1(input: &str) -> io::Result<()>{
    println!("Running Day4, sol1, {}", input);

    let table = getting_input(input).expect("Failed to get input");
    let nrows = table.len();
    let ncols = table[0].len();
    let target: Vec<char> = vec!['X','M','A','S'];
    let size_target = target.len();
    
    let mut answer: u32 = 0;

    for row in 0..=nrows-1{
        for col in 0..=ncols-1{
            if table[row][col] != target[0]{
                continue;
            }
            
            // horizonal left
            if col + size_target <= ncols{
                let candidate: Vec<char> = (0..size_target).map(|i| table[row][col+i]).collect();
                if candidate == target{
                    answer += 1;
                }
            }

            // horizontal right
            if col >= size_target-1{
                let candidate: Vec<char> = (0..size_target).map(|i| table[row][col-i]).collect();
                if candidate == target{
                    answer += 1;
                }
            }
            

            // vertical up
            if row >= size_target-1{
                let candidate: Vec<char> = (0..size_target).map(|i| table[row-i][col]).collect();
                if candidate == target{
                    answer += 1;
                }
            }

            // vertical down
            if row+size_target <= nrows{
                let candidate: Vec<char> = (0..size_target).map(|i| table[row+i][col]).collect();
                if candidate == target{
                    answer += 1;
                }
            }
            
            // diagonal NE
            if row >= size_target-1 && col + size_target <= ncols{
                let candidate: Vec<char> = (0..size_target).map(|i| table[row-i][col+i]).collect();
                if candidate == target{
                    answer += 1;
                }
            }

            // diagonal NW
            if row >= size_target-1 && col >= size_target-1{
                let candidate: Vec<char> = (0..size_target).map(|i| table[row-i][col-i]).collect();
                if candidate == target{
                    answer += 1;
                }
            }
            
            // diagonal SE
            if row + size_target <= nrows && col + size_target <= ncols{
                let candidate: Vec<char> = (0..size_target).map(|i| table[row+i][col+i]).collect();
                if candidate == target{
                    answer += 1;
                }
            }

            // diagonal SW
            if row + size_target <= nrows && col >= size_target-1{
                let candidate: Vec<char> = (0..size_target).map(|i| table[row+i][col-i]).collect();
                if candidate == target{
                    answer += 1;
                }
            }
            

        }
    }
    
    println!("Result: {}", answer);
    
    Ok(())
}


fn solution2(input: &str) -> io::Result<()>{
    println!("Running Day4, sol2, {}", input);

    let table = getting_input(input).expect("Failed to get input");
    let nrows = table.len();
    let ncols = table[0].len();
    let target: Vec<char> = vec!['M','A','S'];
    let target_rev: Vec<char> = target.iter().rev().copied().collect();
    
    let mut answer: u32 = 0;

    for row in 0..=nrows-1{
        for col in 0..=ncols-1{
            if table[row][col] != 'A'{
                continue;
            }
            
            if col+1 < ncols && col >= 1 && row >= 1 && row+1 < nrows{
                let d1: Vec<char> = (0..3).map(|i| table[row-1+i][col-1+i]).collect();
                let d2: Vec<char> = (0..3).map(|i| table[row-1+i][col+1-i]).collect();

                if (d1 == target || d1 == target_rev) && (d2 == target || d2 == target_rev){
                    answer += 1;
                }
            }
        }
    }
    
    println!("Result: {}", answer);
    
    Ok(())
}




pub fn run() {
    let _ = solution1("inputs/example4.txt");
    let _ = solution1("inputs/input4.txt");
    let _ = solution2("inputs/example4.txt");
    let _ = solution2("inputs/input4.txt");
}
