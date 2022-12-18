use crate::io::read_file_lines;
use crate::problem::Problem;
use std::cell::{BorrowMutError, RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::Instant;

pub struct DaySixteen {}

#[derive(Debug, Clone, PartialEq)]
pub struct Valve {
    pub name: String,
    pub is_open: bool,
    pub flow: u64,
    pub neighbours_strings: Vec<String>,
    pub neighbours: Vec<usize>,
}


pub fn make_move(source: usize, me: usize, valves: &Vec<Valve>, states: &mut HashMap<(usize, u64, usize, u64, bool), u64>, mut already_opened: u64, minute: u64, max_t: u64, other_players: bool) -> u64 {
    if minute >= max_t {
        if other_players {
            return make_move(source, source, valves, states, already_opened, 0, max_t, !other_players);
        }
        return 0;
    }

    let state = (me, already_opened, valves.len(), minute, other_players);
    match states.get(&state) {
        None => {}
        Some(val) => {
            return *val;
        }
    }

    let mut ans = 0;
    let mut max1 = 0;
    let mut max2 = 0;

    max2 = valves[me].neighbours.iter().map(|neighbour| {
        make_move(source, *neighbour, valves, states, already_opened, minute + 1, max_t, other_players)
    }).max().unwrap();

    if (already_opened & (1_u64 << me)) == 0 && valves[me].flow > 0 {
        already_opened |= (1_u64 << me);
        let temp = valves[me].flow * (max_t - minute - 1);
        max1 = temp + valves[me].neighbours.iter().map(|neighbour| {
            make_move(source,*neighbour, valves, states, already_opened, minute + 2, max_t, other_players)
        }).max().unwrap();
    }

    ans += max1.max(max2);
    states.insert(state, ans);
    ans
}


impl Problem for DaySixteen {
    fn part_one(&self, input: &str) -> String {

        let contents = read_file_lines(input);

        let mut valves = vec![];

        for line in contents.iter() {
            let line_split = line.split(";").map(|s| s.to_string()).collect::<Vec<String>>();
            let valve_split = line_split[0].split("=").map(|s| s.to_string()).collect::<Vec<String>>();
            let flow = valve_split[1].parse::<u64>().unwrap();
            let name = valve_split[0].split(" ").map(|s| s.to_string()).collect::<Vec<String>>()[1].clone();
            let paths = line_split[1].split(" ").map(|s| s.to_string().replace(",", "")).skip(5).collect::<Vec<String>>();
            let valve = Valve { name, is_open: false, flow, neighbours_strings: paths, neighbours: vec![] };
            valves.push(valve);
        }
        for i in 0..valves.len() {
            let valve = valves[i].clone();
            for neighbour in valve.neighbours_strings.iter() {
                let index = valves.iter().position(|v| &v.name == neighbour).unwrap();
                valves[i].neighbours.push(index);
            }
        }

        // find root
        let mut current = valves.iter().position(|v| v.name == "AA").unwrap();

        let mut states = HashMap::new();

        let start = Instant::now();
        let val = make_move(current, current, &valves, &mut states, 0, 0, 30, false);
        println!("calculation time: {:?}", start.elapsed());

        format!("{:#?}", val)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut valves = vec![];

        for line in contents.iter() {
            let line_split = line.split(";").map(|s| s.to_string()).collect::<Vec<String>>();
            let valve_split = line_split[0].split("=").map(|s| s.to_string()).collect::<Vec<String>>();
            let flow = valve_split[1].parse::<u64>().unwrap();
            let name = valve_split[0].split(" ").map(|s| s.to_string()).collect::<Vec<String>>()[1].clone();
            let paths = line_split[1].split(" ").map(|s| s.to_string().replace(",", "")).skip(5).collect::<Vec<String>>();
            let valve = Valve { name, is_open: false, flow, neighbours_strings: paths, neighbours: vec![] };
            valves.push(valve);
        }
        for i in 0..valves.len() {
            let valve = valves[i].clone();
            for neighbour in valve.neighbours_strings.iter() {
                let index = valves.iter().position(|v| &v.name == neighbour).unwrap();
                valves[i].neighbours.push(index);
            }
        }

        // find root
        let mut current = valves.iter().position(|v| v.name == "AA").unwrap();

        let num_states = 2_usize.pow(valves.len() as u32) * valves.len() * 31 * 2;

        let mut states = HashMap::new();

        let start = Instant::now();
        let val = make_move(current, current, &valves, &mut states, 0, 0, 26, true);
        println!("calculation time: {:?}", start.elapsed());

        format!("{:#?}", val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}