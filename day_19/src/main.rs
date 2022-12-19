use std::{str::FromStr, collections::{HashSet, VecDeque}};

use anyhow::{Context, Error};

#[derive(Debug)]
struct Blueprint {
    ore_robot_ore: u8,
    clay_robot_ore: u8,
    obsidian_robot_ore: u8,
    obsidian_robot_clay: u8,
    geode_robot_ore: u8,
    geode_robot_obsidian: u8,
}

impl FromStr for Blueprint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("Blueprint ")
            .context("expected blueprint prefix")?;
        let mut pieces = s.split(": ");
        let _ = pieces
            .next()
            .context("expected blueprint number")?
            .parse::<usize>()?;
        let s = pieces.next().context("expected blueprint description")?;
        let mut pieces = s.split(". ");

        let ore_robot_ore = pieces
            .next()
            .context("expected ore robot blueprint")?
            .strip_prefix("Each ore robot costs ")
            .context("expected ore robot prefix")?
            .strip_suffix(" ore")
            .context("expected ore robot suffix")?
            .parse()?;
        let clay_robot_ore = pieces
            .next()
            .context("expected clay robot blueprint")?
            .strip_prefix("Each clay robot costs ")
            .context("expected clay robot prefix")?
            .strip_suffix(" ore")
            .context("expected clay robot suffix")?
            .parse()?;

        let mut obsidian_robot_pieces = pieces
            .next()
            .context("expected obsidian robot blueprint")?
            .strip_prefix("Each obsidian robot costs ")
            .context("expected obsidian robot prefix")?
            .split(" ore and ");
        let obsidian_robot_ore = obsidian_robot_pieces
            .next()
            .context("expected obsidian robot ore cost")?
            .parse()?;
        let obsidian_robot_clay = obsidian_robot_pieces
            .next()
            .context("expected obsidian robot clay cost")?
            .strip_suffix(" clay")
            .context("expected obsidian robot suffix")?
            .parse()?;

        let mut geode_robot_pieces = pieces
            .next()
            .context("expected geode robot blueprint")?
            .strip_prefix("Each geode robot costs ")
            .context("expected geode robot prefix")?
            .split(" ore and ");
        let geode_robot_ore = geode_robot_pieces
            .next()
            .context("expected geode robot ore cost")?
            .parse()?;
        let geode_robot_obsidian = geode_robot_pieces
            .next()
            .context("expected geode robot obsidian cost")?
            .strip_suffix(" obsidian.")
            .context("expected geode robot suffix")?
            .parse()?;

        Ok(Self {
            ore_robot_ore,
            clay_robot_ore,
            obsidian_robot_ore,
            obsidian_robot_clay,
            geode_robot_ore,
            geode_robot_obsidian,
        })
    }
}

#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    time_left: u8,
    geodes: u8,
    ore_robots: u8,
    ore: u8,
    clay_robots: u8,
    clay: u8,
    obsidian_robots: u8,
    obsidian: u8,
}

impl Blueprint {
    fn quality(&self, time: usize) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back(State {
            time_left: time as u8,
            ore_robots: 1,
            ..State::default()
        });

        let mut visited = HashSet::new();

        let mut most_geodes = vec![0; time + 1];
        while let Some(state) = queue.pop_front() {
            // No time left to spend doing anything
            if state.time_left == 0 {
                continue;
            }

            // Already visited this state
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state.clone());

            // The most geodes achievable with `i` minutes left is max(most, current)
            for i in 0..=state.time_left as usize {
                most_geodes[i] = u8::max(most_geodes[i], state.geodes);
            }

            // Max possible geodes is the current number of geodes plus the
            // geodes from building a robot every turn until time is up.
            let max_possible_geodes = state.geodes + state.time_left * (state.time_left - 1) / 2;

            // Bail if this state can't make more geodes than the best state so far
            if max_possible_geodes <= most_geodes[state.time_left as usize] {
                continue;
            }

            // Spend a minute
            let time_left = state.time_left - 1;

            // Passive effects
            let ore = state.ore + state.ore_robots;
            let clay = state.clay + state.clay_robots;
            let obsidian = state.obsidian + state.obsidian_robots;

            // Do nothing
            queue.push_back(State {
                time_left,
                ore,
                clay,
                obsidian,
                ..state
            });

            // Build an ore robot
            if state.ore >= self.ore_robot_ore {
                queue.push_back(State {
                    time_left,
                    ore: ore - self.ore_robot_ore,
                    ore_robots: state.ore_robots + 1,
                    clay,
                    obsidian,
                    ..state
                });
            }

            // Build a clay robot
            if state.ore >= self.clay_robot_ore {
                queue.push_back(State {
                    time_left,
                    ore: ore - self.clay_robot_ore,
                    clay,
                    clay_robots: state.clay_robots + 1,
                    obsidian,
                    ..state
                });
            }

            // Build an obsidian robot
            if state.ore >= self.obsidian_robot_ore && state.clay >= self.obsidian_robot_clay {
                queue.push_back(State {
                    time_left,
                    ore: ore - self.obsidian_robot_ore,
                    clay: clay - self.obsidian_robot_clay,
                    obsidian,
                    obsidian_robots: state.obsidian_robots + 1,
                    ..state
                });
            }

            // Build a geode robot (it produces `time_left` geodes)
            if state.ore >= self.geode_robot_ore && state.obsidian >= self.geode_robot_obsidian {
                queue.push_back(State {
                    time_left,
                    geodes: state.geodes + time_left,
                    ore: ore - self.geode_robot_ore,
                    clay,
                    obsidian: obsidian - self.geode_robot_obsidian,
                    ..state
                });
            }
        }

        most_geodes[0] as usize
    }
}

fn solve_part_one(input: &Vec<Blueprint>) -> usize {
    input.iter().map(|b| b.quality(24)).enumerate().map(|(i, q)| (i + 1) * q).sum()
}

fn solve_part_two(input: &Vec<Blueprint>) -> usize {
    input.iter().take(3).map(|b| b.quality(32)).product()
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}
