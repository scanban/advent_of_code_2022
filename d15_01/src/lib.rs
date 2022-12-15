#![allow(dead_code, unused_mut, unused_variables)]

use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp::{max, min},
};

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

fn solve_problem(y_level: i32, input_data: &str) -> i32 {
    lazy_static! {
        static ref RE_SENSORS: Regex = Regex::new(
            r"^Sensor at x=([--\d]+), y=([--\d]+): closest beacon is at x=([--\d]+), y=([--\d]+)$"
        )
        .unwrap();
    }
    let mut lines = input_data.lines();
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    let sensors = lines
        .map(|l| {
            let c = RE_SENSORS.captures(l).unwrap();
            let ret = Sensor {
                x: c.get(1).unwrap().as_str().parse().unwrap(),
                y: c.get(2).unwrap().as_str().parse().unwrap(),
                beacon_x: c.get(3).unwrap().as_str().parse().unwrap(),
                beacon_y: c.get(4).unwrap().as_str().parse().unwrap(),
            };
            min_x = min(min_x, ret.x);
            min_x = min(min_x, ret.beacon_x);
            min_y = min(min_y, ret.y);
            min_y = min(min_y, ret.beacon_y);

            max_x = max(max_x, ret.x);
            max_x = max(max_x, ret.beacon_x);
            max_y = max(max_y, ret.y);
            max_y = max(max_y, ret.beacon_y);

            ret
        })
        .collect::<Vec<Sensor>>();

    let mut t_pos_x_left = i32::MAX;
    let mut t_pos_x_right = i32::MIN;

    let mut zones = Vec::<(i32, i32)>::new();

    for sensor in sensors {
        let s_max = (sensor.x - sensor.beacon_x).abs() + (sensor.y - sensor.beacon_y).abs();
        if (sensor.y - y_level).abs() > s_max {
            continue;
        }
        let pos_x_left = sensor.x - (s_max - (sensor.y - y_level).abs());
        let pos_x_right = sensor.x + (s_max - (sensor.y - y_level).abs());
        t_pos_x_left = min(t_pos_x_left, pos_x_left);
        t_pos_x_right = max(t_pos_x_right, pos_x_right);
        zones.push((pos_x_left, pos_x_right));
    }

    if cfg!(test) {
        dbg!(&zones);
    }

    let mut result = -1;
    for i in t_pos_x_left..=t_pos_x_right {
        if zones.iter().any(|v| i >= v.0 && i <= v.1) {
            result += 1;
        }
    }

    result
}

pub fn solve() -> i32 {
    solve_problem(2000000, include_str!("../input.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve_problem(10, include_str!("../input_test.txt"));
        assert_eq!(result, 26);
    }
}
