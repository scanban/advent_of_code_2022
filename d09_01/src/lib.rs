use std::collections::{HashSet};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct KnotPosition {
    x: i32,
    y: i32,
}

struct Rope {
    head: KnotPosition,
    tail: KnotPosition,
    tail_positions: HashSet<KnotPosition>,
}

impl Rope {
    fn tail_move(&mut self) {
        self.tail_positions.insert(self.tail.clone());
    }

    fn move_left(&mut self) {
        if self.tail.x > self.head.x {
            // move
            self.tail.x = self.head.x;
            self.tail.y = self.head.y;
            self.tail_move();
        }
        self.head.x -= 1;
    }

    fn move_right(&mut self) {
        if self.tail.x < self.head.x {
            // move
            self.tail.x = self.head.x;
            self.tail.y = self.head.y;
            self.tail_move();
        }
        self.head.x += 1;
    }

    fn move_up(&mut self) {
        if self.tail.y < self.head.y {
            // move
            self.tail.x = self.head.x;
            self.tail.y = self.head.y;
            self.tail_move();
        }
        self.head.y += 1;
    }

    fn move_down(&mut self) {
        if self.tail.y > self.head.y {
            // move
            self.tail.x = self.head.x;
            self.tail.y = self.head.y;
            self.tail_move();
        }
        self.head.y -= 1;
    }
}

fn solve_problem(input_data: &str) -> usize {
    let lines = input_data.lines();
    let mut rope: Rope = Rope {
        head: KnotPosition { x: 0, y: 0 },
        tail: KnotPosition { x: 0, y: 0 },
        tail_positions: HashSet::new(),
    };

    rope.tail_positions.insert(KnotPosition { x: 0, y: 0 });

    lines.for_each(|v| {
        let direction = v.as_bytes()[0];
        let steps = v.split(' ').skip(1).next().unwrap().parse().unwrap();

        println!("{}: {}", direction, steps);

        match direction {
            b'L' => { for _ in 0..steps { rope.move_left(); } }
            b'R' => { for _ in 0..steps { rope.move_right(); } }
            b'U' => { for _ in 0..steps { rope.move_up(); } }
            b'D' => { for _ in 0..steps { rope.move_down(); } }
            _ => unreachable!()
        }
    });

    rope.tail_positions.len()
}

pub fn solve() -> usize {
    solve_problem(include_str!("../input.txt"))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve_problem(include_str!("../input_test.txt"));
        assert_eq!(result, 13);
    }
}
