use num::integer::Roots;
use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayTwelve {}

pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

pub fn get_weight(grid: &Vec<Vec<u32>>, location: &[usize; 2]) -> Vec<Vec<f32>> {
    let current_altitude = grid[location[1]][location[0]];
    let mut weight = vec![vec![1.0; grid[0].len()]; grid.len()];
    for altitude in (current_altitude..25).rev() {
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if altitude == grid[y][x] && [x, y] != *location && grid[y][x] - current_altitude <= 1 {
                    weight[y][x] += altitude as f32 / ((x as i32 - location[0] as i32).pow(2) + (y as i32 - location[1] as i32).pow(2)).sqrt() as f32;
                }
            }
        }
    }
    weight
}

pub fn make_move(weights: &Vec<Vec<f32>>, location: &[usize; 2]) -> Direction {
    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;
    for x in 0..weights[0].len() {
        for y in 0..weights.len() {
            if x < location[0] {
                left += (weights[y][x]*100.0) as u32;
            } else if x > location[0] {
                right += (weights[y][x]*100.0) as u32;
            }
            if y < location[1] {
                down += (weights[y][x]*100.0) as u32;
            } else if y > location[1] {
                up += (weights[y][x]*100.0) as u32;
            }
        }
    }
    if [up, down, right, left].iter().max().unwrap() == &up {
        Direction::UP
    } else if [up, down, right, left].iter().max().unwrap() == &down {
        Direction::DOWN
    } else if [up, down, right, left].iter().max().unwrap() == &left {
        Direction::LEFT
    } else if [up, down, right, left].iter().max().unwrap() == &right {
        Direction::RIGHT
    } else {
        Direction::UP
    }
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
                            letters.len() as u32
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
        let mut counter = 0;
        loop {
            if initial == target {
                break;
            }
            let weights = get_weight(&grid, &initial);
            for line in weights.iter() {
                println!("{line:?}");
            }
            let next_move = make_move(&weights, &initial);
            counter += 1;
            match next_move {
                Direction::LEFT => {
                    println!("going left");
                    initial[0] -= 1;
                }
                Direction::RIGHT => {
                    println!("going right");
                    initial[0] += 1;
                }
                Direction::UP => {
                    println!("going up");
                    initial[1] += 1;
                }
                Direction::DOWN => {
                    println!("going down");
                    initial[1] -= 1;
                }
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