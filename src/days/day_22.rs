use std::fs;
use itertools::Itertools;
use regex::Regex;
use crate::problem::Problem;

pub struct DayTwentyTwo {}


#[derive(Debug, Clone)]
pub enum Direction {
    RIGHT = 0,
    DOWN = 1,
    LEFT = 2,
    UP = 3,
}

#[derive(Debug, Clone)]
pub struct Human {
    x: usize,
    y: usize,
    next_x: usize,
    next_y: usize,
    dir: Direction,
    next_dir: Direction,
}

impl Human {
    pub fn make_move(&mut self, distance: &u32, grid: &Vec<Vec<u32>>) {
        let width = grid.len();
        let height = grid[0].len();
        for _ in 0..*distance {
            let dx;
            let dy;
            match self.dir {
                Direction::UP => {
                    dx = 0;
                    dy = -1;
                }
                Direction::DOWN => {
                    dx = 0;
                    dy = 1;
                }
                Direction::LEFT => {
                    dx = -1;
                    dy = 0;
                }
                Direction::RIGHT => {
                    dx = 1;
                    dy = 0;
                }
            }
            // logic for collisions
            if self.y == 0 && dy == -1 {
                // top to bottom wrap around
                let next_y = grid[self.x].iter().positions(|y| *y == 1 || *y == 2).last().unwrap();
                self.next_y = next_y;
                self.next_x = self.x;
            } else if self.y == height - 1 && dy == 1 {
                // bottom to top wrap around
                let next_y = grid[self.x].iter().positions(|y| *y == 1 || *y == 2).nth(0).unwrap();
                self.next_y = next_y;
                self.next_x = self.x;
            } else if self.x == width - 1 && dx == 1 {
                // right to left wrap around
                let next_x = grid.iter().positions(|x| x[self.y] == 1 || x[self.y] == 2).nth(0).unwrap();
                self.next_x = next_x;
                self.next_y = self.y;
            } else if self.x == 0 && dx == -1 {
                // left to right wrap around
                let next_x = grid.iter().positions(|x| x[self.y] == 1 || x[self.y] == 2).last().unwrap();
                self.next_x = next_x;
                self.next_y = self.y;
            } else if grid[(self.x as i32 + dx) as usize][(self.y as i32 + dy) as usize] == 0 {
                // wrap arouuuuund
                match self.dir {
                    Direction::UP => {
                        let next_y = grid[self.x].iter().positions(|y| *y == 1 || *y == 2).last().unwrap();
                        self.next_y = next_y;
                        self.next_x = self.x;
                    }
                    Direction::DOWN => {
                        let next_y = grid[self.x].iter().positions(|y| *y == 1 || *y == 2).nth(0).unwrap();
                        self.next_y = next_y;
                        self.next_x = self.x;
                    }
                    Direction::LEFT => {
                        let next_x = grid.iter().positions(|x| x[self.y] == 1 || x[self.y] == 2).last().unwrap();
                        self.next_x = next_x;
                        self.next_y = self.y;
                    }
                    Direction::RIGHT => {
                        let next_x = grid.iter().positions(|x| x[self.y] == 1 || x[self.y] == 2).nth(0).unwrap();
                        self.next_x = next_x;
                        self.next_y = self.y;
                    }
                }
            } else {
                self.next_x = (self.x as i32 + dx) as usize;
                self.next_y = (self.y as i32 + dy) as usize;
            }

            if grid[self.next_x][self.next_y] == 2 {
                // hit the wall, jack. don't you come back no more, no more, no more
                break;
            } else if grid[self.next_x][self.next_y] == 1 {
                // hit the road, jack. don't you come back no more, no more, no more
                self.x = self.next_x;
                self.y = self.next_y;
            }
        }
    }

