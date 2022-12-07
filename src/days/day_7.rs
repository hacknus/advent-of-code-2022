use std::borrow::BorrowMut;
use crate::problem::Problem;
use crate::io::{read_file_lines, read_from_csv};
use std::cell::RefCell;
use std::rc::Rc;

pub struct DaySeven {}

#[derive(Debug, Clone, PartialEq)]
pub struct Directory {
    name: String,
    size: u32,
    children: Vec<Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    pub fn new() -> Self {
        Directory { name: "".to_string(), size: 0, children: vec![], parent: None }
    }
    pub fn add_child(&mut self, new_node: Rc<RefCell<Directory>>) {
        self.children.push(new_node);
    }
}



impl Problem for DaySeven {
    fn part_one(&self, input: &str) -> String {
        let mut contents = read_file_lines(input);
        println!("{contents:?}");
        let root = Rc::new(RefCell::new(Directory::new()));
        let mut current = Rc::clone(&root);
        for line in contents.iter() {
            if line.contains("$ cd") {
                let cmd = line.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
                let name = cmd[2].clone();
                if name != ".." {
                    if name == "/" {
                        continue;
                    } else {
                        let mut child = Rc::new(RefCell::new(Directory::new()));
                        let mut mut_current = current.borrow_mut();
                        mut_current.children.push(Rc::clone(&child));
                        let mut mut_child = child.borrow_mut();
                        mut_child.name = name;
                        mut_child.parent = Some(Rc::clone(&current));
                        current = Rc::clone(&child);
                    }
                } else {
                    match &current.borrow_mut().get_mut().parent {
                        None => {}
                        Some(papi) => {
                            current = Rc::clone(&papi);
                        }
                    }
                }
            } else if line.contains("$ ls") {} else if line.contains("dir") {} else {
                let output = line.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
                let size = output[0].parse::<u32>().unwrap();
                current.borrow_mut().size += size;
            }
        }


        let mut total_size = 0;
        format!("{:?}", total_size)
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