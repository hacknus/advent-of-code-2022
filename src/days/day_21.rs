use std::collections::HashMap;
use std::ops::Index;
use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayTwentyOne {}


pub fn get_monkey_val_without_human(monkey: String, monkeys_string: &HashMap<String, String>) -> Option<i64> {
    let split_monkey = monkeys_string.get(&monkey).unwrap().split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
    if split_monkey.len() > 1 {
        let mut monkey1 = 0;
        let mut monkey2 = 0;
        if split_monkey[0].as_str() != "humn" {
            match get_monkey_val_without_human(split_monkey[0].clone(), monkeys_string) {
                None => { return None; }
                Some(val) => { monkey1 = val }
            }
        } else {
            return None;
        }
        if split_monkey[2].as_str() != "humn" {
            match get_monkey_val_without_human(split_monkey[2].clone(), monkeys_string) {
                None => { return None; }
                Some(val) => { monkey2 = val }
            }
        } else {
            return None;
        }
        match split_monkey[1].as_str() {
            "+" => {
                Some(monkey1 + monkey2)
            }
            "-" => {
                Some(monkey1 - monkey2)
            }
            "*" => {
                Some(monkey1 * monkey2)
            }
            "/" => {
                Some(monkey1 / monkey2)
            }
            _ => {
                println!("invalid operator found!");
                Some(0)
            }
        }
    } else {
        Some(split_monkey[0].parse::<i64>().unwrap())
    }
}

pub fn propagate_back(monkey: String, monkeys_string: &HashMap<String, String>, previous: i64) -> i64 {
    let split_monkey = monkeys_string.get(&monkey).unwrap().split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
    if monkey == "humn".to_string() {
        return previous;
    }
    if split_monkey.len() > 1 {
        let mut monkey1 = 0;
        let mut monkey2 = 0;
        if split_monkey[0].as_str() != "humn" {
            match get_monkey_val_without_human(split_monkey[0].clone(), monkeys_string) {
                None => {}
                Some(val) => {
                    monkey1 = val;
                    return match split_monkey[1].as_str() {
                        "+" => {
                            propagate_back(split_monkey[2].clone(), monkeys_string, previous - monkey1)
                        }
                        "-" => {
                            propagate_back(split_monkey[2].clone(), monkeys_string, monkey1 - previous)
                        }
                        "*" => {
                            propagate_back(split_monkey[2].clone(), monkeys_string, previous / monkey1)
                        }
                        "/" => {
                            propagate_back(split_monkey[2].clone(), monkeys_string, monkey1 / previous)
                        }
                        _ => {
                            println!("invalid operator found!");
                            0
                        }
                    };
                }
            }
        }
        if split_monkey[2].as_str() != "humn" {
            match get_monkey_val_without_human(split_monkey[2].clone(), monkeys_string) {
                None => {}
                Some(val) => {
                    monkey2 = val;
                    return match split_monkey[1].as_str() {
                        "+" => {
                            propagate_back(split_monkey[0].clone(), monkeys_string, previous - monkey2)
                        }
                        "-" => {
                            propagate_back(split_monkey[0].clone(), monkeys_string, previous + monkey2)
                        }
                        "*" => {
                            propagate_back(split_monkey[0].clone(), monkeys_string, previous / monkey2)
                        }
                        "/" => {
                            propagate_back(split_monkey[0].clone(), monkeys_string, previous * monkey2)
                        }
                        _ => {
                            println!("invalid operator found!");
                            0
                        }
                    };
                }
            }
        }
        return previous;
    } else {
        split_monkey[0].parse::<i64>().unwrap()
    }
}

pub fn get_monkey_val(monkey: String, monkeys_string: &HashMap<String, String>) -> i64 {
    let split_monkey = monkeys_string.get(&monkey).unwrap().split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
    if split_monkey.len() > 1 {
        let monkey1 = get_monkey_val(split_monkey[0].clone(), monkeys_string);
        let monkey2 = get_monkey_val(split_monkey[2].clone(), monkeys_string);
        match split_monkey[1].as_str() {
            "+" => {
                monkey1 + monkey2
            }
            "-" => {
                monkey1 - monkey2
            }
            "*" => {
                monkey1 * monkey2
            }
            "/" => {
                monkey1 / monkey2
            }
            _ => {
                println!("invalid operator found!");
                0
            }
        }
    } else {
        split_monkey[0].parse::<i64>().unwrap()
    }
}

impl Problem for DayTwentyOne {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut monkeys_string = HashMap::new();
        for line in contents.iter() {
            let split_line = line.split(": ").map(|s| s.to_string()).collect::<Vec<String>>();
            monkeys_string.insert(split_line[0].clone(), split_line[1].clone());
        }

        let val = get_monkey_val("root".to_string(), &monkeys_string);

        format!("{}", val)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut monkeys_string = HashMap::new();
        for line in contents.iter() {
            let split_line = line.split(": ").map(|s| s.to_string()).collect::<Vec<String>>();
            monkeys_string.insert(split_line[0].clone(), split_line[1].clone());
        }

        let root_comparison = monkeys_string.get("root").unwrap().split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
        let monkey_left = root_comparison[0].clone();
        let monkey_right = root_comparison[2].clone();

        let mut val1 = 0;
        let mut val2 = 0;
        match get_monkey_val_without_human(monkey_right.clone(), &monkeys_string) {
            None => {}
            Some(to_compare) => {
                monkeys_string.insert("compare".to_string(), to_compare.to_string());
                val1 = propagate_back(monkey_left.clone(), &monkeys_string, to_compare)
            }
        }
        match get_monkey_val_without_human(monkey_left.clone(), &monkeys_string) {
            None => {}
            Some(to_compare) => {
                monkeys_string.insert("compare".to_string(), to_compare.to_string());
                val2 = propagate_back(monkey_right, &monkeys_string, to_compare)
            }
        }


        format!("{}", val1+val2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}