    pub fn make_move_cube(&mut self, distance: &u32, size: &usize, grid: &Vec<Vec<u32>>) {
        // cube layout:
        //
        //              __A__
        //              |B C|
        //      __A__B__|   |_C__
        //      | D             D|
        //      | D             D|
        //      ¨¨F¨¨G¨¨|   |¨E¨¨
        //              |G E|
        //              ¨¨F¨¨

        let width = grid.len();
        let height = grid[0].len();
        for _ in 0..*distance {
            let dx;
            let dy;
            self.next_dir = self.dir.clone();
            match self.dir {
                Direction::UP => {
                    dx = 0;
                    dy = -1;
                }
                Direction::DOWN => {
                    dx = 0;
                    dy = 1;
                }
                Direction::LEFT => {
                    dx = -1;
                    dy = 0;
                }
                Direction::RIGHT => {
                    dx = 1;
                    dy = 0;
                }
            }
            // logic for collisions
            if self.y == 0 && dy == -1 {
                // going over side A up
                self.next_dir = Direction::DOWN;
                self.next_y = *size;
                self.next_x = size - 1 - (self.x - 2 * size);
            } else if self.y == height - 1 && dy == 1 {
                // going over side F down
                self.next_dir = Direction::UP;
                self.next_y = 2 * size - 1;
                self.next_x = size - 1 - (self.x - 2 * size);
            } else if self.x == width - 1 && dx == 1 {
                // right to left wrap around (side D)
                // let next_x = grid.iter().positions(|x| x[self.y] == 1 || x[self.y] == 2).nth(0).unwrap();
                self.next_x = 0;
                self.next_y = self.y;
            } else if self.x == 0 && dx == -1 {
                // left to right wrap around (side D)
                // let next_x = grid.iter().positions(|x| x[self.y] == 1 || x[self.y] == 2).last().unwrap();
                self.next_x = width - 1;
                self.next_y = self.y;
            } else if grid[(self.x as i32 + dx) as usize][(self.y as i32 + dy) as usize] == 0 {
                // wrap around cube!
                match self.dir {
                    Direction::UP => {
                        // going over side A,B or C
                        if self.x < *size {
                            // going over side A up
                            self.next_dir = Direction::DOWN;
                            self.next_y = 0;
                            self.next_x = 3 * size - 1 - self.x;
                        } else if self.x >= *size && self.x < 2 * size {
                            // going over side B up
                            self.next_dir = Direction::RIGHT;
                            self.next_y = self.x - *size;
                            self.next_x = 2 * size;
                        } else if self.x >= 3 * size && self.x < 4 * size {
                            // going over side C up
                            self.next_dir = Direction::LEFT;
                            self.next_y = size - 1 - (self.x - 3 * size);
                            self.next_x = 3 * size - 1;
                        }
                    }
                    Direction::DOWN => {
                        // going over side F,G or E
                        if self.x < *size {
                            // going over side F down
                            self.next_dir = Direction::UP;
                            self.next_y = 3 * size - 1;
                            self.next_x = 3 * size - 1 - self.x;
                        } else if self.x >= *size && self.x < 2 * size {
                            // going over side G down
                            self.next_dir = Direction::RIGHT;
                            self.next_y = 3 * size - 1 - (self.x - *size);
                            self.next_x = 2 * size;
                        } else if self.x >= 3 * size && self.x < 4 * size {
                            // going over side E down
                            self.next_dir = Direction::LEFT;
                            self.next_y = 2 * size + (self.x - 3 * size);
                            self.next_x = 3 * size - 1;
                        }
                    }
                    Direction::LEFT => {
                        // going over B or G
                        if self.y < height / 2 {
                            // going over side B left
                            self.next_dir = Direction::DOWN;
                            self.next_y = *size;
                            self.next_x = size + self.y;
                        } else {
                            // going over side G left
                            self.next_dir = Direction::UP;
                            self.next_y = 2 * size - 1;
                            self.next_x = 2 * size - 1 - (self.y - 2 * size);
                        }
                    }
                    Direction::RIGHT => {
                        // going over C or E
                        if self.y < height / 2 {
                            // going over side C right
                            self.next_dir = Direction::DOWN;
                            self.next_y = *size;
                            self.next_x = 4 * size - 1 - self.y;
                        } else {
                            // going over side E right
                            self.next_dir = Direction::UP;
                            self.next_y = 2 * size - 1;
                            self.next_x = 3 * size + (self.y - 2 * size);
                        }
                    }
                }
            } else {
                self.next_x = (self.x as i32 + dx) as usize;
                self.next_y = (self.y as i32 + dy) as usize;
                self.dir = self.next_dir.clone();
            }

            if grid[self.next_x][self.next_y] == 2 {
                // hit the wall, jack. don't you come back no more, no more, no more
                break;
            } else if grid[self.next_x][self.next_y] == 1 {
                // hit the road, jack. don't you come back no more, no more, no more
                self.x = self.next_x;
                self.y = self.next_y;
                self.dir = self.next_dir.clone();
            }
        }
    }

