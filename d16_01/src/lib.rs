#![allow(dead_code, unused_mut, unused_variables)]

use std::cmp::max;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: i32,
    lead_to: Vec<&'a str>,
    distances: BTreeMap<String, i32>,
}


fn bfs(src: &str, dst: &str, valves: &BTreeMap<&str, Valve>) -> i32 {
    struct Node<'a> {
        name: &'a str,
        distance: i32,
    }

    let mut visited = BTreeSet::<&str>::new();
    let mut queue = VecDeque::<Node>::new();

    queue.push_back(Node { name: src, distance: 0 });
    visited.insert(src);

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        if node.name.eq(dst) {
            return node.distance;
        }
        for v in valves.get(node.name).unwrap().lead_to.iter() {
            if visited.contains(v) { continue; }
            queue.push_back(Node { name: v, distance: node.distance + 1 });
            visited.insert(v);
        }
    }
    0
}

const START_NODE: &str = "AA";

fn calc_permutation(order: &[&str], valves: &BTreeMap<&str, Valve>) -> i32 {
    let mut ret = 0;
    let mut ticks = 30;
    let mut start_node = START_NODE;

    if cfg!(test) {
        println!("{:?}", order);
    }

    for i in 0..order.len() {
        let current_node = order[i];

        let travel_open_ticks = valves.get(start_node).unwrap()
            .distances.get(current_node).unwrap() + 1;
        ticks -= travel_open_ticks;
        if ticks <= 0 {
            break;
        }
        ret += ticks * valves.get(current_node).unwrap().flow_rate;

        start_node = current_node;
    }
    ret
}

fn fill_distances(valve: &str, valves: &BTreeMap<&str, Valve>) -> BTreeMap<String, i32> {
    let mut ret = BTreeMap::<String, i32>::new();

    valves.iter().filter(|&(&k, v)| !k.eq(valve))
        .map(|(&k, v)| (k.to_string(), (bfs(valve, k, valves))))
        .collect()
}

fn permutate2(n:usize, valve_names: &mut Vec<&str>, valves: &BTreeMap<&str, Valve>) -> i32 {
    let mut translation = valves.iter()
        .map(|(&k, v)| k)
        .enumerate()
        .map(|(i, n)| (n, i as u8)).collect::<BTreeMap<&str, u8>>();

    let mut flows = Vec::<i32>::new();
    flows.resize(translation.len(), 0);
    translation.iter().for_each(|(&k, v)| {
        flows[*v as usize] = valves[k].flow_rate;
    });

    let mut valve_ids = valve_names.iter().map(|&k| translation[k]).collect::<Vec<u8>>();


    let mut distances = Vec::<Vec<i32>>::new();
    distances.resize(translation.len(), Vec::<i32>::new());
    translation.iter().for_each(|(&k, v)| {
        let e = & mut distances[translation[k] as usize];
        let valve = &valves[k];

        e.resize(translation.len(), 0);
        valve.distances.iter().for_each(|(k, v)|{
            e[translation[k.as_str()] as usize] = *v;
        })
    });

    let mut stack = (0..n).map(|_| 0).collect::<Vec<usize>>();
    let mut sp = 1usize;
    let mut p = 0u64;

    let mut start_node = translation[START_NODE];
    let mut max_pressure = 0;

    while sp < n {
        if stack[sp] < sp {
            if sp & 1 == 0 {
                valve_ids.swap(0, sp);
            } else {
                valve_ids.swap(stack[sp], sp);
            }

            let mut ticks = 30;
            let mut pressure = 0;
            for i in 0..valve_ids.len() {
                let current_node = valve_ids[i];
                let travel_open_ticks = distances[start_node as usize][current_node as usize] + 1;
                ticks -= travel_open_ticks;
                if ticks <= 0 {
                    break;
                }
                pressure += flows[current_node as usize] * ticks;
                start_node = current_node;
            }
            max_pressure = max(max_pressure, pressure);
            p += 1;
            if p % 100_000_000 == 0 {
                println!("{}", p);
            }
            stack[sp] += 1;
            sp = 1;
        } else {
            stack[sp] = 0;
            sp += 1;
        }
    }

    max_pressure
}

fn permutate(n: usize, valve_names: &mut Vec<&str>, valves: &BTreeMap<&str, Valve>) -> i32 {
    let mut stack = (0..n).map(|_| 0).collect::<Vec<usize>>();
    let mut sp = 1usize;
    let mut p = 0u64;

    let mut max_pressure = calc_permutation(&valve_names, valves);

    while sp < n {
        if stack[sp] < sp {
            if sp & 1 == 0 {
                valve_names.swap(0, sp);
            } else {
                valve_names.swap(stack[sp], sp);
            }
            max_pressure = max(max_pressure, calc_permutation(&valve_names, valves));
            p += 1;
            if p % 100_000_000 == 0 {
                println!("{}", p);
            }
            stack[sp] += 1;
            sp = 1;
        } else {
            stack[sp] = 0;
            sp += 1;
        }
    }
    max_pressure
}

fn solve_problem(input_data: &str) -> i32 {
    lazy_static! {
        static ref RE_VALVES: Regex = Regex::new(
            r"^Valve ([A-Z]{2}) has flow rate=(\d+); \w+ \w+ to \w+ ((?:[A-Z]{2}, )*[A-Z]{2})$"
        )
        .unwrap();

    }
    let lines = input_data.lines();

    let mut valves = lines.map(|l| {
        let c = RE_VALVES.captures(l).unwrap();
        let name = c.get(1).unwrap().as_str();
        (name, Valve {
            name,
            flow_rate: c.get(2).unwrap().as_str().parse().unwrap(),
            lead_to: c.get(3).unwrap().as_str().split(", ").collect::<Vec<&str>>(),
            distances: BTreeMap::new(),
        })
    }).collect::<BTreeMap<&str, Valve>>();

    for v in valves.iter().map(|(&k, v)| k).collect::<Vec<&str>>() {
        valves.get_mut(v).unwrap().distances = fill_distances(v, &valves);
    }

    //dbg!(&valves);

    let mut orders = Vec::<&str>::new();
    valves.iter().filter(|(&k, v)|{ v.flow_rate != 0 })
        .for_each(|(&k, v)| orders.push(k));

    permutate2(orders.len(), &mut orders, &valves)
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
        assert_eq!(result, 1651);
    }
}
