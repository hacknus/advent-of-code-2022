use crate::problem::Problem;
use crate::io::read_from_csv_to_String_by_comma;
use regex::{Match, Regex};

pub struct DayFour {}

impl Problem for DayFour {
    fn part_one(&self, input: &str) -> String {
        let contents = read_from_csv_to_String_by_comma(input);
        println!("contents: {contents:?}");
        let re = Regex::new(r"[0-9]*").unwrap();
        let mut fully_overlap = 0;
        for pairs in contents {
            let mut first_elf = [0; 2];
            let mut second_elf = [0; 2];
            for (i, cap) in re.captures_iter(&pairs[0]).enumerate() {
                match cap.get(0) {
                    None => {}
                    Some(number) => {
                        first_elf[i] = number.as_str().parse::<i32>().unwrap();
                    }
                }
            }
            for (i, cap) in re.captures_iter(&pairs[1]).enumerate() {
                match cap.get(0) {
                    None => {}
                    Some(number) => {
                        second_elf[i] = number.as_str().parse::<i32>().unwrap();
                    }
                }
            }

            if first_elf[0] <= second_elf[0] && second_elf[1] <= first_elf[1] {
                fully_overlap += 1;
            } else if second_elf[0] <= first_elf[0] && first_elf[1] <= second_elf[1] {
                fully_overlap += 1;
            }
        }
        format!("{}", fully_overlap)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_from_csv_to_String_by_comma(input);
        println!("contents: {contents:?}");
        let re = Regex::new(r"[0-9]*").unwrap();
        let mut partially_overlap = 0;
        let mut fully_overlap = 0;
        for pairs in contents {
            let mut first_elf = [0; 2];
            let mut second_elf = [0; 2];
            for (i, cap) in re.captures_iter(&pairs[0]).enumerate() {
                match cap.get(0) {
                    None => {}
                    Some(number) => {
                        first_elf[i] = number.as_str().parse::<i32>().unwrap();
                    }
                }
            }
            for (i, cap) in re.captures_iter(&pairs[1]).enumerate() {
                match cap.get(0) {
                    None => {}
                    Some(number) => {
                        second_elf[i] = number.as_str().parse::<i32>().unwrap();
                    }
                }
            }

            if (first_elf[0]..first_elf[1] + 1).contains(&second_elf[0]) ||
                (first_elf[0]..first_elf[1] + 1).contains(&second_elf[1]) {
                partially_overlap += 1;
            } else if (second_elf[0]..second_elf[1] + 1).contains(&first_elf[0]) ||
                (second_elf[0]..second_elf[1] + 1).contains(&first_elf[1]) {
                partially_overlap += 1;
            }
        }
        format!("{}", partially_overlap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}