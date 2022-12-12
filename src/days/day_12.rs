use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayTwelve {}

pub fn walk(grid: &Vec<Vec<usize>>, index: &[usize; 2], last: &[usize; 2], target: &[usize; 2]) -> u32 {
    let mut walk_counter = 0;
    if index[0] == target[0] && index[1] == target[1] {
        return walk_counter;
    }
    println!("index of walk {index:?}, index of last {last:?}");
    let current_position = grid[index[0]][index[1]];
    if index[0] > 0 {
        let left = [index[0] - 1, index[1]];
        if left[0] != last[0] || left[1] != last[1] {
            if (current_position as i32 - grid[left[0]][left[1]] as i32).abs() <= 1 {
                println!("moving left");
                walk_counter += walk(grid, &left, index, target);
            }
        }
    }
    if index[0] < grid.len() - 1 {
        let right = [index[0] + 1, index[1]];
        if right[0] != last[0] || right[1] != last[1] {
            if (current_position as i32 - grid[right[0]][right[1]] as i32).abs() <= 1 {
                println!("moving right");
                walk_counter += walk(grid, &right, index, target);
            }
        }
    }
    if index[1] < grid[0].len() - 1 {
        let up = [index[0], index[1] + 1];
        if up[0] != last[0] || up[1] != last[1] {
            if (current_position as i32 - grid[up[0]][up[1]] as i32).abs() <= 1 {
                println!("moving up");
                walk_counter += walk(grid, &up, index, target);
            }
        }
    }
    if index[1] > 0 {
        let down = [index[0], index[1] - 1];
        if down[0] != last[0] || down[1] != last[1] {
            if (current_position as i32 - grid[down[0]][down[1]] as i32).abs() <= 1 {
                println!("moving down");
                walk_counter += walk(grid, &down, index, target);
            }
        }
    }
    walk_counter
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
                            25
                        } else {
                            panic!("error: {s}");
                        }
                    }
                    Some(si) => {
                        x += 1;
                        si
                    }
                }
            }).collect::<Vec<usize>>();
            grid.push(row);
        }
        let current_height = 0;
        println!("{grid:?}");
        println!("{initial:?}");
        println!("{target:?}");
        let counter = walk(&grid, &initial, &initial, &target);
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