#![allow(dead_code, unused_mut, unused_variables, unused_must_use)]

use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, Default)]
struct Point {
    row: usize,
    col: usize,
}


// impl Point {
//     fn valid_points(point: &Point, rows: usize, columns: usize) -> Vec<Point> {
//         let mut ret = Vec::<Point>::new();
//         if point.row > 0 && point
//     }
// }

struct Node {
    point: Point,
    distance: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

fn solve_problem(input_data: &str) -> i32 {
    let lines = input_data.lines();
    let mut result = 0;
    let area = lines
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let rows = area.len();
    let cols = area[0].len();
    let mut visited: Vec<Vec<bool>> = Vec::new();

    let mut starting_point = Point::default();
    let mut destination_point = Point::default();
    let mut queue: VecDeque<Node> = VecDeque::new();

    for row in 0..rows {
        for col in 0..cols {
            if area[row][col] == b'S' {
                starting_point = Point { row, col };
            }
            if area[row][col] == b'E' {
                destination_point = Point { row, col };
            }
            visited[row][col] = false;
        }
    }

    let valid_points = |node: &Node| {
        let point = &node.point;

        let valid_heights = |p: &Point| {
            area[p.col][p.row] - 1 ..=area[p.col][p.row] + 1
        };
        let mut ret = Vec::<Point>::new();
        if point.row > 0 &&  valid_heights(point).contains(&area[point.row - 1][point.col]) {
            queue.push_back(Node { point:Point{row: point.row - 1, col: point.col}, 
                distance: node.distance + 1});
        } 
        if point.row + 1 < rows &&  valid_heights(point).contains(&area[point.row + 1][point.col]) {
            queue.push_back(Node { point:Point{row: point.row + 1, col: point.col}, 
                distance: node.distance + 1});
        } 
    }


    dbg!(&starting_point);
    dbg!(&destination_point);

    queue.push_back(Node { point: starting_point, distance: 0});
    visited[starting_point.row][starting_point.col] = true;

    while !queue.is_empty() {
        let p = queue.front().unwrap();

        if p.point == destination_point {
            return p.distance;
        }
        let p = queue.pop_front().unwrap();




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
        assert_eq!(result, 0);
    }
}
