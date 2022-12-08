use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayEight {}

impl Problem for DayEight {
    fn part_one(&self, input: &str) -> String {
        let mut contents = read_file_lines(input);
        let mut matrix = vec![];
        for line in contents.iter() {
            let c = line.chars().map(|s| s.to_digit(10).unwrap()).collect::<Vec<u32>>();
            matrix.push(c);
        }
        let n = matrix.len();
        let m = matrix[0].len();
        let mut num_trees = 0;
        for i in 1..n - 1 {
            for j in 1..m - 1 {
                let mut visible = 0;
                let current_tree = &matrix[i][j];
                let left_trees = &matrix[i][0..j];
                let right_trees = &matrix[i][j + 1..m];
                let down_trees = &matrix[i + 1..n].iter().map(|s| s[j]).collect::<Vec<u32>>();
                let top_trees = &matrix[0..i].iter().map(|s| s[j]).collect::<Vec<u32>>();

                if current_tree > left_trees.iter().max().unwrap() {
                    visible += 1;
                }
                if current_tree > right_trees.iter().max().unwrap() {
                    visible += 1;
                }
                if current_tree > down_trees.iter().max().unwrap() {
                    visible += 1;
                }
                if current_tree > top_trees.iter().max().unwrap() {
                    visible += 1;
                }
                if visible >= 1 {
                    num_trees += 1;
                }
            }
        }
        num_trees += 2 * n + 2 * (m - 2);
        format!("{}", num_trees)
    }

    fn part_two(&self, input: &str) -> String {
        let mut contents = read_file_lines(input);
        let mut matrix = vec![];
        for line in contents.iter() {
            let c = line.chars().map(|s| s.to_digit(10).unwrap()).collect::<Vec<u32>>();
            matrix.push(c);
        }
        let n = matrix.len();
        let m = matrix[0].len();
        let mut scores = vec![];
        for i in 1..n - 1 {
            for j in 1..m - 1 {
                let mut scenic_score = 1;
                let current_tree = &matrix[i][j];
                let left_trees = &matrix[i][0..j];
                let right_trees = &matrix[i][j + 1..m];
                let down_trees = &matrix[i + 1..n].iter().map(|s| s[j]).collect::<Vec<u32>>();
                let top_trees = &matrix[0..i].iter().map(|s| s[j]).collect::<Vec<u32>>();

                let mut ind = 1;
                for (k, s) in left_trees.iter().rev().enumerate() {
                    ind = k + 1;
                    if s >= current_tree {
                        break;
                    }
                }
                scenic_score *= ind;

                let mut ind = 1;
                for (k, s) in right_trees.iter().enumerate() {
                    ind = k + 1;
                    if s >= current_tree {
                        break;
                    }
                }
                scenic_score *= ind;

                let mut ind = 1;
                for (k, s) in down_trees.iter().enumerate() {
                    ind = k + 1;
                    if s >= current_tree {
                        break;
                    }
                }
                scenic_score *= ind;

                let mut ind = 1;
                for (k, s) in top_trees.iter().rev().enumerate() {
                    ind = k + 1;
                    if s >= current_tree {
                        break;
                    }
                }
                scenic_score *= ind;
                scores.push(scenic_score);
            }
        }
        format!("{}", scores.iter().max().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}