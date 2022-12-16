use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
    iter,
};

use anyhow::{Context, Result};
use bitvec::prelude::*;
use solve::Input;

#[derive(Clone, Debug, Default)]
struct Valve {
    flow_rate: usize,
    tunnels: Vec<(usize, usize)>,
}

#[derive(Clone, Debug)]
struct Graph {
    valves: Vec<Valve>,
    start: usize,
}

impl Graph {
    fn densify(&mut self) {
        for i in 0..self.valves.len() {
            if i != self.start && self.valves[i].flow_rate == 0 {
                // Remove this valve and directly connect its neighbors
                let tunnels = core::mem::replace(&mut self.valves[i].tunnels, Vec::new());
                for &(a, a_dist) in tunnels.iter() {
                    // Remove this valve from its neighbor
                    self.valves[a].tunnels.retain(|(to, _)| *to != i);
                    // Update the distances it has to this valve's neighbors
                    for &(b, b_dist) in tunnels.iter().filter(|(to, _)| *to != a) {
                        if let Some(dist) = self.valves[a]
                            .tunnels
                            .iter_mut()
                            .find_map(|(to, dist)| (*to == b).then(|| dist))
                        {
                            *dist = usize::min(*dist, a_dist + b_dist);
                        } else {
                            self.valves[a].tunnels.push((b, a_dist + b_dist));
                        }
                    }
                }
            }
        }
    }
}

impl Input for Graph {
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        let mut valves = Vec::new();
        let mut start = None;
        let mut label_to_index = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let (label, line) = line
                .strip_prefix("Valve ")
                .context("expected valve prefix")?
                .split_at(2);
            let mut pieces = line
                .strip_prefix(" has flow rate=")
                .context("expected flow rate prefix")?
                .split("; ");
            let flow_rate = pieces.next().context("expected flow rate")?;
            let flow_rate = flow_rate.parse()?;

            let i = valves.len();
            valves.insert(
                i,
                Valve {
                    flow_rate,
                    tunnels: Vec::new(),
                },
            );
            label_to_index.insert(label.to_string(), i);
            if label == "AA" {
                start = Some(i);
            }

            let mut tunnels = pieces.next().context("expected tunnels")?.split(", ");
            let first = tunnels.next().context("expected one tunnel")?;
            let (_, first) = first.split_at(first.len() - 2);
            for tunnel in iter::once(first).chain(tunnels) {
                if let Some(&to) = label_to_index.get(tunnel) {
                    valves[i].tunnels.push((to, 1));
                    valves[to].tunnels.push((i, 1));
                }
            }
        }

        Ok(Self {
            valves,
            start: start.unwrap(),
        })
    }
}

fn solve_part_one(input: &Graph) -> usize {
    #[derive(Clone, Hash, Eq, PartialEq)]
    struct Key {
        valve: usize,
        valve_states: BitArray<u64, Lsb0>,
    }

    struct State {
        valve: usize,
        valve_states: BitArray<u64, Lsb0>,
        time_left: usize,
        pressure_released: usize,
    }

    let mut graph = input.clone();
    graph.densify();

    const MINUTES: usize = 30;

    let mut queue = VecDeque::new();
    queue.push_back(State {
        valve: graph.start,
        valve_states: BitArray::ZERO,
        time_left: MINUTES,
        pressure_released: 0,
    });

    // "If you're at `valve` with `valve_states` open and x minutes left, the
    // most total pressure released is y"
    let mut visited = HashMap::<Key, [usize; MINUTES + 1]>::new();

    let mut max_pressure_released = 0;
    while let Some(current) = queue.pop_front() {
        let key = Key {
            valve: current.valve,
            valve_states: current.valve_states,
        };
        let can_skip = visited.contains_key(&key);
        let best_at_minute = visited.entry(key).or_insert([0; MINUTES + 1]);
        if can_skip && best_at_minute[current.time_left] >= current.pressure_released {
            continue;
        }
        for i in 0..=current.time_left {
            best_at_minute[i] = current.pressure_released;
        }

        // Wait here for the remaining time
        max_pressure_released = usize::max(max_pressure_released, current.pressure_released);

        // Turn on the valve in this room
        if current.time_left > 0
            && !current.valve_states[current.valve]
            && graph.valves[current.valve].flow_rate != 0
        {
            let mut valve_states = current.valve_states.clone();
            valve_states.set(current.valve, true);
            queue.push_back(State {
                valve: current.valve,
                valve_states,
                time_left: current.time_left - 1,
                pressure_released: current.pressure_released
                    + graph.valves[current.valve].flow_rate * (current.time_left - 1),
            });
        }

        for &(to, dist) in graph.valves[current.valve].tunnels.iter() {
            // Move to another valve
            if current.time_left > dist {
                queue.push_back(State {
                    valve: to,
                    valve_states: current.valve_states.clone(),
                    time_left: current.time_left - dist,
                    pressure_released: current.pressure_released,
                });
            }
        }
    }

    max_pressure_released
}

