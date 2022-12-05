use crate::problem::Problem;
use crate::io::{read_file_lines, read_from_csv};

pub struct DayFive {}

pub enum FR {
    CRATES,
    CONFIG_LINE,
    MOVES,
}

impl Problem for DayFive {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let conf_line = 9-1;
        let mut state = FR::CRATES;
        let mut stacks_indices = vec![];
        let mut crates = vec![];
        let mut moves = vec![];

        // read cargo crates
        for (line_number, line) in contents.iter().enumerate() {
            match state {
                FR::CRATES => {
                    let mut index = 0;
                    let mut row = vec![];
                    loop {
                        row.push(line[index..index + 3].to_string());
                        index += 4;
                        if index > line.len() {
                            break;
                        }
                    }
                    crates.push(row);
                    if line_number == conf_line - 1 {
                        state = FR::CONFIG_LINE;
                    }
                }
                FR::CONFIG_LINE => {
                    if line == "" {
                        state = FR::MOVES;
                        continue;
                    }
                    stacks_indices = line.split("   ").map(|s| s.to_string()).collect();
                }
                FR::MOVES => {
                    let single_move: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
                    moves.push(vec![single_move[1].clone(), single_move[3].clone(), single_move[5].clone()]);
                }
            }
        }

        let mut stacks = vec![vec![]; stacks_indices.len()];
        for i_str in stacks_indices {
            let i = i_str.split_whitespace().map(|s| s.to_string()).collect::<String>().parse::<usize>().unwrap() - 1;
            for row in &crates {
                for (j, c) in row.iter().enumerate() {
                    if c != "   " && i == j {
                        stacks[i].push(c.to_string());
                    }
                }
            }
        }
        for stack in stacks.iter_mut(){
            *stack = stack.iter().rev().map(|s| s.to_string()).collect::<Vec<String>>();
        }

        // moves
        for m in moves.iter() {
            let mut new_stacks = stacks.clone();
            let amount = m[0].parse::<usize>().unwrap();
            let origin = m[1].parse::<usize>().unwrap() - 1;
            let destination = m[2].parse::<usize>().unwrap() - 1;
            for c in 0..amount {
                let stack = stacks[origin].clone();
                new_stacks[destination].push(stack[stack.len() - 1 - c].clone());
                let len = new_stacks[origin].len();
                new_stacks[origin].remove(len - 1);

            }
            stacks = new_stacks;
        }

        let mut output = "".to_string();
        for stack in stacks {
            output.push_str(stack[stack.len()-1].replace("[","").replace("]","").as_str());
        }

        format!("{}", output)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let conf_line = 9-1;
        let mut state = FR::CRATES;
        let mut stacks_indices = vec![];
        let mut crates = vec![];
        let mut moves = vec![];

        // read cargo crates
        for (line_number, line) in contents.iter().enumerate() {
            match state {
                FR::CRATES => {
                    let mut index = 0;
                    let mut row = vec![];
                    loop {
                        row.push(line[index..index + 3].to_string());
                        index += 4;
                        if index > line.len() {
                            break;
                        }
                    }
                    crates.push(row);
                    if line_number == conf_line - 1 {
                        state = FR::CONFIG_LINE;
                    }
                }
                FR::CONFIG_LINE => {
                    if line == "" {
                        state = FR::MOVES;
                        continue;
                    }
                    stacks_indices = line.split("   ").map(|s| s.to_string()).collect();
                }
                FR::MOVES => {
                    let single_move: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
                    moves.push(vec![single_move[1].clone(), single_move[3].clone(), single_move[5].clone()]);
                }
            }
        }

        let mut stacks = vec![vec![]; stacks_indices.len()];
        for i_str in stacks_indices {
            let i = i_str.split_whitespace().map(|s| s.to_string()).collect::<String>().parse::<usize>().unwrap() - 1;
            for row in &crates {
                for (j, c) in row.iter().enumerate() {
                    if c != "   " && i == j {
                        stacks[i].push(c.to_string());
                    }
                }
            }
        }
        for stack in stacks.iter_mut(){
            *stack = stack.iter().rev().map(|s| s.to_string()).collect::<Vec<String>>();
        }

        // moves
        for m in moves.iter() {
            let mut new_stacks = stacks.clone();
            let amount = m[0].parse::<usize>().unwrap();
            let origin = m[1].parse::<usize>().unwrap() - 1;
            let destination = m[2].parse::<usize>().unwrap() - 1;
            let mut package = vec![];
            for c in 0..amount {
                let stack = stacks[origin].clone();
                package.push(stack[stack.len() - 1 - c].clone());
                let len = new_stacks[origin].len();
                new_stacks[origin].remove(len - 1);

            }

            for p in package.iter().rev() {
                new_stacks[destination].push(p.clone());
            }

            stacks = new_stacks;
        }

        let mut output = "".to_string();
        for stack in stacks {
            output.push_str(stack[stack.len()-1].replace("[","").replace("]","").as_str());
        }

        format!("{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}