    pub fn rotate(&mut self, turn: &str) {
        match self.dir {
            Direction::UP => {
                if turn == "R" {
                    self.dir = Direction::RIGHT;
                } else if turn == "L" {
                    self.dir = Direction::LEFT;
                }
            }
            Direction::DOWN => {
                if turn == "R" {
                    self.dir = Direction::LEFT;
                } else if turn == "L" {
                    self.dir = Direction::RIGHT;
                }
            }
            Direction::LEFT => {
                if turn == "R" {
                    self.dir = Direction::UP;
                } else if turn == "L" {
                    self.dir = Direction::DOWN;
                }
            }
            Direction::RIGHT => {
                if turn == "R" {
                    self.dir = Direction::DOWN;
                } else if turn == "L" {
                    self.dir = Direction::UP;
                }
            }
        }
    }
}

pub fn get_cube_size(grid: &Vec<Vec<u32>>) -> usize {
    let width = grid.len();
    let height = grid[0].len();

    let side_long = width.max(height);
    let size_long = side_long / 4;
    let side_short = width.min(height);
    let size_short = side_short / 3;

    assert_eq!(size_long, size_short);
    size_long
}

pub fn re_orient_grid(size: &usize, grid: &mut Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    // we only distinguish between the two layouts: example and input
    // theoretically we could implement ALL possible layouts
    // but then we would also have to deal with reorienting the direction of the
    // human when he/she starts walking (in these layouts the initial direction is RIGHT)

    let width = grid.len();
    let height = grid[0].len();

    // determine shape
    if width > height {
        // long side horizontal
        // this is the example shape
        let start_x = grid.iter().positions(|p| p[0] == 1).nth(0).unwrap();
        if start_x != 2 * size {
            panic!("Start position not at the right spot");
        }

        // rotate lower right square and shift up by 1
        for x in 0..*size {
            for y in 0..*size {
                grid[3 * size + y][2 * size - 1 - x] = grid[3 * size + x][2 * size + y];
                grid[3 * size + x][2 * size + y] = 0;
            }
        }
    } else {
        // long side is vertical
        // this is the puzzle input shape
        let start_x = grid.iter().positions(|p| p[0] == 1).nth(0).unwrap();
        if start_x != *size {
            panic!("Start position not at the right spot");
        }

        // mirror top right square and shift down by 2
        for x in 0..*size {
            for y in 0..*size {
                grid[3 * size - 1 - x][3 * size - 1 - y] = grid[2 * size + x][y];
                grid[2 * size + x][y] = 0;
            }
        }

        // rotate lower left square and shift right by 1
        for x in 0..*size {
            for y in 0..*size {
                grid[size + y][4 * size - 1 - x] = grid[x][3 * size + y];
                grid[x][3 * size + y] = 0;
            }
        }
    }
    if width > height {
        grid.clone()
    } else {
        // rotating counter-clockwise
        let mut rotated_grid = vec![vec![0; width]; height];
        for x in 0..width {
            for y in 0..height {
                rotated_grid[y][width - 1 - x] = grid[x][y].clone();
            }
        }
        rotated_grid
    }
}


