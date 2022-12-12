use std::cmp::min;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, Default)]
struct Point {
    row: usize,
    col: usize,
}

struct Node {
    point: Point,
    distance: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

fn init_vector_2d<T>(rows: usize, cols: usize, val: T) -> Vec<Vec<T>> where T: Clone {
    let mut ret = Vec::<Vec<T>>::new();
    ret.resize_with(rows, || {
        let mut ret = Vec::<T>::new();
        ret.resize(cols, val.clone());
        ret
    });
    ret
}

fn solve_problem_x(starting_point: Point, destination_point: Point, area: &Vec<Vec<u8>>) -> i32 {
    let rows = area.len();
    let cols = area[0].len();
    let mut visited: Vec<Vec<bool>> = init_vector_2d(rows, cols, false);
    let mut queue: VecDeque<Node> = VecDeque::new();

    queue.push_back(Node { point: starting_point, distance: 0 });
    visited[starting_point.row][starting_point.col] = true;

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        if node.point == destination_point {
            return node.distance;
        }
        let point = &node.point;

        let valid_height = |v| {
            v <= area[point.row][point.col] + 1
        };

        let mut push_node = |row: usize, col: usize| {
            if valid_height(area[row][col]) && !visited[row][col] {
                queue.push_back(Node {
                    point: Point { row, col },
                    distance: node.distance + 1,
                });
                visited[row][col] = true;
            }
        };

        if point.row > 0 { push_node(point.row - 1, point.col); }
        if point.row + 1 < rows { push_node(point.row + 1, point.col); }
        if point.col > 0 { push_node(point.row, point.col - 1); }
        if point.col + 1 < cols { push_node(point.row, point.col + 1); }
    };
    i32::MAX
}

fn solve_problem(input_data: &str) -> i32 {
    let lines = input_data.lines();
    let mut area = lines
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let rows = area.len();
    let cols = area[0].len();

    let mut destination_point = Point::default();

    for row in 0..rows {
        for col in 0..cols {
            if area[row][col] == b'S' {
                area[row][col] = b'a';
            }
            if area[row][col] == b'E' {
                destination_point = Point { row, col };
                area[row][col] = b'z';
            }
        }
    }

    let mut result = i32::MAX;
    for row in 0..rows {
        for col in 0..cols {
            if area[row][col] == b'a' {
                result = min(result, solve_problem_x(Point { row, col }, destination_point, &area));
            }
        }
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
        assert_eq!(result, 29);
    }
}
