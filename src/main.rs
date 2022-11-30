mod io;
mod problem;
mod days;

use problem::Problem;
use days::day_1::DayOne;

fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(DayOne{})),
        //2 => Some(Box::new(DayTwo{})),
        //3 => Some(Box::new(DayThree{})),
        // ...
        _ => None
    }
}

fn main() {
    println!("Hello, world!");
}
