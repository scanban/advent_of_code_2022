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

fn solve_problem(input_data: &str) -> i64 {
    lazy_static! {
        static ref RE_SENSORS: Regex = Regex::new(
            r"^Sensor at x=([--\d]+), y=([--\d]+): closest beacon is at x=([--\d]+), y=([--\d]+)$"
        )
        .unwrap();
    }
    let lines = input_data.lines();

    let sensors = lines
        .map(|l| {
            let c = RE_SENSORS.captures(l).unwrap();
            let ret = Sensor {
                x: c.get(1).unwrap().as_str().parse().unwrap(),
                y: c.get(2).unwrap().as_str().parse().unwrap(),
                beacon_x: c.get(3).unwrap().as_str().parse().unwrap(),
                beacon_y: c.get(4).unwrap().as_str().parse().unwrap(),
            };
            ret
        })
        .collect::<Vec<Sensor>>();


    let mut zones = Vec::<(i32, i32)>::new();

    let max_x_coord: i32;
    let max_y_coord: i32;

    if cfg!(test) {
        max_x_coord = 20;
        max_y_coord = 20;
    } else {
        max_x_coord = 4000000;
        max_y_coord = 4000000;
    }

    let mut t_pos_x_left = i32::MAX;
    let mut t_pos_x_right = i32::MIN;

    for y_level in 0..=max_y_coord {
        zones.clear();
        for sensor in &sensors {
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
        zones.sort_unstable_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
        if cfg!(test) {
            dbg!(&zones);
        }

        if !zones.is_empty() {
            if zones[0].0 > 0 {
                println!("found 0:{} -> ", y_level);
                return y_level as i64;
            }
            let mut z_x_right = zones[0].1;
            for i in 1..zones.len() {
                if z_x_right > max_x_coord { break; }
                if z_x_right < zones[i].0 {
                    let result = (z_x_right + 1) as i64 * 4_000_000 + y_level as i64;
                    return result;
                }
                if z_x_right < zones[i].1 { z_x_right = zones[i].1; }
            }
        }
    }
    0
}

pub fn solve() -> i64 {
    solve_problem(include_str!("../input.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve_problem(include_str!("../input_test.txt"));
        assert_eq!(result, 26);
    }
}
