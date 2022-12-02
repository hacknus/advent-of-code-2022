use crate::io::{read_file_lines, read_from_csv_to_String};
use crate::problem::Problem;
use std::cmp::Ordering;

pub struct DayOne {}

#[derive(Debug, Clone)]
pub struct Elf {
    id: usize,
    sum: u32,
    food: Vec<u32>,
}

impl Problem for DayOne {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut counter = 0;
        let mut elves = vec![];
        let mut elf = Elf { id: 0, sum: 0, food: vec![] };
        for content in contents.iter() {
            if content == "" {
                elf.sum = elf.food.iter().sum::<u32>();
                elves.push(elf);
                counter += 1;
                elf = Elf { id: counter, sum: 0, food: vec![] };
            } else {
                elf.food.push(content.parse::<u32>().expect("unable to parse to u32"))
            }
        }
        elf.sum = elf.food.iter().sum::<u32>();
        elves.push(elf);
        let index_of_max= elves
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| (a.sum as f32).total_cmp(&(b.sum as f32)))
            .map(|(index, _)| index).unwrap() + 1;

        let max_calories = elves.iter().fold(f32::NEG_INFINITY, |ai, bi| ai.max(bi.sum as f32));
        format!("{index_of_max}: {max_calories}", )
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut counter = 0;
        let mut elves = vec![];
        let mut elf = Elf { id: 0, sum: 0, food: vec![] };
        for content in contents.iter() {
            if content == "" {
                elf.sum = elf.food.iter().sum::<u32>();
                elves.push(elf);
                counter += 1;
                elf = Elf { id: counter, sum: 0, food: vec![] };
            } else {
                elf.food.push(content.parse::<u32>().expect("unable to parse to u32"))
            }
        }
        elf.sum = elf.food.iter().sum::<u32>();
        elves.push(elf);
        let index_of_max= elves
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| (a.sum as f32).total_cmp(&(b.sum as f32)))
            .map(|(index, _)| index).unwrap() + 1;

        let max_calories = elves.iter().fold(f32::NEG_INFINITY, |ai, bi| ai.max(bi.sum as f32));

        elves.sort_by_key(|e| e.sum);
        let mut top_three = 0;
        for i in elves.len()-3..elves.len() {
            top_three += elves[i].sum;
        }

        format!("Top Three: {top_three}", )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}