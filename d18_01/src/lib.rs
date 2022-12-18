use std::collections::BTreeSet;

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

fn solve_problem(input_data: &str) -> i32 {
    let lines = input_data.lines();

    let cubes = lines.map(|l| {
        let mut s = l.split(',');
        Cube {
            x: s.next().unwrap().parse().unwrap(),
            y: s.next().unwrap().parse().unwrap(),
            z: s.next().unwrap().parse().unwrap(),
        }
    }).collect::<Vec<Cube>>();

    let cubes_idx = cubes.iter().collect::<BTreeSet<&Cube>>();

    let mut open_sides = 0;

    for c in &cubes {
        if !cubes_idx.contains(&Cube{x: c.x + 1, y: c.y, z: c.z}) {open_sides += 1; }
        if !cubes_idx.contains(&Cube{x: c.x - 1, y: c.y, z: c.z}) {open_sides += 1; }
        if !cubes_idx.contains(&Cube{x: c.x, y: c.y + 1, z: c.z}) {open_sides += 1; }
        if !cubes_idx.contains(&Cube{x: c.x, y: c.y - 1, z: c.z}) {open_sides += 1; }
        if !cubes_idx.contains(&Cube{x: c.x, y: c.y, z: c.z + 1}) {open_sides += 1; }
        if !cubes_idx.contains(&Cube{x: c.x, y: c.y, z: c.z - 1}) {open_sides += 1; }
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
        assert_eq!(result, 64);
    }
}
