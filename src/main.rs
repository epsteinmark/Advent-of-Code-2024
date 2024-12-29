use std::io;
mod day_1;
mod day_2;

pub struct Day {
    value: u8,
    solve_1: fn() -> String,
    solve_2: fn() -> String,
}

impl Day {
    pub fn new(value: u8) -> Day {
        match value {
            1 => Day { value , solve_1: day_1::solve_1, solve_2: day_1::solve_2},
            2 => Day { value, solve_1: day_2::solve_1, solve_2: day_2::solve_2},
            _ => panic!("I haven't done that day yet"),
        }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

pub enum Part {
    One,
    Two,
}

fn main() {

    println!("What day challenge would you like to attempt?");

    let mut day = String::new();
    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line");

    let day = Day::new(day.trim_end().parse().unwrap());

    println!("What part would you like to attempt?");

    let mut part = String::new();
    io::stdin()
        .read_line(&mut part)
        .expect("Failed to read line");

    let ans = match part.trim_end().parse().unwrap() {
        1 => (day.solve_1)(),
        2 => (day.solve_2)(),
        _ => panic!("Invalid part"),
    };

    println!("Answer: {ans}");
}
