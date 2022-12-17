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
    pub neighbours_strings: Vec<String>,
    pub neighbours: Vec<usize>,
}

pub fn get_state(me: usize, already_opened: &u32, valves: &Vec<Valve>, minute: &u32, other_players: &u32) -> usize {
    *already_opened as usize * valves.len()  * 31 * 2 + me * 31 * 2 + *minute as usize * 2 + *other_players as usize
}

const MAX_T: u32 = 30;

pub fn make_move(me: usize, valves: Vec<Valve>, states: &Rc<RefCell<Vec<i32>>>, mut already_opened: u32, minute: u32, other_players: u32) -> u32 {
    let mut valves = valves.clone();
    if minute >= MAX_T {
        return 0;
    }

    let state = get_state(me, &already_opened, &valves, &minute, &other_players);
    if states.borrow()[state] >= 0 {
        return states.borrow()[state] as u32;
    }

    let mut ans = 0;
    let mut max1 = 0;
    let mut max2 = 0;

    max2 = valves[me].neighbours.iter().map(|neighbour| {
        make_move(*neighbour, valves.clone(), &Rc::clone(&states), already_opened, minute + 1, other_players)
    }).max().unwrap();

    if (already_opened & me as u32) == 0 && valves[me].flow > 0 {
        already_opened |= me as u32;
        let temp = valves[me].flow * (MAX_T - minute);
        max1 = temp + valves[me].neighbours.iter().map(|neighbour| {
            make_move(*neighbour, valves.clone(), &Rc::clone(&states), already_opened, minute + 2, other_players)
        }).max().unwrap();
    }

    ans += max1.max(max2);
    states.borrow_mut()[state] = ans as i32;
    ans
}


impl Problem for DaySixteen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut valves = vec![];

        for line in contents.iter() {
            let line_split = line.split(";").map(|s| s.to_string()).collect::<Vec<String>>();
            let valve_split = line_split[0].split("=").map(|s| s.to_string()).collect::<Vec<String>>();
            let flow = valve_split[1].parse::<u32>().unwrap();
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
            println!("{:#?}", valves[i]);
        }

        // find root
        let mut current = valves.iter().position(|v| v.name == "AA").unwrap();

        let num_states = 2_usize.pow(15) * 50 * 30 * 2;
        let mut states = Rc::new(RefCell::new(vec![-1; num_states]));

        let val = make_move(current, valves.clone(), &states, 0, 0, 0);
        println!("{:#?}", val);

        let print_states = states.borrow().iter().map(|s| if *s != -1 { 1 } else { 0 }).sum::<i32>();
        println!("states: {:#?}", print_states);

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