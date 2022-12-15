use std::ops::Index;
use itertools::{Itertools, Unique};
use crate::io::read_file_lines;
use crate::problem::Problem;
use std::collections::HashSet;
use std::time::Instant;

pub struct DayFifthteen {}

#[derive(Debug, Clone, PartialEq)]
pub struct Sensor {
    x: i32,
    y: i32,
    x_b: i32,
    y_b: i32,
    r: i32,
}

pub fn get_tuning_freq(x: i32, y: i32) -> i64 {
    4000000 * x as i64 + y as i64
}

pub fn get_missing(set: HashSet<i32>) -> i32 {
    let mut arr: Vec<i32> = set.into_iter().collect();
    arr.sort();
    for (i, a) in arr.iter().enumerate() {
        if i as i32 != *a {
            return i as i32;
        }
    }
    arr.len() as i32
}

impl Sensor {
    pub fn new(x: i32, y: i32, x_b: i32, y_b: i32) -> Self {
        let r = (x - x_b).abs() + (y - y_b).abs();
        Sensor { x, y, r, x_b, y_b }
    }

    pub fn new_from_line(line: &str) -> Self {
        let line_split = line.split(": ").map(|s| s.to_string()).collect::<Vec<String>>();
        let sensor = &line_split[0];
        let sensor_val = sensor.split("=").map(|s| s.to_string()).collect::<Vec<String>>();
        let x = sensor_val[1].replace(", y", "").parse::<i32>().unwrap();
        let y = sensor_val[2].parse::<i32>().unwrap();
        let beacon = &line_split[1];
        let beacon_val = beacon.split("=").map(|s| s.to_string()).collect::<Vec<String>>();
        let x_b = beacon_val[1].replace(", y", "").parse::<i32>().unwrap();
        let y_b = beacon_val[2].parse::<i32>().unwrap();
        Sensor::new(x, y, x_b, y_b)
    }

    pub fn get_reach(&self, y: i32) -> Option<HashSet<i32>> {
        if y > self.y + self.r || y < self.y - self.r {
            None
        } else {
            let num = 2 * (self.r - (self.y - y).abs()) + 1;
            let start = self.x - num / 2;
            let end = self.x + num / 2;
            let row = HashSet::from_iter(start..=end);
            Some(row)
        }
    }

    pub fn totally_included(&self, y: i32, max_val: i32) -> bool {
        if !(y > self.y + self.r || y < self.y - self.r) {
            let num = 2 * (self.r - (self.y - y).abs()) + 1;
            let start = self.x - num / 2;
            let end = self.x + num / 2;
            if start <= 0 && end >= max_val {
                return true;
            }
        }
        false
    }


    pub fn get_reach_limited(&self, y: i32, max_val: i32) -> Option<[i32; 2]> {
        if y > self.y + self.r || y < self.y - self.r {
            None
        } else {
            let num = 2 * (self.r - (self.y - y).abs()) + 1;
            let mut start = self.x - num / 2;
            let mut end = self.x + num / 2;
            if start < 0 {
                start = 0;
            }
            if end > max_val {
                end = max_val;
            }
            let row = [start, end];
            Some(row)
        }
    }
}

impl Problem for DayFifthteen {
    fn part_one(&self, input: &str) -> String {
        let mut contents = read_file_lines(input);
        let mut sensors = vec![];
        let mut beacon_x = vec![];
        let mut beacon_y = vec![];
        for line in contents.iter() {
            let sensor = Sensor::new_from_line(line);
            beacon_x.push(sensor.x_b.clone());
            beacon_y.push(sensor.y_b.clone());
            sensors.push(sensor);
        }

        let y = 2000000;
        let mut row: HashSet<i32> = HashSet::new();
        for sensor in sensors.iter() {
            match sensor.get_reach(y) {
                None => {}
                Some(mut r) => {
                    row.extend(&r);
                }
            }
        }

        let mut num = row.len();
        num -= row.iter().filter(|&n| *n == y).count();
        println!("{:?}", num);
        format!("{:?}", num)
    }

    fn part_two(&self, input: &str) -> String {
        let mut contents = read_file_lines(input);
        let mut sensors = vec![];
        for line in contents.iter() {
            let sensor = Sensor::new_from_line(line);
            sensors.push(sensor);
        }

        let mut result_x = 0;
        let mut result_y = 0;
        let max_val = 4000000;
        let mut row: Vec<[i32; 2]> = vec![];
        for y in 0..=max_val {
            row = vec![];
            for sensor in sensors.iter() {
                match sensor.get_reach_limited(y, max_val) {
                    None => {}
                    Some(mut r) => {
                        if row.len() > 0 {
                            let mut overlap = false;
                            for i in 0..row.len() {
                                // check for overlap
                                if r[0] < row[i][0] && r[1] >= row[i][0] {
                                    row[i][0] = r[0];
                                    overlap = true;
                                }
                                if r[1] > row[i][1] && r[0] <= row[i][1] {
                                    row[i][1] = r[1];
                                    overlap = true;
                                }
                                if r[1] <= row[i][1] && r[0] >= row[i][0] {
                                    overlap = true;
                                }
                            }
                            if !overlap {
                                row.push(r);
                            }
                            let mut new_row = vec![row[0]];
                            for i in 0..row.len() {
                                // check for overlap
                                if new_row[0][0] > row[i][0] && new_row[0][0] <= row[i][1] {
                                    new_row[0][0] = row[i][0];
                                }
                                if new_row[0][1] < row[i][1] && new_row[0][1] >= row[i][0] {
                                    new_row[0][1] = row[i][1];
                                }
                                if new_row[0][1] >= row[i][1] && new_row[0][0] <= row[i][0] {} else {
                                    new_row.push(row[i]);
                                }
                            }
                            row = new_row;
                        } else {
                            row.push(r);
                        }
                        if row[0][0] == 0 && row[0][1] == max_val {
                            break;
                        }
                    }
                }
            }
            if row[0][0] != 0 || row[0][1] != max_val {
                result_y = y;
                break;
            }
        }
        result_x = row[0][1] + 1;
        format!("{}", get_tuning_freq(result_x, result_y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}