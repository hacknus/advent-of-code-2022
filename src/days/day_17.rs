use std::fs;
use itertools::Itertools;
use crate::problem::Problem;

pub struct DaySeventeen {}

#[derive(Clone, Debug)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub id: u32,
    pub content: Vec<Vec<i32>>,
}

impl Object {
    pub fn one() -> Self {
        Object {
            x: 0,
            y: 0,
            id: 1,
            content: vec![
                vec![1, 1, 1, 1]
            ],
        }
    }

    pub fn two() -> Self {
        Object {
            x: 0,
            y: 0,
            id: 2,
            content: vec![
                vec![0, 1, 0],
                vec![1, 1, 1],
                vec![0, 1, 0],
            ],
        }
    }

    pub fn three() -> Self {
        Object {
            x: 0,
            y: 0,
            id: 3,
            content: vec![
                vec![0, 0, 1],
                vec![0, 0, 1],
                vec![1, 1, 1],
            ],
        }
    }

    pub fn four() -> Self {
        Object {
            x: 0,
            y: 0,
            id: 4,
            content: vec![
                vec![1],
                vec![1],
                vec![1],
                vec![1],
            ],
        }
    }

    pub fn five() -> Self {
        Object {
            x: 0,
            y: 0,
            id: 5,
            content: vec![
                vec![1, 1],
                vec![1, 1],
            ],
        }
    }

