use crate::problem::Problem;
use crate::io::read_file_lines;

pub struct DayThree {}

impl Problem for DayThree {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let letters = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n",
                           "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N",
                           "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
        let mut sum = 0;
        for rucksack in contents {
            let half = rucksack.len() / 2 as usize;
            let first = &rucksack[0..half];
            let second = &rucksack[half..rucksack.len()];
            let mut priority = 0;
            for (i, letter) in letters.iter().enumerate() {
                if first.contains(letter) && second.contains(letter) {
                    priority = i + 1;
                    break;
                }
            }
            sum += priority;
        }
        format!("{sum}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut contents = read_file_lines(input);
        let letters = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n",
                           "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N",
                           "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
        let mut chunks = vec![];
        for i in (0..contents.len()).step_by(3) {
            let mut chunk = vec![];
            for j in 0..3 {
                let rucksack = contents[i + j].clone();
                chunk.push(rucksack);
            }
            chunks.push(chunk);
        }
        let mut sum = 0;
        for chunk in chunks {
            let mut priority = 0;
            for (i, letter) in letters.iter().enumerate() {
                let mut counter = 0;
                for rucksack in chunk.iter() {
                    if rucksack.contains(letter) {
                        counter += 1;
                    }
                }
                if counter == chunk.len() {
                    priority = i + 1;
                    break;
                }
            }
            sum += priority;
        }
        format!("{sum}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}