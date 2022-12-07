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

    pub fn print_walk(&self, indent: u32) {
        println!("{:i$}>>> dir {n} with size {s} ", "", i = indent as usize, n = self.name, s = self.size);
        if self.children.len() != 0 {
            for mut child in &self.children {
                child.borrow_mut().print_walk(indent + 3);
            }
        }
    }

    pub fn add_size_of_children(&mut self) {
        let mut sum = self.size;
        if self.children.len() != 0 {
            for mut child in &self.children {
                let name = child.borrow_mut().name.clone();
                child.borrow_mut().add_size_of_children();
                let size = child.borrow_mut().size;
                sum += child.borrow_mut().size;
            }
        }
        self.size = sum;
    }

    pub fn walk_puzzle_1(&self) -> u32 {
        let mut sum = 0;
        let threshold = 100000;
        if self.size <= threshold {
            sum = self.size
        } else {
            //println!("not counting {}",self.name);
        }
        if self.children.len() != 0 {
            for mut child in &self.children {
                let name = child.borrow_mut().name.clone();
                let size = child.borrow_mut().size;
                let sum_of_child = child.borrow_mut().walk_puzzle_1();
                sum += sum_of_child;
            }
        }
        sum
    }

    pub fn walk_puzzle_2(&self, threshold: &u32) -> Vec<u32> {
        let mut dirs = vec![];
        if self.size >= *threshold {
            dirs.push(self.size);
        } else {
            //println!("not counting {}",self.name);
        }
        if self.children.len() != 0 {
            for mut child in &self.children {
                let name = child.borrow_mut().name.clone();
                let size = child.borrow_mut().size;
                let mut dirs_children = child.borrow_mut().walk_puzzle_2(&threshold);
                dirs.append(&mut dirs_children);
            }
        }
        dirs
    }
}


impl Problem for DaySeven {
    fn part_one(&self, input: &str) -> String {
        let mut contents = read_file_lines(input);
        let mut root = Rc::new(RefCell::new(Directory::new()));
        let mut current = Rc::clone(&root);
        for line in contents.iter() {
            if line.contains("$ cd") {
                let cmd = line.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
                let name = cmd[2].clone();
                if name != ".." {
                    if name == "/" {
                        root.borrow_mut().name = "/".to_string();
                        continue;
                    } else {
                        let mut child = Rc::new(RefCell::new(Directory::new()));
                        current.borrow_mut().children.push(Rc::clone(&child));
                        let mut mut_child = child.borrow_mut();
                        mut_child.name = name;
                        mut_child.parent = Some(Rc::clone(&current));
                        current = Rc::clone(&child);
                    }
                } else {
                    // really.. clone???
                    let parent = current.borrow_mut().parent.clone();
                    if let Some(daddy) = parent {
                        current = Rc::clone(&daddy);
                    }
                }
            } else if line.contains("$ ls") {} else if line.contains("dir") {} else {
                let output = line.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
                let size = output[0].parse::<u32>().unwrap();
                current.borrow_mut().size += size;
            }
        }
        root.borrow_mut().add_size_of_children();
        root.borrow_mut().print_walk(0);
        let total_size = root.borrow_mut().walk_puzzle_1();
        format!("{:?}", total_size)
    }

    fn part_two(&self, input: &str) -> String {
        let total_disk_size = 70000000;
        let update_required_size = 30000000;
        let mut contents = read_file_lines(input);
        let mut root = Rc::new(RefCell::new(Directory::new()));
        let mut current = Rc::clone(&root);
        for line in contents.iter() {
            if line.contains("$ cd") {
                let cmd = line.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
                let name = cmd[2].clone();
                if name != ".." {
                    if name == "/" {
                        root.borrow_mut().name = "/".to_string();
                        continue;
                    } else {
                        let mut child = Rc::new(RefCell::new(Directory::new()));
                        current.borrow_mut().children.push(Rc::clone(&child));
                        let mut mut_child = child.borrow_mut();
                        mut_child.name = name;
                        mut_child.parent = Some(Rc::clone(&current));
                        current = Rc::clone(&child);
                    }
                } else {
                    // really.. clone???
                    let parent = current.borrow_mut().parent.clone();
                    if let Some(daddy) = parent {
                        current = Rc::clone(&daddy);
                    }
                }
            } else if line.contains("$ ls") {} else if line.contains("dir") {} else {
                let output = line.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
                let size = output[0].parse::<u32>().unwrap();
                current.borrow_mut().size += size;
            }
        }
        root.borrow_mut().add_size_of_children();

        let free = total_disk_size - root.borrow_mut().size;
        let missing = update_required_size - free;


        root.borrow_mut().print_walk(0);
        let mut dirs = root.borrow_mut().walk_puzzle_2(&missing);
        dirs.sort();
        format!("{:?}", dirs[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}