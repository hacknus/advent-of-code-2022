use crate::io::read_file_lines;
use crate::problem::Problem;
use std::cell::{BorrowMutError, RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub struct DaySixteen {}

#[derive(Debug, Clone, PartialEq)]
pub struct Valve {
    pub name: String,
    pub is_open: bool,
    pub flow: u32,
    pub neighbours: Vec<String>,
}

const MAX_T: u32 = 30;

pub fn make_move(current: Valve, valves: HashMap<String, Valve>, minute: u32) -> u32 {
    let mut current = current.clone();
    let mut valves = valves.clone();
    if minute > MAX_T {
        return 0;
    }
    let visits = valves.iter().map(|(n, v)| v.is_open as u32).sum::<u32>();
    if visits == valves.len() as u32 {
        return 0;
    }

    let max1 = current.neighbours.iter().map(|neighbour| {
        let mut valves2 = valves.clone();
        let neighbour = valves2.get_mut(neighbour).unwrap();
        if !current.is_open && (MAX_T - minute) * current.flow > neighbour.flow {
            valves.get_mut(&current.name).unwrap().is_open = true;
            make_move(neighbour.clone(), valves.clone(), minute + 2) + current.flow * (MAX_T - minute)
        } else {
            0
        }
    }).max().unwrap();
    let max2 = current.neighbours.iter().map(|neighbour| {
        let neighbour = valves.get_mut(neighbour).unwrap();
        make_move(neighbour.clone(), valves.clone(), minute + 1)
    }).max().unwrap();
    max1.max(max2)
}


impl Problem for DaySixteen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut valves = HashMap::new();

        for line in contents.iter() {
            let line_split = line.split(";").map(|s| s.to_string()).collect::<Vec<String>>();
            let valve_split = line_split[0].split("=").map(|s| s.to_string()).collect::<Vec<String>>();
            let flow = valve_split[1].parse::<u32>().unwrap();
            let name = valve_split[0].split(" ").map(|s| s.to_string()).collect::<Vec<String>>()[1].clone();
            let paths = line_split[1].split(" ").map(|s| s.to_string().replace(",", "")).skip(5).collect::<Vec<String>>();
            let valve = Valve { name, is_open: false, flow, neighbours: paths };
            println!("{valve:?}");
            valves.insert(valve.name.clone(), valve);
        }

        // find root
        let mut current = valves.get("AA").unwrap();

        let val = make_move(current.clone(), valves.clone(), 0);
        println!("{:#?}", val);

        format!("{:#?}", current)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        println!("{contents:?}");
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}