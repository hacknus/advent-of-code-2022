use std::collections::{HashMap, HashSet};
use std::ops::Index;
use itertools::Itertools;
use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayTwentyFour {}

#[derive(Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub struct Blizzard {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Blizzard {
    pub fn move_it(&mut self, width: &usize, height: &usize) {
        match self.dir {
            Direction::UP => {
                if self.y > 1 {
                    self.y -= 1;
                } else {
                    self.y = height - 2;
                }
            }
            Direction::DOWN => {
                if self.y < *height - 2 {
                    self.y += 1;
                } else {
                    self.y = 1;
                }
            }
            Direction::LEFT => {
                if self.x > 1 {
                    self.x -= 1;
                } else {
                    self.x = width - 2;
                }
            }
            Direction::RIGHT => {
                if self.x < *width - 2 {
                    self.x += 1;
                } else {
                    self.x = 1;
                }
            }
        }
    }
}

pub fn boundary_check(field: &[usize; 2], width: &usize, height: &usize) -> bool {
    if field[0] < 1 || field[1] < 1 || field[0] > width - 2 || field[1] > height - 2 {
        if *field != [1, 0] {
            // println!(" boundary check feiled: {field:?}");
            return false;
        }
    }
    true
}

impl Problem for DayTwentyFour {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut blizzards = vec![];
        let mut width = 0;
        let mut height = contents.len();
        for (y, line) in contents.iter().enumerate() {
            width = line.len();
            for (x, field) in line.char_indices() {
                if field == '^' {
                    blizzards.push(Blizzard {
                        x,
                        y,
                        dir: Direction::UP,
                    });
                } else if field == 'v' {
                    blizzards.push(Blizzard {
                        x,
                        y,
                        dir: Direction::DOWN,
                    });
                } else if field == '>' {
                    blizzards.push(Blizzard {
                        x,
                        y,
                        dir: Direction::RIGHT,
                    });
                } else if field == '<' {
                    blizzards.push(Blizzard {
                        x,
                        y,
                        dir: Direction::LEFT,
                    });
                }
            }
        }

