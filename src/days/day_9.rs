use std::os::unix::raw::uid_t;
use crate::io::{read_file_lines, read_from_csv};
use crate::problem::Problem;
use itertools::Itertools;

pub struct DayNine {}

pub fn get_sign(val: &i32) -> i32 {
    if *val > 0 {
        1
    } else {
        -1
    }
}

impl Problem for DayNine {
    fn part_one(&self, input: &str) -> String {
        let mut contents = read_from_csv(input, b' ');
        let mut pos_head = [0; 2];
        let mut pos_tail = [0; 2];
        let mut tail_positions = vec![pos_tail];
        for m in contents.iter() {
            let mut moving_distance = [0; 2];
            match m[0].as_str() {
                "R" => {
                    moving_distance[0] += m[1].parse::<i32>().unwrap();
                }
                "U" => {
                    moving_distance[1] += m[1].parse::<i32>().unwrap();
                }
                "L" => {
                    moving_distance[0] -= m[1].parse::<i32>().unwrap();
                }
                "D" => {
                    moving_distance[1] -= m[1].parse::<i32>().unwrap();
                }
                _ => {}
            }
            let mut sign = 1;
            if moving_distance[0] < 0 {
                sign = -1;
            }
            for _ in 0..moving_distance[0].abs() {
                pos_head[0] += sign as i32;
                let dif_x = pos_head[0] - pos_tail[0];
                let dif_y = pos_head[1] - pos_tail[1];
                if dif_x.abs() > 1 {
                    // check for diagonal
                    if dif_y.abs() == 1 {
                        pos_tail[1] += dif_y;
                        pos_tail[0] += dif_x - sign;
                    } else {
                        pos_tail[0] += dif_x - sign;
                    }
                    tail_positions.push(pos_tail);
                }
            }
            let mut sign = 1;
            if moving_distance[1] < 0 {
                sign = -1;
            }
            for _ in 0..moving_distance[1].abs() {
                pos_head[1] += sign as i32;
                let dif_x = pos_head[0] - pos_tail[0];
                let dif_y = pos_head[1] - pos_tail[1];
                if dif_y.abs() > 1 {
                    // check for diagonal
                    if dif_x.abs() == 1 {
                        pos_tail[0] += dif_x;
                        pos_tail[1] += dif_y - sign;
                    } else {
                        pos_tail[1] += dif_y - sign;
                    }
                    tail_positions.push(pos_tail);
                }
            }
        }
        let tail_positions = tail_positions.iter().unique();
        format!("{}", tail_positions.count())
    }

    fn part_two(&self, input: &str) -> String {
        let mut contents = read_from_csv(input, b' ');
        let mut pos_knots = [[0, 0]; 10];
        let mut tail_positions = vec![[0, 0]];
        for m in contents.iter() {
            let mut moving_distance = [0; 2];
            match m[0].as_str() {
                "R" => {
                    moving_distance[0] += m[1].parse::<i32>().unwrap();
                }
                "U" => {
                    moving_distance[1] += m[1].parse::<i32>().unwrap();
                }
                "L" => {
                    moving_distance[0] -= m[1].parse::<i32>().unwrap();
                }
                "D" => {
                    moving_distance[1] -= m[1].parse::<i32>().unwrap();
                }
                _ => {}
            }
            let mut sign = [1; 2];
            if moving_distance[0] < 0 {
                sign[0] = -1;
            } else if moving_distance[0] == 0 {
                sign[0] = 0;
            }
            if moving_distance[1] < 0 {
                sign[1] = -1;
            } else if moving_distance[1] == 0 {
                sign[1] = 0;
            }

            let to_move = moving_distance.iter().map(|s| s.abs()).max().unwrap();
            for _ in 0..to_move {
                pos_knots[0][0] += sign[0] as i32;
                pos_knots[0][1] += sign[1] as i32;
                for i in 1..pos_knots.len() {
                    let dif_x = pos_knots[i - 1][0] - pos_knots[i][0];
                    let dif_y = pos_knots[i - 1][1] - pos_knots[i][1];

                    if dif_y.abs() > 1 && dif_x.abs() == 1 {
                        pos_knots[i][0] += dif_x;
                        pos_knots[i][1] += dif_y - get_sign(&dif_y);
                        tail_positions.push(pos_knots.last().unwrap().clone());
                    } else if dif_y.abs() == 1 && dif_x.abs() > 1 {
                        pos_knots[i][0] += dif_x - get_sign(&dif_x);
                        pos_knots[i][1] += dif_y;
                        tail_positions.push(pos_knots.last().unwrap().clone());
                    } else if dif_y.abs() > 1 && dif_x.abs() > 1 {
                        pos_knots[i][0] += dif_x - get_sign(&dif_x);
                        pos_knots[i][1] += dif_y - get_sign(&dif_y);
                        tail_positions.push(pos_knots.last().unwrap().clone());
                    } else if dif_x.abs() > 1 {
                        pos_knots[i][0] += dif_x - get_sign(&dif_x);
                        tail_positions.push(pos_knots.last().unwrap().clone());
                    } else if dif_y.abs() > 1 {
                        pos_knots[i][1] += dif_y - get_sign(&dif_y);
                        tail_positions.push(pos_knots.last().unwrap().clone());
                    }
                }
            }
        }
        let tail_positions = tail_positions.iter().unique();
        format!("{}", tail_positions.count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}