fn solve_part_two(input: &Graph) -> usize {
    #[derive(Clone, Hash, Eq, PartialEq)]
    struct Key {
        valve: usize,
        valve_states: BitArray<u64, Lsb0>,
    }

    struct State {
        valve: usize,
        valve_states: BitArray<u64, Lsb0>,
        time_left: usize,
        pressure_released: usize,
    }

    let mut graph = input.clone();
    graph.densify();

    const MINUTES: usize = 26;

    let mut queue = VecDeque::new();
    queue.push_back(State {
        valve: graph.start,
        valve_states: BitArray::ZERO,
        time_left: MINUTES,
        pressure_released: 0,
    });

    // "If you're at `valve` with `valve_states` open and x minutes left, the
    // most total pressure released is y"
    let mut visited = HashMap::<Key, [usize; MINUTES + 1]>::new();

    let mut max_pressure_released = HashMap::new();
    while let Some(current) = queue.pop_front() {
        let key = Key {
            valve: current.valve,
            valve_states: current.valve_states,
        };
        let can_skip = visited.contains_key(&key);
        let best_at_minute = visited.entry(key).or_insert([0; MINUTES + 1]);
        if can_skip && best_at_minute[current.time_left] >= current.pressure_released {
            continue;
        }
        for i in 0..=current.time_left {
            best_at_minute[i] = current.pressure_released;
        }

        // Wait here for the remaining time
        let best = max_pressure_released
            .entry(current.valve_states)
            .or_insert(0);
        *best = usize::max(*best, current.pressure_released);

        // Turn on the valve in this room
        if current.time_left > 0
            && !current.valve_states[current.valve]
            && graph.valves[current.valve].flow_rate != 0
        {
            let mut valve_states = current.valve_states.clone();
            valve_states.set(current.valve, true);
            queue.push_back(State {
                valve: current.valve,
                valve_states,
                time_left: current.time_left - 1,
                pressure_released: current.pressure_released
                    + graph.valves[current.valve].flow_rate * (current.time_left - 1),
            });
        }

        for &(to, dist) in graph.valves[current.valve].tunnels.iter() {
            // Move to another valve
            if current.time_left > dist {
                queue.push_back(State {
                    valve: to,
                    valve_states: current.valve_states.clone(),
                    time_left: current.time_left - dist,
                    pressure_released: current.pressure_released,
                });
            }
        }
    }

    let mut max_total = 0;
    for (you_valves, you_pressure) in max_pressure_released.iter() {
        for (elephant_valves, elephant_pressure) in max_pressure_released.iter() {
            if *you_valves & *elephant_valves == BitArray::<u64, Lsb0>::ZERO {
                max_total = usize::max(max_total, you_pressure + elephant_pressure);
            }
        }
    }

    max_total
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}
