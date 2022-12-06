use crate::problem::Problem;
use crate::io::{read_file_lines, read_from_csv};

pub struct DaySix {}

fn get_start_packet(payload: &str) -> usize {
    let mut last_four = vec![];
    for (i, c) in payload.chars().enumerate() {
        if i < 4 {
            last_four.push(c);
        } else {
            last_four.rotate_left(1);
            let len = last_four.len();
            last_four[len - 1] = c;
            let mut count = 0;
            for j in last_four.iter() {
                if last_four.iter().filter(|n| *n == j).count() > 1 {
                    count += 1;
                }
            }
            if count == 0 {
                return i + 1;
            }
        }
    }
    0
}


fn get_start_message(payload: &str) -> usize {
    let mut last_four = vec![];
    for (i, c) in payload.chars().enumerate() {
        if i < 14 {
            last_four.push(c);
        } else {
            last_four.rotate_left(1);
            let len = last_four.len();
            last_four[len - 1] = c;
            let mut count = 0;
            for j in last_four.iter() {
                if last_four.iter().filter(|n| *n == j).count() > 1 {
                    count += 1;
                }
            }
            if count == 0 {
                return i + 1;
            }
        }
    }
    0
}

impl Problem for DaySix {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        println!("contents: {contents:?}");
        let payload = contents.first().unwrap();
        let start = get_start_packet(&payload);
        format!("{}", start)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        println!("contents: {contents:?}");
        let payload = contents.first().unwrap();
        let start = get_start_message(&payload);
        format!("{}", start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}