        let mut shell = HashSet::new();
        shell.insert([1, 0]);
        let mut counter = 0;
        'main_loop: loop {
            for blizzard in blizzards.iter_mut() {
                blizzard.move_it(&width, &height);
            }
            let mut new_shell = shell.clone();
            for field in shell.iter() {
                // println!("looking at {field:?}");
                // are we there yet?
                if field[0] == width - 2 && field[1] == height - 2 {
                    println!("found {field:?}");
                    break 'main_loop;
                }

                // check if blizzard is on fields next to it
                let mut north = true;
                let mut south = true;
                let mut east = true;
                let mut west = true;
                for blizzard in blizzards.iter() {
                    if field[1] + 1 > height - 2 || (field[0] == blizzard.x && field[1] + 1 == blizzard.y) {
                        south = false;
                    }
                    if field[1] < 1 || (field[0] == blizzard.x && field[1] - 1 == blizzard.y) {
                        north = false;
                    }
                    if field[0] + 1 > width - 2 || (field[0] + 1 == blizzard.x && field[1] == blizzard.y) {
                        east = false;
                    }
                    if field[0] < 1 || (field[0] - 1 == blizzard.x && field[1] == blizzard.y) {
                        west = false;
                    }
                    if field[0] == blizzard.x && field[1] == blizzard.y {
                        new_shell.remove(field);
                        // println!("removing {field:?} because of blizzard {blizzard:?}");
                    }
                }

                if north {
                    if boundary_check(&[field[0], field[1] - 1], &width, &height) {
                        // println!("moving north from {:?} to  {:?}", field, [field[0], field[1] - 1]);
                        new_shell.insert([field[0], field[1] - 1]);
                    }
                }
                if south {
                    if boundary_check(&[field[0], field[1] + 1], &width, &height) {
                        // println!("moving south from {:?} to  {:?}", field, [field[0], field[1] + 1]);
                        new_shell.insert([field[0], field[1] + 1]);
                    }
                }
                if east {
                    if boundary_check(&[field[0] + 1, field[1]], &width, &height) {
                        // println!("moving east from {:?} to  {:?}", field, [field[0] + 1, field[1]]);
                        new_shell.insert([field[0] + 1, field[1]]);
                    }
                }
                if west {
                    if boundary_check(&[field[0] - 1, field[1]], &width, &height) {
                        // println!("moving west from {:?} to  {:?}", field, [field[0] - 1, field[1]]);
                        new_shell.insert([field[0] - 1, field[1]]);
                    }
                }
            }
            counter += 1;
            shell = new_shell;
        }


        format!("{}", counter + 1)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut blizzards = vec![];
        let mut width = 0;
        let mut height = contents.len();
        for (y, line) in contents.iter().enumerate() {
            width = line.len();
            for (x, field) in line.char_indices() {
                if field == '^' {
                    blizzards.push(Blizzard {
                        x,
                        y,
                        dir: Direction::UP,
                    });
                } else if field == 'v' {
                    blizzards.push(Blizzard {
                        x,
                        y,
                        dir: Direction::DOWN,
                    });
                } else if field == '>' {
                    blizzards.push(Blizzard {
                        x,
                        y,
                        dir: Direction::RIGHT,
                    });
                } else if field == '<' {
                    blizzards.push(Blizzard {
                        x,
                        y,
                        dir: Direction::LEFT,
                    });
                }
            }
        }

        let mut shell = HashSet::new();
        shell.insert([1, 0]);
        let mut counter = 0;
        'main_loop: loop {
            for blizzard in blizzards.iter_mut() {
                blizzard.move_it(&width, &height);
            }
            let mut new_shell = shell.clone();
            for field in shell.iter() {
                // println!("looking at {field:?}");
                // are we there yet?
                if field[0] == width - 2 && field[1] == height - 2 {
                    println!("found {field:?}");
                    break 'main_loop;
                }

                // check if blizzard is on fields next to it
                let mut north = true;
                let mut south = true;
                let mut east = true;
                let mut west = true;
                for blizzard in blizzards.iter() {
                    if field[1] + 1 > height - 2 || (field[0] == blizzard.x && field[1] + 1 == blizzard.y) {
                        south = false;
                    }
                    if field[1] < 1 || (field[0] == blizzard.x && field[1] - 1 == blizzard.y) {
                        north = false;
                    }
                    if field[0] + 1 > width - 2 || (field[0] + 1 == blizzard.x && field[1] == blizzard.y) {
                        east = false;
                    }
                    if field[0] < 1 || (field[0] - 1 == blizzard.x && field[1] == blizzard.y) {
                        west = false;
                    }
                    if field[0] == blizzard.x && field[1] == blizzard.y {
                        new_shell.remove(field);
                        // println!("removing {field:?} because of blizzard {blizzard:?}");
                    }
                }

                if north {
                    if boundary_check(&[field[0], field[1] - 1], &width, &height) {
                        // println!("moving north from {:?} to  {:?}", field, [field[0], field[1] - 1]);
                        new_shell.insert([field[0], field[1] - 1]);
                    }
                }
                if south {
                    if boundary_check(&[field[0], field[1] + 1], &width, &height) {
                        // println!("moving south from {:?} to  {:?}", field, [field[0], field[1] + 1]);
                        new_shell.insert([field[0], field[1] + 1]);
                    }
                }
                if east {
                    if boundary_check(&[field[0] + 1, field[1]], &width, &height) {
                        // println!("moving east from {:?} to  {:?}", field, [field[0] + 1, field[1]]);
                        new_shell.insert([field[0] + 1, field[1]]);
                    }
                }
                if west {
                    if boundary_check(&[field[0] - 1, field[1]], &width, &height) {
                        // println!("moving west from {:?} to  {:?}", field, [field[0] - 1, field[1]]);
                        new_shell.insert([field[0] - 1, field[1]]);
                    }
                }
            }
            counter += 1;
            shell = new_shell;
        }

        counter += 1;
        let mut shell = HashSet::new();
        shell.insert([width - 2, height - 1]);
        'main_loop: loop {
            for blizzard in blizzards.iter_mut() {
                blizzard.move_it(&width, &height);
            }
            let mut new_shell = shell.clone();
            for field in shell.iter() {
                // println!("looking at {field:?}");
                // are we there yet?
                if field[0] == 1 && field[1] == 1 {
                    println!("found {field:?}");
                    break 'main_loop;
                }

                // check if blizzard is on fields next to it
                let mut north = true;
                let mut south = true;
                let mut east = true;
                let mut west = true;
                for blizzard in blizzards.iter() {
                    if field[1] + 1 > height - 2 || (field[0] == blizzard.x && field[1] + 1 == blizzard.y) {
                        south = false;
                    }
                    if field[1] < 1 || (field[0] == blizzard.x && field[1] - 1 == blizzard.y) {
                        north = false;
                    }
                    if field[0] + 1 > width - 2 || (field[0] + 1 == blizzard.x && field[1] == blizzard.y) {
                        east = false;
                    }
                    if field[0] < 1 || (field[0] - 1 == blizzard.x && field[1] == blizzard.y) {
                        west = false;
                    }
                    if field[0] == blizzard.x && field[1] == blizzard.y {
                        new_shell.remove(field);
                        // println!("removing {field:?} because of blizzard {blizzard:?}");
                    }
                }

                if north {
                    if boundary_check(&[field[0], field[1] - 1], &width, &height) {
                        // println!("moving north from {:?} to  {:?}", field, [field[0], field[1] - 1]);
                        new_shell.insert([field[0], field[1] - 1]);
                    }
                }
                if south {
                    if boundary_check(&[field[0], field[1] + 1], &width, &height) {
                        // println!("moving south from {:?} to  {:?}", field, [field[0], field[1] + 1]);
                        new_shell.insert([field[0], field[1] + 1]);
                    }
                }
                if east {
                    if boundary_check(&[field[0] + 1, field[1]], &width, &height) {
                        // println!("moving east from {:?} to  {:?}", field, [field[0] + 1, field[1]]);
                        new_shell.insert([field[0] + 1, field[1]]);
                    }
                }
                if west {
                    if boundary_check(&[field[0] - 1, field[1]], &width, &height) {
                        // println!("moving west from {:?} to  {:?}", field, [field[0] - 1, field[1]]);
                        new_shell.insert([field[0] - 1, field[1]]);
                    }
                }
            }
            counter += 1;
            shell = new_shell;
        }

        counter += 1;
        let mut shell = HashSet::new();
        shell.insert([1, 0]);
        'main_loop: loop {
            for blizzard in blizzards.iter_mut() {
                blizzard.move_it(&width, &height);
            }
            let mut new_shell = shell.clone();
            for field in shell.iter() {
                // println!("looking at {field:?}");
                // are we there yet?
                if field[0] == width - 2 && field[1] == height - 2 {
                    println!("found {field:?}");
                    break 'main_loop;
                }

                // check if blizzard is on fields next to it
                let mut north = true;
                let mut south = true;
                let mut east = true;
                let mut west = true;
                for blizzard in blizzards.iter() {
                    if field[1] + 1 > height - 2 || (field[0] == blizzard.x && field[1] + 1 == blizzard.y) {
                        south = false;
                    }
                    if field[1] < 1 || (field[0] == blizzard.x && field[1] - 1 == blizzard.y) {
                        north = false;
                    }
                    if field[0] + 1 > width - 2 || (field[0] + 1 == blizzard.x && field[1] == blizzard.y) {
                        east = false;
                    }
                    if field[0] < 1 || (field[0] - 1 == blizzard.x && field[1] == blizzard.y) {
                        west = false;
                    }
                    if field[0] == blizzard.x && field[1] == blizzard.y {
                        new_shell.remove(field);
                        // println!("removing {field:?} because of blizzard {blizzard:?}");
                    }
                }

                if north {
                    if boundary_check(&[field[0], field[1] - 1], &width, &height) {
                        // println!("moving north from {:?} to  {:?}", field, [field[0], field[1] - 1]);
                        new_shell.insert([field[0], field[1] - 1]);
                    }
                }
                if south {
                    if boundary_check(&[field[0], field[1] + 1], &width, &height) {
                        // println!("moving south from {:?} to  {:?}", field, [field[0], field[1] + 1]);
                        new_shell.insert([field[0], field[1] + 1]);
                    }
                }
                if east {
                    if boundary_check(&[field[0] + 1, field[1]], &width, &height) {
                        // println!("moving east from {:?} to  {:?}", field, [field[0] + 1, field[1]]);
                        new_shell.insert([field[0] + 1, field[1]]);
                    }
                }
                if west {
                    if boundary_check(&[field[0] - 1, field[1]], &width, &height) {
                        // println!("moving west from {:?} to  {:?}", field, [field[0] - 1, field[1]]);
                        new_shell.insert([field[0] - 1, field[1]]);
                    }
                }
            }
            counter += 1;
            shell = new_shell;
        }

        format!("{}", counter + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}