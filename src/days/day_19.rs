use regex::Regex;
use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayNineteen {}

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub id: u32,
    pub ore_bot: [u32; 1],
    pub clay_bot: [u32; 1],
    pub obsidian_bot: [u32; 2],
    pub geode_bot: [u32; 2],
}

#[derive(Debug, Clone)]
pub struct State {
    pub time: u32,
    pub ore: u32,
    pub clay: u32,
    pub obsidian: u32,
    pub geode: u32,
    pub n_ore_bots: u32,
    pub n_clay_bots: u32,
    pub n_obsidian_bots: u32,
    pub n_geode_bots: u32,
}

impl Blueprint {
    pub fn next_move(&mut self, mut state: State, mut do_not_buy: [bool; 4]) -> u32 {
        let mut geode = 0;
        if state.time == 0 {
            return state.geode;
        }
        state.time -= 1;

        let ore_threshold = *[self.ore_bot[0], self.clay_bot[0], self.obsidian_bot[0], self.geode_bot[0]].iter().max().unwrap();

        if !do_not_buy[0] && state.ore >= self.ore_bot[0] && state.n_ore_bots <= ore_threshold {
            let mut next_state = state.clone();
            let mut next_do_not_buy = [false; 4];

            next_state.ore += next_state.n_ore_bots;
            next_state.clay += next_state.n_clay_bots;
            next_state.obsidian += next_state.n_obsidian_bots;
            next_state.geode += next_state.n_geode_bots;

            next_state.ore -= self.ore_bot[0];
            next_state.n_ore_bots += 1;
            geode = geode.max(self.next_move(next_state, next_do_not_buy));
        }
        if !do_not_buy[1] && state.ore >= self.clay_bot[0] {
            let mut next_state = state.clone();
            let mut next_do_not_buy = [false; 4];

            next_state.ore += next_state.n_ore_bots;
            next_state.clay += next_state.n_clay_bots;
            next_state.obsidian += next_state.n_obsidian_bots;
            next_state.geode += next_state.n_geode_bots;

            next_state.ore -= self.clay_bot[0];
            next_state.n_clay_bots += 1;

            geode = geode.max(self.next_move(next_state, next_do_not_buy));
        }
        if !do_not_buy[2] && state.ore >= self.obsidian_bot[0] &&
            state.clay >= self.obsidian_bot[1]  {
            let mut next_state = state.clone();
            let mut next_do_not_buy = [false; 4];

            next_state.ore += next_state.n_ore_bots;
            next_state.clay += next_state.n_clay_bots;
            next_state.obsidian += next_state.n_obsidian_bots;
            next_state.geode += next_state.n_geode_bots;

            next_state.ore -= self.obsidian_bot[0];
            next_state.clay -= self.obsidian_bot[1];
            next_state.n_obsidian_bots += 1;
            geode = geode.max(self.next_move(next_state, next_do_not_buy));
        }
        if !do_not_buy[3] && state.ore >= self.geode_bot[0] &&
            state.obsidian >= self.geode_bot[1] {
            let mut next_state = state.clone();
            let mut next_do_not_buy = [false; 4];

            next_state.ore += next_state.n_ore_bots;
            next_state.clay += next_state.n_clay_bots;
            next_state.obsidian += next_state.n_obsidian_bots;
            next_state.geode += next_state.n_geode_bots;

            next_state.ore -= self.geode_bot[0];
            next_state.obsidian -= self.geode_bot[1];
            next_state.n_geode_bots += 1;
            geode = geode.max(self.next_move(next_state, next_do_not_buy));
        }
        let mut next_do_not_buy = [false; 4];

        if state.ore >= self.ore_bot[0] {
            next_do_not_buy[0] = true;
        }
        if state.ore >= self.clay_bot[0] {
            next_do_not_buy[1] = true;
        }
        if state.ore >= self.obsidian_bot[0] &&
            state.clay >= self.obsidian_bot[1] {
            next_do_not_buy[2] = true;
        }
        if state.ore >= self.geode_bot[0] &&
            state.obsidian >= self.geode_bot[1] {
            next_do_not_buy[3] = true;
        }
        state.ore += state.n_ore_bots;
        state.clay += state.n_clay_bots;
        state.obsidian += state.n_obsidian_bots;
        state.geode += state.n_geode_bots;

        geode.max(self.next_move(state, next_do_not_buy))
    }

    pub fn run(&mut self, time: u32) -> u32 {
        let state = State {
            time,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            n_ore_bots: 1,
            n_clay_bots: 0,
            n_obsidian_bots: 0,
            n_geode_bots: 0,
        };
        let geode = self.next_move(state, [false; 4]);
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
                .collect::<Vec<u32>>();
            let blueprint = Blueprint {
                id: vals[0],
                ore_bot: [vals[1]],
                clay_bot: [vals[2]],
                obsidian_bot: [vals[3], vals[4]],
                geode_bot: [vals[5], vals[6]],
            };
            blueprints.push(blueprint);
        }
        let mut quality_levels = 0;
        for blueprint in blueprints.iter_mut() {
            let num_geodes = blueprint.run(24);
            let id = blueprint.id;
            quality_levels += num_geodes * id;
        }

        format!("{:#?}", quality_levels)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let re = Regex::new(r"[0-9]*").unwrap();
        let mut blueprints = vec![];
        for blueprint_line in contents.iter() {
            let vals = re.find_iter(blueprint_line)
                .filter_map(|digits| digits.as_str().parse().ok())
                .collect::<Vec<u32>>();
            let blueprint = Blueprint {
                id: vals[0],
                ore_bot: [vals[1]],
                clay_bot: [vals[2]],
                obsidian_bot: [vals[3], vals[4]],
                geode_bot: [vals[5], vals[6]],
            };
            blueprints.push(blueprint);
        }
        let mut prod = 1;
        for i in 0..3 {
            let num_geodes = blueprints[i].run(32);
            println!("found {} with {}", blueprints[i].id, num_geodes);
            prod *= num_geodes as u64;
        }
        format!("{}", prod)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}