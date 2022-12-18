use std::cmp::{max, min};
use std::collections::{BTreeSet, VecDeque};

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq, Hash, Copy, Clone)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

fn solve_problem(input_data: &str) -> i32 {
    let lines = input_data.lines();

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut min_z = i32::MAX;
    let mut max_z = i32::MIN;

    let cubes = lines.map(|l| {
        let mut s = l.split(',');
        let ret = Cube {
            x: s.next().unwrap().parse().unwrap(),
            y: s.next().unwrap().parse().unwrap(),
            z: s.next().unwrap().parse().unwrap(),
        };
        min_x = min(min_x, ret.x);
        max_x = max(max_x, ret.x);
        min_y = min(min_y, ret.y);
        max_y = max(max_y, ret.y);
        min_z = min(min_z, ret.z);
        max_z = max(max_z, ret.z);

        ret
    }).collect::<Vec<Cube>>();

    let cubes_idx = cubes.iter().collect::<BTreeSet<&Cube>>();
    let mut cubes_visited = BTreeSet::<Cube>::new();
    let mut cubes_process_queue = VecDeque::<Cube>::new();

    min_x -= 1;
    max_x += 1;
    min_y -= 1;
    max_y += 1;
    min_z -= 1;
    max_z += 1;

    cubes_process_queue.push_back(Cube { x: min_x, y: min_y, z: min_z });
    cubes_visited.insert(Cube { x: min_x, y: min_y, z: min_z });
    let mut open_sides = 0;

    while !cubes_process_queue.is_empty() {
        let c = cubes_process_queue.pop_front().unwrap();
        
        let mut prc = |c| {
            if !cubes_idx.contains(&c) && !cubes_visited.contains(&c)
                && c.x >= min_x && c.x <= max_x
                && c.y >= min_y && c.y <= max_y
                && c.z >= min_z && c.z <= max_z {
            cubes_process_queue.push_back(c);
            cubes_visited.insert(c);
        }};
        
        if cubes_idx.contains(&Cube { x: c.x + 1, y: c.y, z: c.z }) { open_sides += 1; }
        if cubes_idx.contains(&Cube { x: c.x - 1, y: c.y, z: c.z }) { open_sides += 1; }
        if cubes_idx.contains(&Cube { x: c.x, y: c.y + 1, z: c.z }) { open_sides += 1; }
        if cubes_idx.contains(&Cube { x: c.x, y: c.y - 1, z: c.z }) { open_sides += 1; }
        if cubes_idx.contains(&Cube { x: c.x, y: c.y, z: c.z + 1 }) { open_sides += 1; }
        if cubes_idx.contains(&Cube { x: c.x, y: c.y, z: c.z - 1 }) { open_sides += 1; }

        prc(Cube { x: c.x + 1, y: c.y, z: c.z });
        prc(Cube { x: c.x - 1, y: c.y, z: c.z });
        prc(Cube { x: c.x, y: c.y + 1, z: c.z });
        prc(Cube { x: c.x, y: c.y - 1, z: c.z });
        prc(Cube { x: c.x, y: c.y, z: c.z + 1 });
        prc(Cube { x: c.x, y: c.y, z: c.z - 1 });
    }

    open_sides
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
        assert_eq!(result, 58);
    }
}
