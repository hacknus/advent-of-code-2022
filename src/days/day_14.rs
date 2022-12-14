use std::os::unix::raw::uid_t;
use crate::io::read_file_lines;
use crate::problem::Problem;
use array2d::{Array2D, Error};

pub struct DayFourteen {}

pub fn drop(grid: &mut Array2D<u32>, start: [usize; 2]) -> u32 {
    let mut counter = 0;
    let mut x = start[0];
    let mut y = start[1];
    loop {
        if y + 1 >= grid.num_columns() {
            println!("reached bottom");
            break;
        }
        if x + 1 >= grid.num_rows() {
            println!("reached right");
            break;
        }
        if x - 1 < 0 {
            println!("reached left");
            break;
        }
        if grid[(x, y + 1)] == 0 {
            // next is free
            y += 1;
        } else {
            // next is blocked, check left
            if grid[(x - 1, y + 1)] == 0 {
                // left is free
                y += 1;
                x -= 1;
            } else {
                // left is blocked, check right
                if grid[(x + 1, y + 1)] == 0 {
                    // right is free
                    y += 1;
                    x += 1;
                } else {
                    // right is blocked, place here
                    grid[(x, y)] = 2;
                    counter += 1;
                    if x == start[0] && y == start[1] {
                        println!("source is blocked!");
                        break;
                    }
                    x = start[0];
                    y = start[1];
                }
            }
        }
    }
    counter
}

impl Problem for DayFourteen {
    fn part_one(&self, input: &str) -> String {
        let mut contents = read_file_lines(input);
        let mut lines = vec![];
        let mut xs = vec![];
        let mut ys = vec![];
        println!("contents: {contents:?}");
        for line in contents.iter_mut() {
            let vals = line.split(" -> ").map(|p| {
                let ps = p.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
                let x = ps[0].parse::<i32>().unwrap();
                let y = ps[1].parse::<i32>().unwrap();
                xs.push(x);
                ys.push(y);
                [x, y]
            }).collect::<Vec<[i32; 2]>>();
            lines.push(vals);
        }
        println!("lines: {lines:?}");
        xs.push(500);
        ys.push(0);
        let min_x = xs.iter().min().unwrap() - 1;
        let max_x = xs.iter().max().unwrap() + 1;
        let min_y = ys.iter().min().unwrap();
        let max_y = ys.iter().max().unwrap();
        println!("x: {min_x:?} < {max_x:?}");
        println!("y: {min_y:?} < {max_y:?}");
        let n = max_x - min_x;
        let m = max_y - min_y + 1;
        let mut grid = Array2D::filled_with(0, n as usize, m as usize);
        println!("n={} m={}", grid.num_rows(), grid.num_columns());
        for line in lines.iter() {
            for i in 0..line.len() - 1 {
                let mut start_x = line[i][0] - min_x;
                let mut start_y = line[i][1] - min_y;
                let mut end_x = line[i + 1][0] - min_x;
                let mut end_y = line[i + 1][1] - min_y;
                if start_x > end_x {
                    (start_x, end_x) = (end_x, start_x);
                }
                if start_y > end_y {
                    (start_y, end_y) = (end_y, start_y);
                }
                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        grid[(x as usize, y as usize)] = 1;
                    }
                }
            }
        }

        let initial = [500 - min_x as usize, 0 - *min_y as usize];
        let amount = drop(&mut grid, initial);
        format!("{}", amount)
    }

    fn part_two(&self, input: &str) -> String {
        let mut contents = read_file_lines(input);
        let mut lines = vec![];
        let mut xs = vec![];
        let mut ys = vec![];
        println!("contents: {contents:?}");
        for line in contents.iter_mut() {
            let vals = line.split(" -> ").map(|p| {
                let ps = p.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
                let x = ps[0].parse::<i32>().unwrap();
                let y = ps[1].parse::<i32>().unwrap();
                xs.push(x);
                ys.push(y);
                [x, y]
            }).collect::<Vec<[i32; 2]>>();
            lines.push(vals);
        }
        println!("lines: {lines:?}");
        xs.push(500);
        ys.push(0);
        let mut min_x = xs.iter().min().unwrap() - 1;
        let mut max_x = xs.iter().max().unwrap() + 1;
        let min_y = ys.iter().min().unwrap();
        let max_y = ys.iter().max().unwrap();
        let delta = (max_y - min_y).abs() / 1;
        min_x -= delta;
        max_x += delta;
        println!("x: {min_x:?} < {max_x:?}");
        println!("y: {min_y:?} < {max_y:?}");
        let n = max_x - min_x;
        let m = max_y - min_y + 1 + 2;
        let mut grid = Array2D::filled_with(0, n as usize, m as usize);
        println!("n={} m={}", grid.num_rows(), grid.num_columns());
        for line in lines.iter() {
            for i in 0..line.len() - 1 {
                let mut start_x = line[i][0] - min_x;
                let mut start_y = line[i][1] - min_y;
                let mut end_x = line[i + 1][0] - min_x;
                let mut end_y = line[i + 1][1] - min_y;
                if start_x > end_x {
                    (start_x, end_x) = (end_x, start_x);
                }
                if start_y > end_y {
                    (start_y, end_y) = (end_y, start_y);
                }
                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        grid[(x as usize, y as usize)] = 1;
                    }
                }
            }
        }
        for x in 0..n {
            grid[(x as usize, m as usize - 1)] = 1;
        }
        for row in grid.as_columns().iter() {
            println!("{row:?}");
        }

        let initial = [500 - min_x as usize, 0 - *min_y as usize];
        let amount = drop(&mut grid, initial);

        for row in grid.as_columns().iter() {
            println!("{row:?}");
        }
        format!("{}", amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}