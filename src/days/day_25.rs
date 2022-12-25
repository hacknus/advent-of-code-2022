use std::num::ParseIntError;
use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayTwentyFive {}

pub fn snafu_to_decimal(line: &str) -> u64 {
    let mut val = 0;
    let length = line.len();
    for (i, c) in line.char_indices() {
        let place = 5_u64.pow(length as u32 - i as u32 - 1);
        match c {
            '0' => { val += place * 0 }
            '1' => { val += place * 1 }
            '2' => { val += place * 2 }
            '-' => { val -= place * 1 }
            '=' => { val -= place * 2 }
            _ => {}
        }
    }
    val
}

fn format_base_5(mut x: u64) -> String {
    let mut result = vec![];
    loop {
        result.push((x % 5).to_string());
        if x / 5 == 0 {
            break;
        }
        x = x / 5;
    }
    result.into_iter().rev().collect::<String>()
}

pub fn decimal_to_snafu(mut val: u64) -> String {
    let mut snafu = "".to_string();
    let base_5 = format_base_5(val);
    let mut take_over = 0;
    for (i, c) in base_5.to_string().char_indices().rev() {
        let mut number = c.to_string().parse::<u64>().unwrap();
        if take_over > 0 {
            number += take_over;
            take_over = 0;
        }
        if number <= 2 {
            snafu += &number.to_string();
        } else {
            if number == 3 {
                snafu += "=";
                take_over = 1;
            } else if number == 4 {
                snafu += "-";
                take_over = 1;
            } else if number == 5 {
                snafu += "0";
                take_over = 1;
            }
        }
    }
    if take_over > 0 {
        snafu += &take_over.to_string();
    }
    snafu.chars().rev().collect::<String>()
}

impl Problem for DayTwentyFive {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut sum = 0;
        for line in contents.iter() {
            sum += snafu_to_decimal(line.as_str());
        }
        let snafu_sum = decimal_to_snafu(sum);

        format!("{}", snafu_sum)
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}