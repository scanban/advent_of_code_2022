#![allow(dead_code, unused_mut, unused_variables)]

use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore_robot_cost_ore: i32,
    clay_robot_cost_ore: i32,
    //
    obsidian_robot_cost_ore: i32,
    obsidian_robot_cost_clay: i32,
    //
    geode_robot_cost_ore: i32,
    geode_robot_cost_obsidian: i32,
}

const RUN_TIME: i32 = 24;

fn blueprint_process2(blueprint: &Blueprint) -> i32 {
    #[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
    struct State {
        resources_ore: i32,
        resources_clay: i32,
        resources_obsidian: i32,
        //
        robots_ore: i32,
        robots_clay: i32,
        robots_obsidian: i32,
        robots_geode: i32,
        geodes: i32,
        ticks: i32,
    }

    let mut queue = VecDeque::<State>::new();
    queue.push_back(State {
        resources_ore: 0,
        resources_clay: 0,
        resources_obsidian: 0,
        robots_ore: 1,
        robots_clay: 0,
        robots_obsidian: 0,
        robots_geode: 0,
        geodes: 0,
        ticks: 0,
    });

    let mut max_geodes = i32::MIN;
    let mut cache = HashSet::<State>::new();

    while !queue.is_empty() {
        let mut current_state = queue.pop_front().unwrap();

        if current_state.ticks >= RUN_TIME {
            //if current_state.geodes > max_geodes { println!("{:?}", current_state); }
            //max_geodes = max(max_geodes, current_state.geodes);
        } else if cache.contains(&current_state) {
            continue;
        } else {
            //cache.insert(current_state);
            /*            if current_state.geodes > max_geodes { println!("{:?}", current_state); }
                        max_geodes = max(max_geodes, current_state.geodes);
            */
            let queue_len = queue.len();

            // ore bot
            let ticks_needed = 1 + if current_state.resources_ore >= blueprint.ore_robot_cost_ore {
                0
            } else {
                let resource_delta = blueprint.ore_robot_cost_ore - current_state.resources_ore;
                let per_tick_produce = current_state.robots_ore;
                (resource_delta + per_tick_produce - 1) / per_tick_produce
            };
            if current_state.ticks + ticks_needed < RUN_TIME {
                queue.push_front(State {
                    resources_ore: current_state.resources_ore + current_state.robots_ore * ticks_needed - blueprint.ore_robot_cost_ore,
                    resources_clay: current_state.resources_clay + current_state.robots_clay * ticks_needed,
                    resources_obsidian: current_state.resources_obsidian + current_state.robots_obsidian * ticks_needed,
                    geodes: current_state.geodes + current_state.robots_geode * ticks_needed,
                    //
                    robots_ore: current_state.robots_ore + 1,
                    robots_clay: current_state.robots_clay,
                    robots_obsidian: current_state.robots_obsidian,
                    robots_geode: current_state.robots_geode,
                    //
                    ticks: current_state.ticks + ticks_needed,
                })
            }
            // clay bot
            let ticks_needed = 1 + if current_state.resources_ore >= blueprint.clay_robot_cost_ore {
                0
            } else {
                let resource_delta = blueprint.clay_robot_cost_ore - current_state.resources_ore;
                let per_tick_produce = current_state.robots_ore;
                (resource_delta + per_tick_produce - 1) / per_tick_produce
            };
            if current_state.ticks + ticks_needed < RUN_TIME {
                queue.push_front(State {
                    resources_ore: current_state.resources_ore + current_state.robots_ore * ticks_needed - blueprint.clay_robot_cost_ore,
                    resources_clay: current_state.resources_clay + current_state.robots_clay * ticks_needed,
                    resources_obsidian: current_state.resources_obsidian + current_state.robots_obsidian * ticks_needed,
                    geodes: current_state.geodes + current_state.robots_geode * ticks_needed,
                    //
                    robots_ore: current_state.robots_ore,
                    robots_clay: current_state.robots_clay + 1,
                    robots_obsidian: current_state.robots_obsidian,
                    robots_geode: current_state.robots_geode,
                    //
                    ticks: current_state.ticks + ticks_needed,
                })
            }
            if current_state.robots_clay > 0 {
                // obsidian bot
                let ticks_needed = 1 + max(if current_state.resources_ore >= blueprint.obsidian_robot_cost_ore {
                    0
                } else {
                    let resource_delta = blueprint.obsidian_robot_cost_ore - current_state.resources_ore;
                    let per_tick_produce = current_state.robots_ore;
                    (resource_delta + per_tick_produce - 1) / per_tick_produce
                }, if current_state.resources_clay >= blueprint.obsidian_robot_cost_clay {
                    0
                } else {
                    let resource_delta = blueprint.obsidian_robot_cost_clay - current_state.resources_clay;
                    let per_tick_produce = current_state.robots_clay;
                    (resource_delta + per_tick_produce - 1) / per_tick_produce
                });
                if current_state.ticks + ticks_needed < RUN_TIME {
                    queue.push_front(State {
                        resources_ore: current_state.resources_ore + current_state.robots_ore * ticks_needed - blueprint.obsidian_robot_cost_ore,
                        resources_clay: current_state.resources_clay + current_state.robots_clay * ticks_needed - blueprint.obsidian_robot_cost_clay,
                        resources_obsidian: current_state.resources_obsidian + current_state.robots_obsidian * ticks_needed,
                        geodes: current_state.geodes + current_state.robots_geode * ticks_needed,
                        //
                        robots_ore: current_state.robots_ore,
                        robots_clay: current_state.robots_clay,
                        robots_obsidian: current_state.robots_obsidian + 1,
                        robots_geode: current_state.robots_geode,
                        //
                        ticks: current_state.ticks + ticks_needed,
                    })
                }
            }
            // geode bot
            if current_state.robots_obsidian > 0 {
                let ticks_needed = 1 + max(if current_state.resources_ore >= blueprint.geode_robot_cost_ore {
                    0
                } else {
                    let resource_delta = blueprint.geode_robot_cost_ore - current_state.resources_ore;
                    let per_tick_produce = current_state.robots_ore;
                    (resource_delta + per_tick_produce - 1) / per_tick_produce
                }, if current_state.resources_obsidian >= blueprint.geode_robot_cost_obsidian {
                    0
                } else {
                    let resource_delta = blueprint.geode_robot_cost_obsidian - current_state.resources_obsidian;
                    let per_tick_produce = current_state.robots_obsidian;
                    (resource_delta + per_tick_produce - 1) / per_tick_produce
                });
                if current_state.ticks + ticks_needed < RUN_TIME {
                    queue.push_front(State {
                        resources_ore: current_state.resources_ore + current_state.robots_ore * ticks_needed - blueprint.geode_robot_cost_ore,
                        resources_clay: current_state.resources_clay + current_state.robots_clay * ticks_needed,
                        resources_obsidian: current_state.resources_obsidian + current_state.robots_obsidian * ticks_needed - blueprint.geode_robot_cost_obsidian,
                        geodes: current_state.geodes + current_state.robots_geode * ticks_needed,
                        //
                        robots_ore: current_state.robots_ore,
                        robots_clay: current_state.robots_clay,
                        robots_obsidian: current_state.robots_obsidian,
                        robots_geode: current_state.robots_geode + 1,
                        //
                        ticks: current_state.ticks + ticks_needed,
                    })
                }
            }

            if queue_len == queue.len() {
                let geodes = current_state.geodes + current_state.robots_geode * (RUN_TIME - current_state.ticks);
                if geodes == 9
                    && current_state.robots_ore == 1
                    && current_state.robots_clay == 4
                    && current_state.robots_obsidian == 2
                    && current_state.robots_geode == 2 {
                    println!("obsidian:{:?}", current_state);
                }
                max_geodes = max(max_geodes, geodes);
            }
        }
        //
    }

    max_geodes
}

fn solve_problem(input_data: &str) -> i32 {
    lazy_static! {
        static ref RE_BLUEPRINT: Regex = Regex::new(
            r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$"
        )
        .unwrap();

    }
    let lines = input_data.lines();

    let blueprints = lines.map(|s| {
        let c = RE_BLUEPRINT.captures(s).unwrap();
        let parse = |id| { c.get(id).unwrap().as_str().parse().unwrap() };
        Blueprint {
            id: parse(1),
            ore_robot_cost_ore: parse(2),
            clay_robot_cost_ore: parse(3),
            //
            obsidian_robot_cost_ore: parse(4),
            obsidian_robot_cost_clay: parse(5),
            //
            geode_robot_cost_ore: parse(6),
            geode_robot_cost_obsidian: parse(7),
        }
    }).collect::<Vec<Blueprint>>();

    let mut result = 0;
    for b in &blueprints {
        let bp_geodes = blueprint_process2(b);
        println!("BP {}: {}", b.id, bp_geodes);
        result += b.id * bp_geodes;
    }

    result
}

pub fn solve() -> i32 {
    solve_problem(include_str!("../input.txt"))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve_problem(include_str!("../input_test.txt"));
        assert_eq!(result, 33);
    }
}
