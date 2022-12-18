use itertools::Itertools;
use crate::days::day_18::CubeLabel::{BACKGROUND, EMPTY, HOLE, LAVA};
use crate::io::read_from_csv;
use crate::problem::Problem;

pub struct DayEighteen {}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum CubeLabel {
    EMPTY,
    LAVA,
    HOLE,
    BACKGROUND
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cube {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub id: CubeLabel,
}

pub fn is_touching(a: &Cube, b: &Cube) -> i32 {
    if a.x == b.x + 1 && a.y == b.y && a.z == b.z {
        return 1;
    } else if a.x == b.x - 1 && a.y == b.y && a.z == b.z {
        return 1;
    } else if a.x == b.x && a.y == b.y + 1 && a.z == b.z {
        return 1;
    } else if a.x == b.x && a.y == b.y - 1 && a.z == b.z {
        return 1;
    } else if a.x == b.x && a.y == b.y && a.z == b.z + 1 {
        return 1;
    } else if a.x == b.x && a.y == b.y && a.z == b.z - 1 {
        return 1;
    }
    0
}

pub fn waterfall(index: usize, cubes: &mut Vec<Cube>) {
    let min_x = cubes.iter().map(|c| c.x).min().unwrap();
    let max_x = cubes.iter().map(|c| c.x).max().unwrap();
    let min_y = cubes.iter().map(|c| c.y).min().unwrap();
    let max_y = cubes.iter().map(|c| c.y).max().unwrap();
    let min_z = cubes.iter().map(|c| c.z).min().unwrap();
    let max_z = cubes.iter().map(|c| c.z).max().unwrap();

    if cubes[index].id != EMPTY {
        return;
    }
    if cubes[index].x >= max_x || cubes[index].x <= min_x ||
        cubes[index].y >= max_y || cubes[index].y <= min_y ||
        cubes[index].z >= max_z || cubes[index].z <= min_z {
        cubes[index].id = BACKGROUND;
        return;
    }
    cubes[index].id = HOLE;
    for (x, y, z) in [(0, 0, 0), (-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, 1), (0, 0, -1)].iter() {
        match cubes.iter().position(|c| c.x == cubes[index].x + x
            && c.y == cubes[index].y + y && c.z == cubes[index].z + z) {
            None => {}
            Some(next) => {
                waterfall(next, cubes);
                if cubes[next].id == BACKGROUND {
                    cubes[index].id = BACKGROUND;
                    return;
                }
            }
        }
    }
}

pub fn populate_cubes(cubes: &mut Vec<Cube>) {
    let min_x = cubes.iter().map(|c| c.x).min().unwrap();
    let max_x = cubes.iter().map(|c| c.x).max().unwrap();
    let min_y = cubes.iter().map(|c| c.y).min().unwrap();
    let max_y = cubes.iter().map(|c| c.y).max().unwrap();
    let min_z = cubes.iter().map(|c| c.z).min().unwrap();
    let max_z = cubes.iter().map(|c| c.z).max().unwrap();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                let mut cube = Cube {
                    x,
                    y,
                    z,
                    id: LAVA,
                };
                if !cubes.contains(&cube) {
                    cube.id = EMPTY;
                    cubes.push(cube);
                }
            }
        }
    }
}


impl Problem for DayEighteen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_from_csv(input, b',');
        let mut cubes = vec![];
        for line in contents.iter() {
            let cube: [i32; 3] = line.iter().map(|l| l.parse::<i32>().unwrap()).collect::<Vec<i32>>().try_into().unwrap();
            cubes.push(Cube {
                x: cube[0],
                y: cube[1],
                z: cube[2],
                id: LAVA,
            });
        }
        let num_cubes = cubes.len();
        let mut counter = 0;
        for i in 0..cubes.len() {
            for j in 0..cubes.len() {
                if i != j {
                    counter += is_touching(&cubes[i], &cubes[j]);
                }
            }
        }

        let surface = num_cubes as i32 * 6 - counter;
        format!("{}", surface)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_from_csv(input, b',');
        let mut cubes = vec![];
        for line in contents.iter() {
            let cube: [i32; 3] = line.iter().map(|l| l.parse::<i32>().unwrap()).collect::<Vec<i32>>().try_into().unwrap();
            cubes.push(Cube {
                x: cube[0],
                y: cube[1],
                z: cube[2],
                id: LAVA,
            });
        }
        let num_cubes = cubes.len() as i32;
        let mut count_surfaces = 0;
        for i in 0..cubes.len() {
            for j in 0..cubes.len() {
                if i != j {
                    count_surfaces += is_touching(&cubes[i], &cubes[j]);
                }
            }
        }

        populate_cubes(&mut cubes);

        for i in 0..cubes.len() {
            waterfall(i, &mut cubes);
        }

        let mut inside_cubes = cubes.iter().filter(|c| c.id == HOLE).collect::<Vec<&Cube>>();
        let mut count_inside = inside_cubes.len() as i32;


        // subtract all touching surfaces of inside cubes
        let mut touching_insides = 0;
        for i in 0..inside_cubes.len() {
            for j in 0..inside_cubes.len() {
                if i != j {
                    touching_insides += is_touching(&inside_cubes[i], &inside_cubes[j]);
                }
            }
        }

        let surface = num_cubes * 6 - count_surfaces - count_inside * 6 + touching_insides;
        format!("{}", surface)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}