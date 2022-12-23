use std::cmp::max;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Valve<'a> {
    flow_rate: i32,
    lead_to: Vec<&'a str>,
    distances: BTreeMap<String, i32>,
}

#[derive(Debug)]
struct BinaryValve {
    flow_rate: i32,
    distances: BTreeMap<u8, i32>,
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
const SIMULATION_TICKS: i32 = 26;

fn fill_distances(valve: &str, valves: &BTreeMap<&str, Valve>) -> BTreeMap<String, i32> {
    valves.iter().filter(|&(&k, _)| !k.eq(valve))
        .map(|(&k, _)| (k.to_string(), (bfs(valve, k, valves))))
        .collect()
}

fn permutate2(start_node: u8, binary_valves: &BTreeMap<u8, BinaryValve>, open_valves: u64) -> i32 {
    #[derive(Debug)]
    struct State {
        valve_id: u8,
        ticks: i32,
        open_valves: u64,
        pressure: i32,
    }
    let mut max_pressure = 0;
    let mut queue = VecDeque::<State>::new();

    queue.push_back(State { ticks: SIMULATION_TICKS, valve_id: start_node, open_valves, pressure: 0 });

    while !queue.is_empty() {
        let current_node = queue.pop_front().unwrap();
        max_pressure = max(max_pressure, current_node.pressure);

        for (&node_id, distance) in &binary_valves.get(&current_node.valve_id).unwrap().distances {
            if current_node.open_valves & (1u64 << node_id) != 0 { continue; }
            let ticks = current_node.ticks - (distance + 1);
            if ticks <= 0 { continue; }

            let bnode = binary_valves.get(&node_id).unwrap();
            if bnode.flow_rate == 0 { continue; }
            queue.push_back(State {
                ticks,
                valve_id: node_id,
                open_valves: current_node.open_valves | (1u64 << node_id),
                pressure: bnode.flow_rate * ticks + current_node.pressure,
            });
        }
    }

    //dbg!(&cache);

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
            flow_rate: c.get(2).unwrap().as_str().parse().unwrap(),
            lead_to: c.get(3).unwrap().as_str().split(", ").collect::<Vec<&str>>(),
            distances: BTreeMap::new(),
        })
    }).collect::<BTreeMap<&str, Valve>>();

    for v in valves.iter().map(|(&k, _)| k).collect::<Vec<&str>>() {
        valves.get_mut(v).unwrap().distances = fill_distances(v, &valves);
    }

    valves = valves.into_iter()
        .filter(|(k, v)| { k.eq(&START_NODE) || v.flow_rate != 0 }).collect();

    let translation = valves.iter()
        .map(|(&k, _)| k)
        .enumerate()
        .map(|(i, n)| (n, i as u8)).collect::<BTreeMap<&str, u8>>();

    let binary_valves = valves.iter().map(|(&k, v)| {
        let distances = v.distances.iter().filter(|&(k, v)| {
            translation.get(k.as_str()).is_some()
        }).map(|(k, v)| {
            (translation[k.as_str()], *v)
        }).collect::<BTreeMap<u8, i32>>();
        (translation[k], BinaryValve { flow_rate: v.flow_rate, distances })
    }).collect::<BTreeMap<u8, BinaryValve>>();

    //dbg!(&binary_valves);
    println!("{}", binary_valves.len());

    let mut orders = Vec::<&str>::new();
    valves.iter().filter(|(_, v)| { v.flow_rate != 0 })
        .for_each(|(&k, _)| orders.push(k));

    let mut max_pressure = 0;

    // let check all combinations of the initially open valves for me
    // and elephant
    for i in 0..1u64 << binary_valves.len() {
        max_pressure = max(max_pressure,
                           permutate2(translation[START_NODE], &binary_valves, i) +
                               permutate2(translation[START_NODE], &binary_valves, !i),
        );
    }

    max_pressure
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
        assert_eq!(result, 1707);
    }
}
