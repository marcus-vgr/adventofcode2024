mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {

    let days: Vec<fn()> = vec![ // Vector where each element is the function of the day
        day1::run,
        day2::run,
        day3::run,
        day4::run,
        day5::run,
        day6::run,
    ];

    let day: usize = std::env::args() 
        .nth(1) // Capturing the second argument when running. Argument 0 is always programm name...
        .expect("Provide a day")
        .parse() // Convert the string (argument) into the dtype I want (usize). Basically it is doing "n".parse::<usize>()
        .expect("Invalid day");
    
    if let Some(run) = days.get(day - 1) {
        run();
    } else {
        eprintln!("Day not implemented");
    }

}
