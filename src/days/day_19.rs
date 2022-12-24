use regex::Regex;
use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayNineteen {}

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub id: i32,
    pub ore_bot: [i32; 1],
    pub clay_bot: [i32; 1],
    pub obsidian_bot: [i32; 2],
    pub geode_bot: [i32; 2],
}

impl Blueprint {
    pub fn run(&mut self, time: i32) -> i32 {
        let mut ore = 0;
        let mut clay = 0;
        let mut obsidian = 0;
        let mut geode = 0;
        let mut ore_bots = 1;
        let mut clay_bots = 0;
        let mut obsidian_bots = 0;
        let mut geode_bots = 0;

        for t in 0..time {
            let mut ore_bots_temp = ore_bots;
            let mut clay_bots_temp = clay_bots;
            let mut obsidian_bots_temp = obsidian_bots;
            let mut geode_bots_temp = geode_bots;

            let steps_to_ore_bot = self.ore_bot[0];
            let steps_to_clay_bot = self.clay_bot[0];
            let mut steps_to_obs_bot = 0;
            let mut steps_to_geo_bot = 0;
            if clay_bots == 0 {
                steps_to_obs_bot = self.obsidian_bot[1] - clay;
            } else {
                steps_to_obs_bot = ((self.obsidian_bot[1] - clay) / clay_bots).max((self.obsidian_bot[0] - ore) / ore_bots)
            }
            if obsidian_bots == 0 {
                steps_to_geo_bot = self.geode_bot[1] - obsidian;
            } else {
                steps_to_geo_bot = ((self.geode_bot[1] - obsidian) / obsidian_bots).max((self.geode_bot[0] - ore) / ore_bots);
            }

            println!("steps_to_geo_bot: {steps_to_geo_bot}");
            println!("steps_to_obs_bot: {steps_to_obs_bot}");
            println!("steps_to_clay_bot: {steps_to_clay_bot}");
            println!("steps_to_ore_bot: {steps_to_ore_bot}");
            if ore >= self.geode_bot[0] &&
                obsidian >= self.geode_bot[1] {
                ore -= self.geode_bot[0];
                obsidian -= self.geode_bot[1];
                geode_bots_temp += 1;
            } else if ore >= self.obsidian_bot[0] &&
                clay >= self.obsidian_bot[1] &&
                steps_to_obs_bot < steps_to_geo_bot {
                ore -= self.obsidian_bot[0];
                clay -= self.obsidian_bot[1];
                obsidian_bots_temp += 1;
            } else if ore >= self.clay_bot[0] &&
                clay < self.obsidian_bot[1] &&
                steps_to_clay_bot < steps_to_obs_bot {
                ore -= self.clay_bot[0];
                clay_bots_temp += 1;
            } else if ore >= self.ore_bot[0] &&
                steps_to_ore_bot < steps_to_clay_bot {
                ore -= self.ore_bot[0];
                ore_bots_temp += 1;
            }


            for _ in 0..ore_bots {
                ore += 1;
            }
            for _ in 0..clay_bots {
                clay += 1;
            }
            for _ in 0..obsidian_bots {
                obsidian += 1;
            }
            for _ in 0..geode_bots {
                geode += 1;
            }
            ore_bots = ore_bots_temp;
            clay_bots = clay_bots_temp;
            obsidian_bots = obsidian_bots_temp;
            geode_bots = geode_bots_temp;
            println!("round {}", t + 1);
            println!("ore_bots: {}", ore_bots);
            println!("ore: {}", ore);
            println!("clay_bots: {}", clay_bots);
            println!("clay: {}", clay);
            println!("obsidian_bots: {}", obsidian_bots);
            println!("obsidian: {}", obsidian);
            println!("geode_bots: {}", geode_bots);
            println!("geode: {}", geode);
            println!("");
        }
        geode
    }
}

impl Problem for DayNineteen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let re = Regex::new(r"[0-9]*").unwrap();
        let mut blueprints = vec![];
        for blueprint_line in contents.iter() {
            let vals = re.find_iter(blueprint_line)
                .filter_map(|digits| digits.as_str().parse().ok())
                .collect::<Vec<i32>>();
            let blueprint = Blueprint {
                id: vals[0],
                ore_bot: [vals[1]],
                clay_bot: [vals[2]],
                obsidian_bot: [vals[3], vals[4]],
                geode_bot: [vals[5], vals[6]],
            };
            blueprints.push(blueprint);
        }
        let mut geode_num = vec![];
        for blueprint in blueprints.iter_mut() {
            geode_num.push(blueprint.run(24));
        }

        format!("{:#?}", geode_num)
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}