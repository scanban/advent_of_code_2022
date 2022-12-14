#![allow(dead_code, unused_mut, unused_variables)]

use std::cmp::{max, min};

const SZ_X: usize = 1000;
const SZ_Y: usize = 1000;

fn parse_element(e: &str) -> (usize, usize) {
    let mut es = e.split(',');
    (es.next().unwrap().parse().unwrap(), es.next().unwrap().parse().unwrap())
}

fn print_pane(p: &[[u8; SZ_Y]; SZ_X], min_x: usize, max_x: usize, max_y: usize) {
    for row in 0..=max_y {
        for col in min_x..=max_x {
            if p[col][row] == 1 { print!("#"); }
            else if p[col][row] == 2 { print!("o"); }
            else { print!("."); }
        }
        println!();
    }
}

fn solve_problem(input_data: &str) -> i32 {
    let mut lines = input_data.lines();
    let mut result = 0;
    let mut min_x: usize = usize::MAX;
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    let mut pane = [[0u8; SZ_Y]; SZ_X];

    for line in lines {
        let mut elements = line.split(" -> ");
        let (mut current_x, mut current_y) = parse_element(elements.next().unwrap());

        pane[current_x][current_y] = 1;

        elements.for_each(|e| {
            let (dest_x, dest_y) = parse_element(e);

            for i in min(current_y, dest_y)..=max(current_y, dest_y) {
                max_y = max(max_y, i);
                pane[current_x][i] = 1;
            }
            current_y = dest_y;
            for i in min(current_x, dest_x)..=max(current_x, dest_x) {
                max_x = max(max_x, i);
                min_x = min(min_x, i);
                pane[i][current_y] = 1;
            }
            current_x = dest_x;
        });
    }

    max_y += 2;

    for i in 0..SZ_X {
        pane[i][max_y] = 1;
    }

    if cfg!(test) {
        print_pane(&pane, min_x, max_x, max_y);
    }

    loop {
        let mut sand_x = 500;
        let mut sand_y = 0;

        loop {
            if pane[500][0] == 2 { return result; }
            if pane[sand_x][sand_y + 1] == 0 { sand_y += 1; continue; }
            if pane[sand_x - 1][sand_y + 1] == 0 { sand_x -= 1; sand_y += 1; continue; }
            if pane[sand_x + 1][sand_y + 1] == 0 { sand_x += 1; sand_y += 1; continue; }
            pane[sand_x][sand_y] = 2;
            break;
        }
        if cfg!(test) {
            print_pane(&pane, min_x, max_x, max_y);
        }
        result += 1;
    }
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
        assert_eq!(result, 93);
    }
}
