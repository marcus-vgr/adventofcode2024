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

fn add_antinode(antinode_table: &mut Vec<Vec<char>>, r1: usize, c1: usize, r2: usize, c2: usize){

    let r1 = r1 as isize;
    let r2 = r2 as isize;
    let c1 = c1 as isize;
    let c2 = c2 as isize;
    let nrows: isize = antinode_table.len() as isize;
    let ncols: isize = antinode_table[0].len() as isize;

    let cdist = c2 - c1;
    let rdist = r2 - r1;

    let (r_node1, c_node1) = (r1-rdist, c1-cdist);
    let (r_node2, c_node2) = (r2+rdist, c2+cdist);

    if (r_node1 >= 0) && (r_node1 < nrows) && (c_node1 >= 0) && (c_node1 < ncols){
        antinode_table[r_node1 as usize][c_node1 as usize] = '#';
    }
    if (r_node2 >= 0) && (r_node2 < nrows) && (c_node2 >= 0) && (c_node2 < ncols){
        antinode_table[r_node2 as usize][c_node2 as usize] = '#';
    }

}


fn add_antinode_resonate(antinode_table: &mut Vec<Vec<char>>, r1: usize, c1: usize, r2: usize, c2: usize){

    let r1 = r1 as isize;
    let r2 = r2 as isize;
    let c1 = c1 as isize;
    let c2 = c2 as isize;
    let nrows: isize = antinode_table.len() as isize;
    let ncols: isize = antinode_table[0].len() as isize;

    let cdist = c2 - c1;
    let rdist = r2 - r1;

    let (mut r_node1, mut c_node1) = (r1-0, c1-0);
    loop {
        if (r_node1 >= 0) && (r_node1 < nrows) && (c_node1 >= 0) && (c_node1 < ncols){
            antinode_table[r_node1 as usize][c_node1 as usize] = '#';
            (r_node1, c_node1) = (r_node1-rdist, c_node1-cdist);
        }
        else{
            break;
        }    
    }

    let (mut r_node2, mut c_node2) = (r1+0, c1+0);
    loop {
        if (r_node2 >= 0) && (r_node2 < nrows) && (c_node2 >= 0) && (c_node2 < ncols){
            antinode_table[r_node2 as usize][c_node2 as usize] = '#';
            (r_node2, c_node2) = (r_node2+rdist, c_node2+cdist);
        }
        else{
            break;
        }    
    }

}


fn solution1(input: &str) -> io::Result<()>{
    println!("Running Day8, sol1, {}", input);

    let antenna_table = getting_input(input).expect("Failed to get input");
    let nrows: usize = antenna_table.len();
    let ncols: usize = antenna_table[0].len();
    let mut antinode_table: Vec<Vec<char>> = vec![vec!['.'; ncols]; nrows];

    for r1 in 0..nrows{
        for c1 in 0..ncols{
            let c: char = antenna_table[r1][c1];
            if c == '.'{
                continue;
            }
            
            for c2 in c1+1..ncols{
                if antenna_table[r1][c2] == c{
                    add_antinode(&mut antinode_table, r1, c1, r1, c2);
                }
            }

            for r2 in r1+1..nrows{
                for c2 in 0..ncols{
                    if antenna_table[r2][c2] == c{
                        add_antinode(&mut antinode_table, r1, c1, r2, c2);
                    }
                } 
            }
        }
    }

    
    let mut answer: u32 = 0;
    for r in 0..nrows{
        for c in 0..ncols{
            if antinode_table[r][c] == '#'{
                answer += 1;
            }
        }
    }

    println!("{}", answer);

    Ok(())
}


fn solution2(input: &str) -> io::Result<()>{
    println!("Running Day8, sol2, {}", input);

    let antenna_table = getting_input(input).expect("Failed to get input");
    let nrows: usize = antenna_table.len();
    let ncols: usize = antenna_table[0].len();
    let mut antinode_table: Vec<Vec<char>> = vec![vec!['.'; ncols]; nrows];

    for r1 in 0..nrows{
        for c1 in 0..ncols{
            let c: char = antenna_table[r1][c1];
            if c == '.'{
                continue;
            }
            
            for c2 in c1+1..ncols{
                if antenna_table[r1][c2] == c{
                    add_antinode_resonate(&mut antinode_table, r1, c1, r1, c2);
                }
            }

            for r2 in r1+1..nrows{
                for c2 in 0..ncols{
                    if antenna_table[r2][c2] == c{
                        add_antinode_resonate(&mut antinode_table, r1, c1, r2, c2);
                    }
                } 
            }
        }
    }

    
    let mut answer: u32 = 0;
    for r in 0..nrows{
        for c in 0..ncols{
            if antinode_table[r][c] == '#'{
                answer += 1;
            }
        }
    }

    println!("{}", answer);

    Ok(())
}


pub fn run() {
    let _ = solution1("inputs/example8.txt");
    let _ = solution1("inputs/input8.txt");
    let _ = solution2("inputs/example8.txt");
    let _ = solution2("inputs/input8.txt");
}