impl Problem for DayTwentyTwo {
    fn part_one(&self, input: &str) -> String {
        let contents = fs::read_to_string(input)
            .expect("Should have been able to read the file");
        let contents_split = contents.split("\n\n")
            .map(|s| s.to_string()).collect::<Vec<String>>();
        let grid_raw = &contents_split[0];
        let instructions = &contents_split[1];
        let grid_lines = grid_raw.split("\n")
            .map(|s| s.to_string()).collect::<Vec<String>>();
        let width = grid_lines.iter().map(|l| l.len()).max().unwrap();
        let height = grid_lines.len();

        let mut grid = vec![vec![0; height]; width];
        for (y, line) in grid_lines.iter().enumerate() {
            for (x, field) in line.char_indices() {
                if field == '.' {
                    grid[x][y] = 1;
                } else if field == '#' {
                    grid[x][y] = 2;
                }
            }
        }

        let re_numbers = Regex::new(r"[0-9]*").unwrap();
        let numbers = re_numbers.find_iter(instructions)
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect::<Vec<u32>>();
        let re_turns = Regex::new(r"[A-Z]").unwrap();
        let turns = re_turns.find_iter(instructions)
            .filter_map(|digits| Option::from(digits.as_str().to_string()))
            .collect::<Vec<String>>();

        let x_start = grid.iter().position(|x| x[0] == 1).unwrap();
        let mut myself = Human {
            x: x_start,
            y: 0,
            next_x: 0,
            next_y: 0,
            dir: Direction::RIGHT,
            next_dir: Direction::RIGHT,
        };
        for (i, distance) in numbers.iter().enumerate() {
            // move forward
            myself.make_move(&distance, &grid);

            // rotate
            if i < turns.len() {
                myself.rotate(&turns[i]);
            }
        }

        format!("{}", 1000 * (myself.y + 1) + 4 * (myself.x + 1) + myself.dir as usize)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = fs::read_to_string(input)
            .expect("Should have been able to read the file");
        let contents_split = contents.split("\n\n")
            .map(|s| s.to_string()).collect::<Vec<String>>();
        let grid_raw = &contents_split[0];
        let instructions = &contents_split[1];
        let grid_lines = grid_raw.split("\n")
            .map(|s| s.to_string()).collect::<Vec<String>>();
        let width = grid_lines.iter().map(|l| l.len()).max().unwrap();
        let height = grid_lines.len();

        let mut grid = vec![vec![0; height]; width];
        for (y, line) in grid_lines.iter().enumerate() {
            for (x, field) in line.char_indices() {
                if field == '.' {
                    grid[x][y] = 1;
                } else if field == '#' {
                    grid[x][y] = 2;
                }
            }
        }

        let re_numbers = Regex::new(r"[0-9]*").unwrap();
        let numbers = re_numbers.find_iter(instructions)
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect::<Vec<u32>>();
        let re_turns = Regex::new(r"[A-Z]").unwrap();
        let turns = re_turns.find_iter(instructions)
            .filter_map(|digits| Option::from(digits.as_str().to_string()))
            .collect::<Vec<String>>();

        let cube_size = get_cube_size(&grid);
        let mut grid = re_orient_grid(&cube_size, &mut grid);

        let old_width = width;
        let width = grid.len();
        let height = grid[0].len();

        let dir;
        let x_start;
        let y_start;
        if old_width != width {
            // rotation registered, need to transform start location and direction!
            dir = Direction::UP;
            x_start = 0;
            y_start = grid[0].iter().positions(|y| *y == 1).last().unwrap();
        } else {
            // example, no rotation registered
            x_start = grid.iter().position(|x| x[0] == 1).unwrap();
            y_start = 0;
            dir = Direction::RIGHT;
        }

        let mut myself = Human {
            x: x_start,
            y: y_start,
            next_x: 0,
            next_y: 0,
            dir: dir.clone(),
            next_dir: dir,
        };

        for (i, distance) in numbers.iter().enumerate() {
            // move forward
            myself.make_move_cube(&distance, &cube_size, &grid);

            // rotate
            if i < turns.len() {
                myself.rotate(&turns[i]);
            }
        }

        // if we are in a previously rotated grid square, we would have to adjust it.
        // this is not the case for the example!

        let mut x = 0;
        let mut y = 0;
        let mut dir = Direction::RIGHT;
        if old_width != width {
            // ATTENTION: this transformation is more or less hardcoded, and does not handle all cases!
            // rotation registered, need to rotate 90 deg clockwise
            x = myself.x;
            y = myself.y;
            dir = myself.dir;
            let temp_x = x;
            x = height - 1 - y;
            y = temp_x;
            match dir {
                Direction::RIGHT => {
                    dir = Direction::DOWN;
                }
                Direction::DOWN => {
                    dir = Direction::LEFT;
                }
                Direction::LEFT => {
                    dir = Direction::UP;
                }
                Direction::UP => {
                    dir = Direction::RIGHT;
                }
            }

        } else {
            x = myself.x;
            y = myself.y;
            dir = myself.dir;
        }

        // higher than  6217
        // higher than 34491
        // higher than 33491
        //        not  45258
        //        not 123048
        //        not 129134
        //        not 131143
        // lower than 197026
        // lower than 200007
        format!("{}", 1000 * (y + 1) + 4 * (x + 1) + dir as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}