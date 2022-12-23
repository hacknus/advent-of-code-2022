use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayTwentyThree {}

#[derive(Debug, Clone, PartialEq)]
pub struct Elf {
    pub pos: [i32; 2],
    pub next: [i32; 2],
}

impl Problem for DayTwentyThree {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut elves = vec![];
        for (y, row) in contents.iter().enumerate() {
            for (x, element) in row.char_indices() {
                if element == '#' {
                    elves.push(Elf { pos: [x as i32, y as i32], next: [x as i32, y as i32] })
                }
            }
        }

        for round in 0..10 {

            // first half of move
            for i in 0..elves.len() {
                let mut north = true;
                let mut northeast = true;
                let mut northwest = true;
                let mut east = true;
                let mut west = true;
                let mut southeast = true;
                let mut southwest = true;
                let mut south = true;
                for j in 0..elves.len() {
                    if i != j {
                        if elves[j].pos[0] == elves[i].pos[0] && elves[j].pos[1] == elves[i].pos[1] - 1 {
                            north = false;
                        }
                        if elves[j].pos[0] == elves[i].pos[0] + 1 && elves[j].pos[1] == elves[i].pos[1] - 1 {
                            northeast = false;
                        }
                        if elves[j].pos[0] == elves[i].pos[0] - 1 && elves[j].pos[1] == elves[i].pos[1] - 1 {
                            northwest = false;
                        }
                        if elves[j].pos[0] == elves[i].pos[0] - 1 && elves[j].pos[1] == elves[i].pos[1] {
                            west = false;
                        }
                        if elves[j].pos[0] == elves[i].pos[0] + 1 && elves[j].pos[1] == elves[i].pos[1] {
                            east = false;
                        }
                        if elves[j].pos[0] == elves[i].pos[0] && elves[j].pos[1] == elves[i].pos[1] + 1 {
                            south = false;
                        }
                        if elves[j].pos[0] == elves[i].pos[0] + 1 && elves[j].pos[1] == elves[i].pos[1] + 1 {
                            southeast = false;
                        }
                        if elves[j].pos[0] == elves[i].pos[0] - 1 && elves[j].pos[1] == elves[i].pos[1] + 1 {
                            southwest = false;
                        }
                    }
                }
                if north && south && southeast && northeast && northwest && southwest && west && east {
                    // stay here
                    elves[i].next = elves[i].pos;
                } else {
                    if round % 4 == 0 {
                        if north && northeast && northwest {
                            // move north
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] - 1];
                        } else if south && southwest && southeast {
                            // move south
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] + 1];
                        } else if northwest && west && southwest {
                            // move west
                            elves[i].next = [elves[i].pos[0] - 1, elves[i].pos[1]];
                        } else if northeast && east && southeast {
                            // move east
                            elves[i].next = [elves[i].pos[0] + 1, elves[i].pos[1]];
                        } else {
                            elves[i].next = elves[i].pos;
                        }
                    } else if round % 4 == 1 {
                        if south && southwest && southeast {
                            // move south
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] + 1];
                        } else if northwest && west && southwest {
                            // move west
                            elves[i].next = [elves[i].pos[0] - 1, elves[i].pos[1]];
                        } else if northeast && east && southeast {
                            // move east
                            elves[i].next = [elves[i].pos[0] + 1, elves[i].pos[1]];
                        } else if north && northeast && northwest {
                            // move north
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] - 1];
                        } else {
                            elves[i].next = elves[i].pos;
                        }
                    } else if round % 4 == 2 {
                        if northwest && west && southwest {
                            // move west
                            elves[i].next = [elves[i].pos[0] - 1, elves[i].pos[1]];
                        } else if northeast && east && southeast {
                            // move east
                            elves[i].next = [elves[i].pos[0] + 1, elves[i].pos[1]];
                        } else if north && northeast && northwest {
                            // move north
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] - 1];
                        } else if south && southwest && southeast {
                            // move south
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] + 1];
                        } else {
                            elves[i].next = elves[i].pos;
                        }
                    } else if round % 4 == 3 {
                        if northeast && east && southeast {
                            // move east
                            elves[i].next = [elves[i].pos[0] + 1, elves[i].pos[1]];
                        } else if north && northeast && northwest {
                            // move north
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] - 1];
                        } else if south && southwest && southeast {
                            // move south
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] + 1];
                        } else if northwest && west && southwest {
                            // move west
                            elves[i].next = [elves[i].pos[0] - 1, elves[i].pos[1]];
                        } else {
                            elves[i].next = elves[i].pos;
                        }
                    }
                }
            }

            // second half of move
            let mut elves_to_freeze = vec![];
            for i in 0..elves.len() {
                for j in 0..elves.len() {
                    if i != j {
                        if elves[i].next == elves[j].next {
                            elves_to_freeze.push(i);
                        }
                    }
                }
            }
            for i in elves_to_freeze.iter() {
                elves[*i].next = elves[*i].pos;
            }
            for elf in elves.iter_mut() {
                elf.pos = elf.next;
            }
        }

        let x = elves.iter().map(|elf| elf.pos[0] as isize).collect::<Vec<isize>>();
        let y = elves.iter().map(|elf| elf.pos[1] as isize).collect::<Vec<isize>>();

        let xmin = x.iter().min().unwrap();
        let xmax = x.iter().max().unwrap();
        let ymin = y.iter().min().unwrap();
        let ymax = y.iter().max().unwrap();

        let mut empty_counter = 0;
        for x in *xmin..=*xmax {
            for y in *ymin..=*ymax {
                let mut is_empty = true;
                for elf in elves.iter() {
                    if elf.pos == [x as i32, y as i32] {
                        is_empty = false;
                    }
                }
                if is_empty {
                    empty_counter += 1;
                }
            }
        }

        format!("{}", empty_counter)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut elves = vec![];
        for (y, row) in contents.iter().enumerate() {
            for (x, element) in row.char_indices() {
                if element == '#' {
                    elves.push(Elf { pos: [x as i32, y as i32], next: [x as i32, y as i32] })
                }
            }
        }

        let mut round = 0;
        let mut old_state = elves.clone();
        loop {
            // first half of move
            for i in 0..elves.len() {
                let mut north = true;
                let mut northeast = true;
                let mut northwest = true;
                let mut east = true;
                let mut west = true;
                let mut southeast = true;
                let mut southwest = true;
                let mut south = true;
                for j in 0..elves.len() {
                    if i != j {
                        if elves[j].pos[0] == elves[i].pos[0] && elves[j].pos[1] == elves[i].pos[1] - 1 {
                            north = false;
                        }
                        if elves[j].pos[0] == elves[i].pos[0] + 1 && elves[j].pos[1] == elves[i].pos[1] - 1 {
                            northeast = false;
                        }
                        if elves[j].pos[0] == elves[i].pos[0] - 1 && elves[j].pos[1] == elves[i].pos[1] - 1 {
                            northwest = false;
                        }
                        if elves[j].pos[0] == elves[i].pos[0] - 1 && elves[j].pos[1] == elves[i].pos[1] {
                            west = false;
                        }
                        if elves[j].pos[0] == elves[i].pos[0] + 1 && elves[j].pos[1] == elves[i].pos[1] {
                            east = false;
                        }
                        if elves[j].pos[0] == elves[i].pos[0] && elves[j].pos[1] == elves[i].pos[1] + 1 {
                            south = false;
                        }
                        if elves[j].pos[0] == elves[i].pos[0] + 1 && elves[j].pos[1] == elves[i].pos[1] + 1 {
                            southeast = false;
                        }
                        if elves[j].pos[0] == elves[i].pos[0] - 1 && elves[j].pos[1] == elves[i].pos[1] + 1 {
                            southwest = false;
                        }
                    }
                }
                if north && south && southeast && northeast && northwest && southwest && west && east {
                    // stay here
                    elves[i].next = elves[i].pos;
                } else {
                    if round % 4 == 0 {
                        if north && northeast && northwest {
                            // move north
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] - 1];
                        } else if south && southwest && southeast {
                            // move south
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] + 1];
                        } else if northwest && west && southwest {
                            // move west
                            elves[i].next = [elves[i].pos[0] - 1, elves[i].pos[1]];
                        } else if northeast && east && southeast {
                            // move east
                            elves[i].next = [elves[i].pos[0] + 1, elves[i].pos[1]];
                        } else {
                            elves[i].next = elves[i].pos;
                        }
                    } else if round % 4 == 1 {
                        if south && southwest && southeast {
                            // move south
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] + 1];
                        } else if northwest && west && southwest {
                            // move west
                            elves[i].next = [elves[i].pos[0] - 1, elves[i].pos[1]];
                        } else if northeast && east && southeast {
                            // move east
                            elves[i].next = [elves[i].pos[0] + 1, elves[i].pos[1]];
                        } else if north && northeast && northwest {
                            // move north
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] - 1];
                        } else {
                            elves[i].next = elves[i].pos;
                        }
                    } else if round % 4 == 2 {
                        if northwest && west && southwest {
                            // move west
                            elves[i].next = [elves[i].pos[0] - 1, elves[i].pos[1]];
                        } else if northeast && east && southeast {
                            // move east
                            elves[i].next = [elves[i].pos[0] + 1, elves[i].pos[1]];
                        } else if north && northeast && northwest {
                            // move north
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] - 1];
                        } else if south && southwest && southeast {
                            // move south
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] + 1];
                        } else {
                            elves[i].next = elves[i].pos;
                        }
                    } else if round % 4 == 3 {
                        if northeast && east && southeast {
                            // move east
                            elves[i].next = [elves[i].pos[0] + 1, elves[i].pos[1]];
                        } else if north && northeast && northwest {
                            // move north
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] - 1];
                        } else if south && southwest && southeast {
                            // move south
                            elves[i].next = [elves[i].pos[0], elves[i].pos[1] + 1];
                        } else if northwest && west && southwest {
                            // move west
                            elves[i].next = [elves[i].pos[0] - 1, elves[i].pos[1]];
                        } else {
                            elves[i].next = elves[i].pos;
                        }
                    }
                }
            }

            // second half of move
            let mut elves_to_freeze = vec![];
            for i in 0..elves.len() {
                for j in 0..elves.len() {
                    if i != j {
                        if elves[i].next == elves[j].next {
                            elves_to_freeze.push(i);
                        }
                    }
                }
            }
            for i in elves_to_freeze.iter() {
                elves[*i].next = elves[*i].pos;
            }
            for elf in elves.iter_mut() {
                elf.pos = elf.next;
            }
            round += 1;
            if old_state == elves {
                break;
            }
            old_state = elves.clone();
        }
        format!("{}", round)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}