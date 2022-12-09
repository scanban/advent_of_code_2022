use std::collections::{HashSet};
use std::fmt::{Debug, Formatter, Write};

const NUMBER_OF_KNOTS: usize = 10;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct KnotPosition {
    x: i32,
    y: i32,
}

struct Rope {
    knot_positions: [KnotPosition; NUMBER_OF_KNOTS],
    tail_positions: HashSet<KnotPosition>,
}

impl Debug for Rope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const P_LIMIT: i32 = 20;
        const LABELS: &[u8] = "H123456789".as_bytes();

        for row in (-P_LIMIT..=P_LIMIT).rev() {
            for col in -P_LIMIT..=P_LIMIT {
                match self.knot_positions.iter().position(|v| *v == KnotPosition { x: col, y: row })
                {
                    Some(v) => { let _ = f.write_char(LABELS[v] as char); }
                    _ => { let _ = f.write_char('.'); }
                }
            }
            let _ = f.write_str("\n");
        }
        Ok(())
    }
}

struct MoveElement {
    dx: i32,
    dy: i32,
}

const MOVE_LEFT: MoveElement = MoveElement { dx: -1, dy: 0 };
const MOVE_RIGHT: MoveElement = MoveElement { dx: 1, dy: 0 };
const MOVE_UP: MoveElement = MoveElement { dx: 0, dy: 1 };
const MOVE_DOWN: MoveElement = MoveElement { dx: 0, dy: -1 };

impl Rope {
    fn position_move(p: &mut HashSet<KnotPosition>, k: &KnotPosition) {
        if cfg!(test) {
            println!("{:?}", k);
        }
        p.insert(k.clone());
    }

    fn step(head: &KnotPosition, tail: &KnotPosition) -> Option<KnotPosition> {
        const MOVE_MATRIX: [[(i32, i32); 5]; 5] = [
            [(1, -1), (1, -1), (0, -1), (-1, -1), (-1, -1)],
            [(1, -1), (0, 0), (0, 0), (0, 0), (-1, -1)],
            [(1, 0), (0, 0), (0, 0), (0, 0), (-1, 0)],
            [(1, 1), (0, 0), (0, 0), (0, 0), (-1, 1)],
            [(1, 1), (1, 1), (0, 1), (-1, 1), (-1, 1)],
        ];

        let m_row = (head.y - tail.y + 2) as usize;
        let m_col = (tail.x - head.x + 2) as usize;

        let (dx, dy) = MOVE_MATRIX[m_row][m_col];

        if dx == 0 && dy == 0 {
            return None;
        }

        Some(KnotPosition { x: tail.x + dx, y: tail.y + dy })
    }

    fn move_head(&mut self, m: &MoveElement) {
        self.knot_positions[0].x += m.dx;
        self.knot_positions[0].y += m.dy;

        for i in 0..NUMBER_OF_KNOTS - 1 {
            let (h_a, t_a) = self.knot_positions.split_at_mut(i + 1);
            let head = h_a.last_mut().unwrap();
            let tail = t_a.first_mut().unwrap();

            match Rope::step(head, tail)
            {
                Some(v) => {
                    *tail = v;
                }
                _ => { return; }
            }
        }
        Rope::position_move(&mut self.tail_positions, self.knot_positions.last().unwrap());
    }
}

fn solve_problem(input_data: &str) -> usize {
    let lines = input_data.lines();
    let mut rope = Rope {
        knot_positions: [KnotPosition { x: 0, y: 0 }; NUMBER_OF_KNOTS],
        tail_positions: HashSet::new(),
    };

    rope.tail_positions.insert(KnotPosition { x: 0, y: 0 });

    lines.for_each(|v| {
        let direction = v.as_bytes()[0];
        let steps = v.split(' ').skip(1).next().unwrap().parse().unwrap();

        match direction {
            b'L' => { for _ in 0..steps { rope.move_head(&MOVE_LEFT); } }
            b'R' => { for _ in 0..steps { rope.move_head(&MOVE_RIGHT); } }
            b'U' => { for _ in 0..steps { rope.move_head(&MOVE_UP); } }
            b'D' => { for _ in 0..steps { rope.move_head(&MOVE_DOWN); } }
            _ => unreachable!()
        }
        if cfg!(test) {
            println!("{:?}", rope);
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
        assert_eq!(result, 36);
    }
}
