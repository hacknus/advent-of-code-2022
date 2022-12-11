use std::fs;
use std::num::ParseIntError;
use crate::io::read_file_lines;
use crate::problem::Problem;
use num::Integer;

pub struct DayEleven {}

#[derive(Debug)]
pub struct Monkey {
    pub id: usize,
    pub items: Vec<u64>,
    pub counter: u64,
    add: u64,
    mul: u64,
    square: u64,
    pub test_divide: u64,
    test_true: usize,
    test_false: usize,
}

pub fn lcm(vals: &[u64]) -> u64 {
    vals.iter().cloned().reduce(|a, b| { a.lcm(&b) }).unwrap()
}


impl Monkey {
    pub fn new(content: &str) -> Self {
        let lines = content.split("\n").map(|s| s.to_string()).collect::<Vec<String>>();
        let mut id = 0;
        let mut items = vec![];
        let mut test_divide = 1;
        let mut add = 0;
        let mut mul = 1;
        let mut square = 1;
        let mut test_true = 0;
        let mut test_false = 1;
        for line in lines.iter() {
            let fields = line.split(":").map(|s| s.to_string()).collect::<Vec<String>>();
            if fields[0].contains("Monkey") {
                id = fields[0].split(" ").map(|s| s.to_string()).collect::<Vec<String>>()[1].parse::<usize>().unwrap();
            }
            if fields[0].contains("Starting items") {
                items = fields[1].replace(" ", "").split(",").filter_map(|s| s.parse::<u64>().ok()).collect::<Vec<_>>();
            }
            if fields[0].contains("Operation") {
                if fields[1].contains("*") {
                    let r = fields[1].split(" ").map(|s| s.to_string()).collect::<Vec<String>>().last().unwrap().parse::<u64>();
                    match r {
                        Ok(val) => { mul = val }
                        Err(_) => { square = 2 }
                    }
                } else if fields[1].contains("+") {
                    add = fields[1].split(" ").map(|s| s.to_string()).collect::<Vec<String>>().last().unwrap().parse::<u64>().unwrap();
                }
            }
            if fields[0].contains("Test") {
                test_divide = fields[1].split(" ").map(|s| s.to_string()).collect::<Vec<String>>().last().unwrap().parse::<u64>().unwrap();
            }
            if fields[0].contains("true") {
                test_true = fields[1].split(" ").map(|s| s.to_string()).collect::<Vec<String>>().last().unwrap().parse::<usize>().unwrap();
            }
            if fields[0].contains("false") {
                test_false = fields[1].split(" ").map(|s| s.to_string()).collect::<Vec<String>>().last().unwrap().parse::<usize>().unwrap();
            }
        }
        Monkey { id, items, counter: 0, add, mul, square, test_divide, test_true, test_false }
    }

    pub fn inspect(&mut self) {
        self.counter += self.items.len() as u64;
        self.items = self.items.iter().map(|i| ((i.pow(self.square as u32) + self.add) * self.mul) / 3).collect();
    }

    pub fn inspect_worried(&mut self, scalar: u64) {
        self.counter += self.items.len() as u64;
        self.items = self.items.iter().map(|i| {
            let val = (i.pow(self.square as u32) + self.add) * self.mul;
            val % scalar
        }).collect();
    }

    pub fn test(&self) -> Vec<usize> {
        let mut destination = vec![];
        for item in self.items.iter() {
            if item % self.test_divide == 0 {
                destination.push(self.test_true);
            } else {
                destination.push(self.test_false);
            }
        }
        destination
    }
}

impl Problem for DayEleven {
    fn part_one(&self, input: &str) -> String {
        let raw_contents = fs::read_to_string(input)
            .expect("Should have been able to read the file");
        let monkeys_raw = raw_contents.split("\n\n").map(|s| s.to_string()).collect::<Vec<String>>();
        let mut monkeys = vec![];
        for monkey_raw in monkeys_raw.iter() {
            monkeys.push(Monkey::new(monkey_raw));
        }

        for round in 0..20 {
            for i in 0..monkeys.len() {
                monkeys[i].inspect();
                let destinations = monkeys[i].test();
                let temp_items = monkeys[i].items.clone();
                for (item, dest) in temp_items.iter().zip(destinations.iter()) {
                    monkeys[*dest].items.push(item.clone());
                }
                monkeys[i].items = vec![];
            }
        }
        let mut inspection_times = vec![];
        for monkey in monkeys.iter() {
            inspection_times.push(monkey.counter);
        }
        inspection_times.sort();
        inspection_times.reverse();

        format!("{}", inspection_times[0] * inspection_times[1])
    }

    fn part_two(&self, input: &str) -> String {
        let raw_contents = fs::read_to_string(input)
            .expect("Should have been able to read the file");
        let monkeys_raw = raw_contents.split("\n\n").map(|s| s.to_string()).collect::<Vec<String>>();
        let mut monkeys = vec![];
        for monkey_raw in monkeys_raw.iter() {
            monkeys.push(Monkey::new(monkey_raw));
        }

        let mut vals = vec![];
        for monkey in monkeys.iter() {
            vals.push(monkey.test_divide);
        }
        let lcm_scalar = lcm(&vals);

        for round in 0..10000 {
            for i in 0..monkeys.len() {
                monkeys[i].inspect_worried(lcm_scalar);
                let destinations = monkeys[i].test();
                let temp_items = monkeys[i].items.clone();
                for (item, dest) in temp_items.iter().zip(destinations.iter()) {
                    monkeys[*dest].items.push(item.clone());
                }
                monkeys[i].items = vec![];
            }
        }

        let mut inspection_times = vec![];
        for monkey in monkeys.iter() {
            inspection_times.push(monkey.counter);
        }
        inspection_times.sort();
        inspection_times.reverse();

        format!("{}", inspection_times[0] * inspection_times[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}