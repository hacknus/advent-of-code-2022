use num::integer::Roots;
use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayTwelve {}


pub fn get_left(pos: &[usize; 2]) -> Option<[usize; 2]> {
    if pos[0] > 0 {
        Some([pos[0] - 1, pos[1]])
    } else {
        None
    }
}

pub fn get_right(pos: &[usize; 2]) -> Option<[usize; 2]> {
    Some([pos[0] + 1, pos[1]])
}

pub fn get_up(pos: &[usize; 2]) -> Option<[usize; 2]> {
    if pos[1] > 0 {
        Some([pos[0], pos[1] - 1])
    } else {
        None
    }
}

pub fn get_down(pos: &[usize; 2]) -> Option<[usize; 2]> {
    Some([pos[0], pos[1] + 1])
}

pub fn is_adjacent(me: &[usize; 2], other: &[usize; 2]) -> bool {
    match get_down(&me) {
        None => {}
        Some(down) => {
            if down == *other {
                return true;
            }
        }
    }
    match get_up(&me) {
        None => {}
        Some(up) => {
            if up == *other {
                return true;
            }
        }
    }
    match get_right(&me) {
        None => {}
        Some(right) => {
            if right == *other {
                return true;
            }
        }
    }
    match get_left(&me) {
        None => {}
        Some(left) => {
            if left == *other {
                return true;
            }
        }
    }
    false
}

impl Problem for DayTwelve {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut grid = vec![];
        let letters = "abcdefghijklmnopqrstuvwxyz";
        let mut target = [0; 2];
        let mut initial = [0; 2];
        for (y, line) in contents.iter().enumerate() {
            let mut x = 0;
            let row = line.chars().map(|s| {
                match letters.chars().position(|si| si == s) {
                    None => {
                        if s == 'S' {
                            initial = [x, y];
                            x += 1;
                            0
                        } else if s == 'E' {
                            target = [x, y];
                            x += 1;
                            letters.len() as u32 - 1
                        } else {
                            panic!("error: {s}");
                        }
                    }
                    Some(si) => {
                        x += 1;
                        si as u32
                    }
                }
            }).collect::<Vec<u32>>();
            grid.push(row);
        }
        let current_height = 0;
        println!("{grid:?}");
        println!("{initial:?}");
        println!("{target:?}");
        for line in grid.iter() {
            println!("{line:?}");
        }
        let mut currently_wet = vec![initial];
        let mut history = currently_wet.clone();
        let mut counter = 0;
        loop {
            // get all adjacent with equal or 1 lower
            let mut new_currently_wet = vec![];
            for i in 0..currently_wet.len() {
                let cell = currently_wet[i];
                for x in 0..grid[0].len() {
                    for y in 0..grid.len() {
                        let current_altitude = grid[cell[1]][cell[0]];
                        if grid[y][x] <= current_altitude + 1 {
                            if is_adjacent(&cell, &[x, y]) {
                                if !history.contains(&[x, y]) {
                                    new_currently_wet.push([x, y]);
                                    history.push([x, y]);
                                }
                            }
                        }
                    }
                }
            }
            currently_wet = new_currently_wet;
            counter += 1;
            println!("{currently_wet:?}");
            // check if target is in there
            if currently_wet.contains(&target) {
                break;
            }
        }

        format!("{}", counter)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        println!("{contents:?}");
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}