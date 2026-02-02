use crate::utils::read_lines;
use std::io;
use std::collections::HashMap;

fn getting_input(input: &str) -> io::Result<Vec<Vec<char>>>{

    let lines = read_lines(input).expect("File not found");
    let mut table: Vec<Vec<char>> = Vec::new();
    
    for line in lines{
        let line = line?;
        table.push(line.chars().collect());
    }
    
    Ok(table)

}

fn get_guard_position(table: &Vec<Vec<char>>) -> (i16, i16){

    let ncols = table[0].len();
    let nrows = table.len();

    for r in 0..nrows{
        for c in 0..ncols{
            if table[r][c] == '^'{
                return (r as i16,c as i16);
            }
        } 
    }

    return (-1,-1);

}

fn rotate_guard(orientation: u16) -> u16{
    (orientation + 1) % 4
}

fn move_guard(orientation: u16, r: i16, c: i16) -> (i16, i16){
    match orientation {
        0 => (r-1,c), // go up
        1 => (r,c+1), // go right
        2 => (r+1,c), // go down
        3 => (r,c-1), // go left
        _ => (-1,-1),
    }

}

fn is_valid_coord(r: i16, c: i16, nrows: i16, ncols: i16) -> bool{
    if (r >= 0) && (r < nrows) && (c >= 0) && (c < ncols){
        return true;
    }
    return false;
}


fn solution1(input: &str) -> io::Result<()>{
    println!("Running Day6, sol1, {}", input);

    let mut table = getting_input(input).expect("Failed to get input");
    let ncols: i16 = table[0].len() as i16;
    let nrows: i16 = table.len() as i16;

    let (mut r, mut c) = get_guard_position(&table);
    table[r as usize][c as usize] = 'X';
    let mut orientation: u16 = 0;

    loop {
        let (rnew, cnew) = move_guard(orientation, r, c);
        
        if is_valid_coord(rnew, cnew, nrows, ncols){
            if table[rnew as usize][cnew as usize] == '#'{
                orientation = rotate_guard(orientation);
            }
            else{
                r = rnew;
                c = cnew;
                table[r as usize][c as usize] = 'X';
            }
        }
        else{
            break;
        }
        
    }

    let mut answer: u32 = 0;
    for r in 0..nrows{
        for c in 0..ncols{
            if table[r as usize][c as usize] == 'X'{
                answer += 1;
            }
        }
    }

    println!("{}", answer);

    Ok(())
}


fn solution2(input: &str) -> io::Result<()>{
    println!("Running Day6, sol2, {}", input);

    let main_table = getting_input(input).expect("Failed to get input");
    let ncols: i16 = main_table[0].len() as i16;
    let nrows: i16 = main_table.len() as i16;
    let mut answer: u32 = 0;

    let mut new_locations: Vec<(i16,i16)> = Vec::new();
    for r_extra in 0..nrows{
        for c_extra in 0..ncols{
            if main_table[r_extra as usize][c_extra as usize] == '.'{
                new_locations.push((r_extra,c_extra));
            }
        }
    } 

    for (r_extra, c_extra) in new_locations{

        let mut table = main_table.clone();

        let (mut r, mut c) = get_guard_position(&table);
        table[r as usize][c as usize] = 'X';
        table[r_extra as usize][c_extra as usize] = '#';
        let mut orientation: u16 = 0;

        let mut history: HashMap<(i16,i16), Vec<u16>> = HashMap::new();

        loop {
            let (rnew, cnew) = move_guard(orientation, r, c);
            
            if is_valid_coord(rnew, cnew, nrows, ncols){
                if table[rnew as usize][cnew as usize] == '#'{
                    orientation = rotate_guard(orientation);
                }
                else{
                    r = rnew;
                    c = cnew;
                    if table[r as usize][c as usize] == 'X'{
                        if let Some(h) = history.get_mut(&(r,c)){
                            if h.contains(&orientation){
                                answer += 1;
                                break;
                            }
                            else{
                                h.push(orientation);
                                r = rnew;
                                c = cnew; 
                            }
                        }
                    }
                    else{
                        table[r as usize][c as usize] = 'X';
                        history.insert((r,c), Vec::from([orientation]));    
                    }
                    
                }
            }
            else{
                break;
            }
            
        }


    }


    println!("{}", answer);

    Ok(())
}


pub fn run() {
    let _ = solution1("inputs/example6.txt");
    let _ = solution1("inputs/input6.txt");
    let _ = solution2("inputs/example6.txt");
    let _ = solution2("inputs/input6.txt");
}
