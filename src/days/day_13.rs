use std::fs;
use std::num::ParseIntError;
use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayThirteen {}

#[derive(Debug)]
pub struct Element {
    pub val: Option<u32>,
    pub children: Option<Vec<Element>>,
}

impl Element {
    pub fn new(s: &str) -> Self {
        let mut val = None;
        let mut children = None;
        if s.len() == 0 {
            return Element { val, children };
        }
        match s.parse::<u32>() {
            Ok(val) => {
                return Element { val: Some(val), children };
            }
            Err(_) => {}
        }
        return match extract_vec(s) {
            None => {
                Element { val, children }
            }
            Some(vec) => {
                let mut ele_vec = vec![];
                for v in vec.iter() {
                    ele_vec.push(Element::new(&v));
                }
                children = Some(ele_vec);
                Element { val, children }
            }
        };
    }
}

pub fn helper_old(val: &str) -> Vec<String> {
    let mut vec = vec![];
    let mut temp_vec = vec![];
    let mut skip_next = false;
    let mut bracket_counter = 0;
    let mut number = "".to_string();
    for si in val.chars() {
        if si == '[' {
            bracket_counter += 1;
        }
        if si == ']' {
            bracket_counter -= 1;
        }
        if skip_next && si == ',' {
            skip_next = false;
            continue;
        }
        if bracket_counter == 0 {
            temp_vec.push(si.to_string());
            vec.push(temp_vec.iter().map(|si| si.to_string()).collect::<String>());
            temp_vec = vec![];
            skip_next = true;
        } else {
            if si.is_digit(10) {
                number.push(si);
            } else if si == ',' || si == ']' {
                temp_vec.push(number.clone());
                number = "".to_string();
            }
        }
        if si == ',' || si == '[' || si == ']' {
            temp_vec.push(si.to_string());
        }
    }
    if temp_vec.len() != 0 {
        vec.push(temp_vec.iter().map(|si| si.to_string()).collect::<String>());
    }
    println!("found finally {vec:?}");
    return vec;
}


pub fn helper(val: &str) -> Vec<String> {
    let mut vec = vec![];
    let mut temp_vec = vec![];
    let mut skip_next = false;
    let mut bracket_counter = 0;
    let mut number = "".to_string();
    for si in val.chars() {
        //println!("char : {si}");
        if si == '[' {
            bracket_counter += 1;
        }
        if si == ']' {
            bracket_counter -= 1;
        }
        if si.is_digit(10) {
            //println!("adding {si}");
            number.push(si);
        } else {
            if number != "".to_string() {
                //println!("pushing : {number}");
                temp_vec.push(number.clone());
                number = "".to_string();
            }
        }
        if skip_next && si == ',' {
            if temp_vec.len() != 0 {
                //println!("pushing to super: {temp_vec:?}");
                vec.push(temp_vec.iter().map(|si| si.to_string()).collect::<String>());
            }
            temp_vec = vec![];
            skip_next = false;
            continue;
        } else {}
        if bracket_counter == 0 {
            if temp_vec.len() != 0 {
                temp_vec.push(si.to_string());
                //println!("pushing to super: {temp_vec:?}");
                vec.push(temp_vec.iter().map(|si| si.to_string()).collect::<String>());
                temp_vec = vec![];
            }

            skip_next = true;
        } else {
            if si == ',' || si == '[' || si == ']' {
                //println!("pushing delim : {si}");
                temp_vec.push(si.to_string());
            }
        }
    }
    if number != "".to_string() {
        //println!("pushing to super {number}");
        temp_vec.push(number.clone());
    }
    if temp_vec.len() != 0 {
        vec.push(temp_vec.iter().map(|si| si.to_string()).collect::<String>());
    }
    // println!("found finally {vec:?}");
    return vec;
}

pub fn extract_vec(s: &str) -> Option<Vec<String>> {
    let len = s.len();
    // println!("extracting :{s:?}");
    if s.contains("[") {
        let mut val = s.chars()
            .take(len - 1)
            .skip(1)
            .collect::<String>();
        if val.len() == 0 {
            return None;
        }
        if val.contains("[") {
            let vec = helper(&val);
            return Some(vec);
        } else {
            let v = val.split(",").map(|s| s.to_string()).collect();
            return Some(v);
        }
    } else {
        let v = s.split(",").map(|s| s.to_string()).collect();
        return Some(v);
    }
}

pub fn check_order(left: &Element, right: &Element) -> Option<bool> {
    match (left.val, right.val) {
        (Some(a), Some(b)) => {
            println!("comparing {a} and {b}");
            if a > b {
                println!("false on order");
                return Some(false);
            } else if a == b {
                return None;
            } else {
                return Some(true);
            }
        }
        (Some(a), None) => {
            match &right.children {
                None => {
                    return Some(false);
                }
                Some(elements) => {
                    match check_order(left, &elements[0]) {
                        None => { return None; }
                        Some(b) => { return Some(b); }
                    }
                }
            }
        }
        (None, Some(b)) => {
            match &left.children {
                None => {
                    return Some(true);
                }
                Some(elements) => {
                    match check_order(&elements[0], right) {
                        None => { return None; }
                        Some(b) => { return Some(b); }
                    }
                }
            }
        }
        (None, None) => {
            match (&left.children, &right.children) {
                (Some(ele_a), Some(ele_b)) => {
                    for (a, b) in ele_a.iter().zip(ele_b.iter()) {
                        match check_order(a, b) {
                            None => {}
                            Some(temp_bool) => {
                                return Some(temp_bool);
                            }
                        }
                    }
                    if ele_a.len() > ele_b.len() {
                        println!("right is shorter than left, false!");
                        return Some(false);
                    } else if ele_a.len() < ele_b.len() {
                        println!("left is shorter than right, correct!");
                        return Some(true);
                    }
                }
                (Some(ele_a), None) => {
                    println!("comparing {ele_a:?} and no right");
                    return Some(false);
                }
                (None, Some(ele_b)) => {
                    println!("comparing no left and {ele_b:?}");
                    return Some(true);
                }
                (None, None) => {
                    println!("both empty");
                    return Some(true);
                }
            }
        }
    }
    println!("hit none!");
    None
}

impl Problem for DayThirteen {
    fn part_one(&self, input: &str) -> String {
        let contents_raw = fs::read_to_string(input)
            .expect("Should have been able to read the file");
        let contents = contents_raw.split("\n\n").map(|s| s.to_string()).collect::<Vec<String>>();
        println!("{contents:?}");
        let mut indices = vec![];
        for (i, pair_raw) in contents.iter().enumerate() {
            let pair = pair_raw.split("\n").map(|s| s.to_string()).collect::<Vec<String>>();
            println!("raw left: {}  raw right: {}", pair[0], pair[1]);
            let mut left = Element::new(&pair[0]);
            let mut right = Element::new(&pair[1]);
            // println!("left: {:#?} ", left);
            // println!("right: {:#?} ", right);
            match check_order(&left, &right) {
                None => {}
                Some(b) => {
                    if b {
                        println!("{} is ordered", i + 1);
                        indices.push(i + 1);
                    }
                }
            }
        }
        format!("{}", indices.iter().sum::<usize>())
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