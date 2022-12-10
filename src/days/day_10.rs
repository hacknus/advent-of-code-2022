use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayTen {}

impl Problem for DayTen {
    fn part_one(&self, input: &str) -> String {
        let mut contents = read_file_lines(input);
        println!("{contents:?}");
        let mut cycles = vec![1];
        for instruction in contents.iter() {
            if instruction.contains("noop") {
                cycles.push(*cycles.last().unwrap());
            } else if instruction.contains("addx") {
                let val = instruction.split(" ").last().unwrap().parse::<i32>().unwrap();
                cycles.push(*cycles.last().unwrap());
                cycles.push(*cycles.last().unwrap() + val);
            }
        }
        let indices = [20, 60, 100, 140, 180, 220];
        let mut signal_strengths = vec![];
        for (i, s) in cycles.iter().enumerate() {
            if indices.contains(&(i as i32 + 1)) {
                signal_strengths.push(*s * (i + 1) as i32);
            }
        }
        format!("{:?}", signal_strengths.iter().sum::<i32>())
    }

    fn part_two(&self, input: &str) -> String {
        let mut contents = read_file_lines(input);
        println!("{contents:?}");
        let mut cycles = vec![1];
        for instruction in contents.iter() {
            if instruction.contains("noop") {
                cycles.push(*cycles.last().unwrap());
            } else if instruction.contains("addx") {
                let val = instruction.split(" ").last().unwrap().parse::<i32>().unwrap();
                cycles.push(*cycles.last().unwrap());
                cycles.push(*cycles.last().unwrap() + val);
            }
        }

        let mut crt_output = "\n".to_string();
        for line in cycles[0..cycles.len() - 1].chunks(40) {
            let mut rendered_line = "".to_string();
            for (i, pixel) in line.iter().enumerate() {
                let sprite = [*pixel, *pixel+1, *pixel+2];
                if sprite.contains(&(i as i32 + 1)) {
                    rendered_line += "#";
                } else {
                    rendered_line += ".";
                }
            }
            crt_output += rendered_line.as_str();
            crt_output += "\n";
        }
        format!("{}", crt_output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}