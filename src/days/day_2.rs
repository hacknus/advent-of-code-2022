use crate::io::read_from_csv_to_String;
use crate::problem::Problem;

pub struct DayTwo {}

// A ROCK
// B PAPER
// C SCISSORS

// X ROCK
// Y PAPER
// Z SCISSORS

pub fn get_opponent(opponent: &str) -> u32 {
    match opponent {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0
    }
}

pub fn get_me(me: &str) -> u32 {
    match me {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0
    }
}

pub fn get_new_score(me: &str) -> u32 {
    match me {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0
    }
}

pub fn get_score(opponent: &str, me: &str) -> u32 {
    let opponent = get_opponent(opponent);
    let me = get_me(me);
    let outcome = match (opponent, me) {
        (1, 1) => 3,
        (2, 2) => 3,
        (3, 3) => 3,
        (1, 2) => 6,
        (1, 3) => 0,
        (2, 1) => 0,
        (2, 3) => 6,
        (3, 1) => 6,
        (3, 2) => 0,
        _ => 0
    };
    me + outcome
}

impl Problem for DayTwo {
    fn part_one(&self, input: &str) -> String {
        let contents = read_from_csv_to_String(input);
        let mut score = 0;
        for row in contents {
            score += get_score(&row[0], &row[1]);
        }
        format!("{score}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut contents = read_from_csv_to_String(input);
        let mut score = 0;
        for row in contents.iter_mut() {
            let my_move = match row[1].as_str() {
                "X" => match row[0].as_str() { // i want to lose
                    "A" => 3,
                    "B" => 1,
                    "C" => 2,
                    _ => 0
                },
                "Y" => match row[0].as_str() { // i want a draw
                    "A" => 1,
                    "B" => 2,
                    "C" => 3,
                    _ => 0
                },
                "Z" => match row[0].as_str() { // i want to win
                    "A" => 2,
                    "B" => 3,
                    "C" => 1,
                    _ => 0
                },
                _ => 0
            };
            score += my_move + get_new_score(&row[1]);
        }
        format!("{score}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}