use crate::io::read_file_lines;
use crate::problem::Problem;
use std::cell::{BorrowMutError, RefCell, RefMut};
use std::rc::Rc;

pub struct DaySixteen {}

#[derive(Debug, PartialEq)]
pub struct Valve {
    pub name: String,
    pub flow: u32,
    pub neighbours_names: Vec<String>,
    pub neighbours: Vec<Rc<RefCell<Valve>>>,
}

impl Valve {
    pub fn populate(&mut self, valves: &Vec<Rc<RefCell<Valve>>>) {
        for neighbour_name in self.neighbours_names.iter() {
            println!("looking at neighbours: {neighbour_name}");
            for valve in valves.iter() {
                match valve.try_borrow_mut() {
                    Ok(v) => {
                        if v.name == *neighbour_name {
                            println!("adding to neighbours!");
                            self.neighbours.push(Rc::clone(&valve));
                        }
                    }
                    Err(_) => {}
                }
            }
        }
    }
}

pub struct Path {
    pub this: Rc<RefCell<Valve>>,
    pub next: Option<Rc<RefCell<Path>>>,
}

impl Path {
    pub fn build(&mut self, path_length: u32) {
        if path_length > 30 {
            return;
        }
        println!("building {path_length}");
        for neighbour in self.this.borrow_mut().neighbours.iter() {
            let mut path = Rc::new(RefCell::new(Path { this: Rc::clone(neighbour), next: None }));
            path.borrow_mut().build(path_length + 1);
            self.next = Some(Rc::clone(&path));
        }
    }

    pub fn walk(&mut self) {}
}

impl Problem for DaySixteen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut valves: Vec<Rc<RefCell<Valve>>> = vec![];

        for line in contents.iter() {
            let line_split = line.split(";").map(|s| s.to_string()).collect::<Vec<String>>();
            let valve_split = line_split[0].split("=").map(|s| s.to_string()).collect::<Vec<String>>();
            let flow = valve_split[1].parse::<u32>().unwrap();
            let name = valve_split[0].split(" ").map(|s| s.to_string()).collect::<Vec<String>>()[1].clone();
            let paths = line_split[1].split(" ").map(|s| s.to_string().replace(",", "")).skip(5).collect::<Vec<String>>();
            let valve = Valve { name, flow, neighbours_names: paths, neighbours: vec![] };
            println!("{valve:?}");
            valves.push(Rc::new(RefCell::new(valve)));
        }

        for valve in valves.iter() {
            println!("looking at: {}", valve.borrow_mut().name);
            valve.borrow_mut().populate(&valves);
        }
        println!("finding root");
        // find root
        let mut root = Rc::new(RefCell::new(Valve {
            name: "".to_string(),
            flow: 0,
            neighbours_names: vec![],
            neighbours: vec![],
        }));
        for valve in valves.iter() {
            if valve.borrow_mut().name == "AA" {
                root = Rc::clone(valve);
            }
        }

        let mut path = Path { this: root, next: None };

        path.build(0);

        format!("{:#?}", 0)
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