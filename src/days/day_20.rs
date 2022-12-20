use std::process::exit;
use itertools::Itertools;
use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayTwenty {}

fn move_me(arr: &mut [i64], indices: &mut [usize], index: usize) {
    let old_index = index;
    let mut new_index;
    if (index as i64 + arr[index]) <= 0 {
        // underflow
        new_index = (arr.len() as i64 - 1 + (index as i64 + arr[index]) as i64 % (arr.len() - 1) as i64) as usize;
    } else if (index as i64 + arr[index]) as usize >= arr.len() {
        // overflow
        new_index = (index as i64 + arr[index]) as usize % (arr.len() - 1);
    } else {
        new_index = (index as i64 + arr[index]) as usize;
    }
    if old_index < new_index {
        arr[old_index..=new_index].rotate_left(1);
        indices[old_index..=new_index].rotate_left(1);
    } else {
        arr[new_index..=old_index].rotate_right(1);
        indices[new_index..=old_index].rotate_right(1);
    }
}

impl Problem for DayTwenty {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut numbers = contents.iter().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        let mut indices = (0..numbers.len()).collect::<Vec<usize>>();

        for i in 0..numbers.len() {
            let j = indices.iter().position(|n| *n == i).unwrap();
            move_me(&mut numbers, &mut indices, j);
            //println!("{numbers:?}");
        }
        let zero_position = numbers.iter().position(|n| *n == 0).unwrap();

        let mut sum = 0;
        for i in (1000..=3000).step_by(1000) {
            sum += numbers[(zero_position + i) % numbers.len()];
            // println!("{i}th number from 0: {}", numbers[(zero_position + i) % numbers.len()]);
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut numbers = contents.iter().map(|l| 811589153 * l.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        println!("{numbers:?}");

        let mut indices = (0..numbers.len()).collect::<Vec<usize>>();

        for round in 0..10 {
            for i in 0..numbers.len() {
                let j = indices.iter().position(|n| *n == i).unwrap();
                move_me(&mut numbers, &mut indices, j);
                //println!("{numbers:?}");
            }
        }
        let zero_position = numbers.iter().position(|n| *n == 0).unwrap();

        let mut sum = 0;
        for i in (1000..=3000).step_by(1000) {
            sum += numbers[(zero_position + i) % numbers.len()];
            println!("{i}th number from 0: {}", numbers[(zero_position + i) % numbers.len()]);
        }

        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}