    pub fn move_it(&mut self, run: &Move, field: &mut Vec<Vec<i32>>) -> bool {
        let mut x = self.x;
        let mut y = self.y;
        match run {
            Move::Left => {
                if self.x > 0 {
                    x -= 1;
                }
            }
            Move::Right => {
                if self.x + (self.content[0].len() as i32) < 7 {
                    x += 1;
                }
            }
            Move::None => {}
        }
        let mut valid = true;
        for yi in 0..self.content.len() {
            for xi in 0..self.content[0].len() {
                if field[xi + x as usize][y as usize + yi] + self.content[self.content.len() - 1 - yi][xi] > 1
                {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            self.x = x;
        } else {
            x = self.x;
        }

        // falling
        y -= 1;
        let mut valid = true;
        for yi in 0..self.content.len() {
            for xi in 0..self.content[0].len() {
                if field[xi + x as usize][y as usize + yi] + self.content[self.content.len() - 1 - yi][xi] > 1
                {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            self.y = y;
        } else {
            for yi in 0..self.content.len() {
                for xi in 0..self.content[0].len() {
                    field[xi + self.x as usize][self.y as usize + yi] += self.content[self.content.len() - 1 - yi][xi];
                }
            }
            return false;
        }
        return true;
    }
}

pub fn print_grid(object: &Object, field: &Vec<Vec<i32>>) {
    println!("field:");
    for y in (0..field[0].len()).rev() {
        let mut row = "".to_string();
        for x in 0..field.len() {
            if field[x][y] == 1 {
                row += "#";
            } else if object.x <= x as i32
                && object.y <= y as i32
                && y <= object.y as usize + object.content.len() - 1
                && x <= object.x as usize + object.content[0].len() - 1 {
                if object.content[object.content.len() - 1 - (y - object.y as usize)][x - object.x as usize] == 1 {
                    row += "@";
                } else {
                    row += ".";
                }
            } else {
                row += "."
            }
        }
        println!("{row}");
    }
}

#[derive(Debug)]
pub enum Move {
    Left,
    Right,
    None,
}

impl Problem for DaySeventeen {
    fn part_one(&self, input: &str) -> String {
        let moves = fs::read_to_string(input)
            .expect("Should have been able to read the file").chars().map(|s| {
            if s == '<' {
                Move::Left
            } else if s == '>' {
                Move::Right
            } else {
                Move::None
            }
        }).collect::<Vec<Move>>();

        let objects = [Object::one(), Object::two(), Object::three(), Object::four(), Object::five()];

        let mut object_index = 0;

        let mut field = vec![vec![1]; 7];

        let mut object = objects[object_index].clone();
        object.x = 2;
        object.y = field[0].len() as i32 + 3;
        for i in 0..field.len() {
            field[i].push(0);
            field[i].push(0);
            field[i].push(0);
            field[i].push(0);
        }

        let mut run_counter = 0;
        'mainloop: loop {
            for run in moves.iter() {
                if !object.move_it(run, &mut field) {
                    run_counter += 1;
                    if run_counter == 2022 {
                        print_grid(&object, &field);
                        break 'mainloop;
                    }
                    object_index += 1;
                    if object_index >= objects.len() {
                        object_index = 0;
                    }
                    object = objects[object_index].clone();
                    object.x = 2;
                    let max_altitude = field.iter().map(|s| s.iter().positions(|si| *si == 1).max().unwrap()).max().unwrap();
                    object.y = max_altitude as i32 + 3 + 1;
                    let delta = (max_altitude as i32 + 3 + 1 + object.content.len() as i32) - field[0].len() as i32;
                    for i in 0..field.len() {
                        if delta > 0 {
                            for _ in 0..(delta as usize) {
                                field[i].push(0);
                            }
                        }
                    }
                }
            }
        }

        let max_altitude = field.iter().map(|s| s.iter().positions(|si| *si == 1).max().unwrap()).max().unwrap();

        format!("{}", max_altitude)
    }

    fn part_two(&self, input: &str) -> String {
        let moves = fs::read_to_string(input)
            .expect("Should have been able to read the file").chars().map(|s| {
            if s == '<' {
                Move::Left
            } else if s == '>' {
                Move::Right
            } else {
                Move::None
            }
        }).collect::<Vec<Move>>();

        let objects = [Object::one(), Object::two(), Object::three(), Object::four(), Object::five()];

        let mut object_index = 0;

        let mut field = vec![vec![1]; 7];

        let mut object = objects[object_index].clone();
        object.x = 2;
        object.y = field[0].len() as i32 + 3;
        for i in 0..field.len() {
            field[i].push(0);
            field[i].push(0);
            field[i].push(0);
            field[i].push(0);
        }

        let mut run_counter = 0;
        let mut rocks_in_pattern = 0;
        let mut max_altitude1 = 0;
        let mut max_altitude2 = 0;
        let mut rock_offset = 0;
        let mut to_go = 0;
        let mut cycles = 0;
        let mut compare_i = 0;
        let mut compare_cycle = 0;
        let mut compare_x = 0;
        'mainloop: loop {
            cycles += 1;
            for (i, run) in moves.iter().enumerate() {
                //print_grid(&object, &field);
                if !object.move_it(run, &mut field) {
                    run_counter += 1;

                    if object.id == 1 && run_counter != 0 && to_go == 0 && cycles > 2 && rock_offset == 0 {
                        rock_offset = run_counter;
                        compare_i = i;
                        compare_cycle = cycles;
                        compare_x = object.x;
                        max_altitude1 = field.iter().map(|s| s.iter().positions(|si| *si == 1).max().unwrap()).max().unwrap();
                    } else if object.id == 1 && i == compare_i && run_counter != 0 && to_go == 0 && object.x == compare_x && rock_offset != 0 && cycles > compare_cycle {
                        print_grid(&object, &field);
                        rocks_in_pattern = run_counter - rock_offset;
                        to_go = run_counter as i64 + (1000000000000 - rock_offset) % rocks_in_pattern as i64;
                        max_altitude2 = field.iter().map(|s| s.iter().positions(|si| *si == 1).max().unwrap()).max().unwrap();
                    }
                    if run_counter as i64 >= to_go && to_go != 0 {
                        break 'mainloop;
                    }
                    object_index += 1;
                    if object_index >= objects.len() {
                        object_index = 0;
                    }
                    object = objects[object_index].clone();
                    object.x = 2;
                    let max_altitude = field.iter().map(|s| s.iter().positions(|si| *si == 1).max().unwrap()).max().unwrap();
                    object.y = max_altitude as i32 + 3 + 1;
                    let delta = (max_altitude as i32 + 3 + 1 + object.content.len() as i32) - field[0].len() as i32;
                    for i in 0..field.len() {
                        if delta > 0 {
                            for _ in 0..(delta as usize) {
                                field[i].push(0);
                            }
                        }
                    }
                }
            }
        }

        print_grid(&object, &field);

        let max_altitude = field.iter().map(|s| s.iter().positions(|si| *si == 1).max().unwrap()).max().unwrap();

        format!("{}", max_altitude1 as i64 + (max_altitude2 - max_altitude1 ) as i64 * ((1000000000000 - rock_offset) / (rocks_in_pattern) as i64) + (max_altitude - max_altitude2 ) as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}