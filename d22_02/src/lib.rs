#![allow(dead_code, unused_mut, unused_variables)]

use lazy_static::lazy_static;
use regex::Regex;

const OPEN_TILE: u8 = b'.';
const WALL_TILE: u8 = b'#';
const EMPTY_TILE: u8 = b' ';

#[derive(Debug, Copy, Clone)]
enum MoveInstruction {
    R,
    L,
    STEPS(i32),
}

#[derive(Debug, Copy, Clone)]
struct Mover {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    direction: Direction,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    RIGHT = 0,
    DOWN = 1,
    LEFT = 2,
    UP = 3,
}

impl Direction {
    fn from_i32(v: i32) -> Direction {
        match v {
            0 => Direction::RIGHT,
            1 => Direction::DOWN,
            2 => Direction::LEFT,
            3 => Direction::UP,
            _ => unreachable!()
        }
    }
}

impl Mover {
    const DIRECTION: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    fn deltas(direction: &Direction) -> (i32, i32) {
        Mover::DIRECTION[*direction as usize]
    }

    fn rotate(&mut self, direction: &MoveInstruction) {
        match direction {
            MoveInstruction::R => {
                self.direction = if (self.direction as i32) < 3 { Direction::from_i32(self.direction as i32 + 1) } else { Direction::from_i32(0) };
            }
            MoveInstruction::L => {
                self.direction = if (self.direction as i32) > 0 { Direction::from_i32(self.direction as i32 - 1) } else { Direction::from_i32(3) };
            }
            _ => unreachable!()
        }
        (self.dx, self.dy) = Mover::DIRECTION[self.direction as usize];
    }
}

/*
      0   50   100 150
0         +---+---+
          | 0 | 4 |
50        +---+---+
          | 1 |
100   +---+---+
      | 5 | 2 |
150   +---+---+
      | 3 |
200   +---+

 */

const SIDE_MAX: i32 = 49;

fn jump(mover: &mut Mover, maze: &Vec<Vec<u8>>) {
    #[derive(Debug, Eq, PartialEq)]
    struct Side {
        top_y: i32,
        top_x: i32,
        bot_y: i32,
        bot_x: i32,
    }
    impl Side {
        const SIDE_0: Side = Side { top_y: 0, top_x: 50, bot_y: 50 - 1, bot_x: 100 - 1 };
        const SIDE_1: Side = Side { top_y: 50, top_x: 50, bot_y: 100 - 1, bot_x: 100 - 1 };
        const SIDE_2: Side = Side { top_y: 100, top_x: 50, bot_y: 150 - 1, bot_x: 100 - 1 };
        const SIDE_3: Side = Side { top_y: 150, top_x: 0, bot_y: 200 - 1, bot_x: 50 - 1 };
        const SIDE_4: Side = Side { top_y: 0, top_x: 100, bot_y: 50 - 1, bot_x: 150 - 1 };
        const SIDE_5: Side = Side { top_y: 100, top_x: 0, bot_y: 150 - 1, bot_x: 50 - 1 };

        fn x_in_side(&self, x: i32) -> bool {
            x >= self.top_x && x <= self.bot_x
        }
        fn y_in_side(&self, y: i32) -> bool {
            y >= self.top_y && y <= self.bot_y
        }

        fn in_side(&self, x: i32, y: i32) -> bool {
            self.x_in_side(x) && self.y_in_side(y)
        }

        fn which_side(x: i32, y: i32) -> &'static Side {
            const SIDES: [Side; 6] = [Side::SIDE_0, Side::SIDE_1, Side::SIDE_2, Side::SIDE_3, Side::SIDE_4, Side::SIDE_5];

            SIDES.iter().find(|v| v.in_side(x, y)).unwrap()
        }
    }
    struct WormHole {
        s1: Side,
        s2: Side,
        // x, y -> (x, y)
        coords: fn(i32, i32) -> (i32, i32),
        dir: Direction,
        new_dir: Direction,
    }

    const WMS: [WormHole; 24] = [
        // 0
        WormHole { s1: Side::SIDE_0, s2: Side::SIDE_5, dir: Direction::LEFT, new_dir: Direction::RIGHT, coords: |x, y| (0, SIDE_MAX - y) },
        WormHole { s1: Side::SIDE_0, s2: Side::SIDE_4, dir: Direction::RIGHT, new_dir: Direction::RIGHT, coords: |x, y| (0, y) },
        WormHole { s1: Side::SIDE_0, s2: Side::SIDE_3, dir: Direction::UP, new_dir: Direction::RIGHT, coords: |x, y| (0, x) },
        WormHole { s1: Side::SIDE_0, s2: Side::SIDE_1, dir: Direction::DOWN, new_dir: Direction::DOWN, coords: |x, y| (x, 0) },

        // 1
        WormHole { s1: Side::SIDE_1, s2: Side::SIDE_5, dir: Direction::LEFT, new_dir: Direction::DOWN, coords: |x, y| (y, 0) },
        WormHole { s1: Side::SIDE_1, s2: Side::SIDE_4, dir: Direction::RIGHT, new_dir: Direction::UP, coords: |x, y| (y, SIDE_MAX) },
        WormHole { s1: Side::SIDE_1, s2: Side::SIDE_0, dir: Direction::UP, new_dir: Direction::UP, coords: |x, y| (x, SIDE_MAX) },
        WormHole { s1: Side::SIDE_1, s2: Side::SIDE_2, dir: Direction::DOWN, new_dir: Direction::DOWN, coords: |x, y| (x, 0) },

        // 2
        WormHole { s1: Side::SIDE_2, s2: Side::SIDE_5, dir: Direction::LEFT, new_dir: Direction::LEFT, coords: |x, y| (SIDE_MAX, y) },
        WormHole { s1: Side::SIDE_2, s2: Side::SIDE_4, dir: Direction::RIGHT, new_dir: Direction::LEFT, coords: |x, y| (SIDE_MAX, SIDE_MAX - y) },
        WormHole { s1: Side::SIDE_2, s2: Side::SIDE_1, dir: Direction::UP, new_dir: Direction::UP, coords: |x, y| (x, SIDE_MAX) },
        WormHole { s1: Side::SIDE_2, s2: Side::SIDE_3, dir: Direction::DOWN, new_dir: Direction::LEFT, coords: |x, y| (SIDE_MAX, x) },

        // 3
        WormHole { s1: Side::SIDE_3, s2: Side::SIDE_0, dir: Direction::LEFT, new_dir: Direction::DOWN, coords: |x, y| (y, 0) },
        WormHole { s1: Side::SIDE_3, s2: Side::SIDE_2, dir: Direction::RIGHT, new_dir: Direction::UP, coords: |x, y| (y, SIDE_MAX) },
        WormHole { s1: Side::SIDE_3, s2: Side::SIDE_5, dir: Direction::UP, new_dir: Direction::UP, coords: |x, y| (x, SIDE_MAX) },
        WormHole { s1: Side::SIDE_3, s2: Side::SIDE_4, dir: Direction::DOWN, new_dir: Direction::DOWN, coords: |x, y| (x, 0) },

        // 4
        WormHole { s1: Side::SIDE_4, s2: Side::SIDE_0, dir: Direction::LEFT, new_dir: Direction::LEFT, coords: |x, y| (SIDE_MAX, y) },
        WormHole { s1: Side::SIDE_4, s2: Side::SIDE_2, dir: Direction::RIGHT, new_dir: Direction::LEFT, coords: |x, y| (SIDE_MAX, SIDE_MAX - y) },
        WormHole { s1: Side::SIDE_4, s2: Side::SIDE_3, dir: Direction::UP, new_dir: Direction::UP, coords: |x, y| (x, SIDE_MAX) },
        WormHole { s1: Side::SIDE_4, s2: Side::SIDE_1, dir: Direction::DOWN, new_dir: Direction::LEFT, coords: |x, y| (SIDE_MAX, x) },

        // 5
        WormHole { s1: Side::SIDE_5, s2: Side::SIDE_0, dir: Direction::LEFT, new_dir: Direction::RIGHT, coords: |x, y| (0, SIDE_MAX - y) },
        WormHole { s1: Side::SIDE_5, s2: Side::SIDE_2, dir: Direction::RIGHT, new_dir: Direction::RIGHT, coords: |x, y| (0, y) },
        WormHole { s1: Side::SIDE_5, s2: Side::SIDE_1, dir: Direction::UP, new_dir: Direction::RIGHT, coords: |x, y| (0, x) },
        WormHole { s1: Side::SIDE_5, s2: Side::SIDE_3, dir: Direction::DOWN, new_dir: Direction::DOWN, coords: |x, y| (x, 0) },
    ];

    let mut next_x;
    let mut next_y;
    let mut next_direction: Direction; // dx, dy

    let side = Side::which_side(mover.x, mover.y);
    if (mover.x == side.top_x && mover.direction == Direction::LEFT)
        || (mover.x == side.bot_x && mover.direction == Direction::RIGHT)
        || (mover.y == side.top_y && mover.direction == Direction::UP)
        || (mover.y == side.bot_y && mover.direction == Direction::DOWN) {
        if cfg!(test) {
            println!("Jump");
        }

        let w = WMS.iter().find(|v| v.s1 == *side && v.dir == mover.direction).unwrap();
        (next_x, next_y) = (w.coords)(mover.x - side.top_x, mover.y - side.top_y);
        next_x += w.s2.top_x;
        next_y += w.s2.top_y;
        next_direction = w.new_dir;
    } else {
        next_x = mover.x + mover.dx;
        next_y = mover.y + mover.dy;
        next_direction = mover.direction;
    }

    if maze[next_y as usize][next_x as usize] == OPEN_TILE {
        mover.x = next_x;
        mover.y = next_y;
        mover.direction = next_direction;
        (mover.dx, mover.dy) = Mover::deltas(&mover.direction);
    }
}

fn solve_problem(input_data: &str) -> i32 {
    lazy_static! {
        static ref RE_MOVE_INSTRUCTIONS: Regex = Regex::new(r"(\d+|L|R)").unwrap();
    }
    let mut lines = input_data.lines();
    let mut maze = Vec::<Vec<u8>>::new();


    loop {
        let line = lines.next().unwrap();
        if line.is_empty() { break; }
        maze.push(Vec::from(line.as_bytes()));
    }

    let max_line_len = maze.iter().map(|v| v.len()).max().unwrap();
    let cube_side_len = maze.iter().map(|v| v.iter().filter(|&&c| c != b' ').count()).min().unwrap();

    if cfg!(test) {
        dbg!(cube_side_len);
    }

    maze.iter_mut().for_each(|v| v.resize(max_line_len, b' '));

    let move_instructions = RE_MOVE_INSTRUCTIONS.captures_iter(lines.next().unwrap())
        .map(|v| { v.get(1).unwrap().as_str() }).map(|v| {
        let steps = v.parse::<i32>();
        if steps.is_err() {
            match v {
                "R" => return MoveInstruction::R,
                "L" => return MoveInstruction::L,
                _ => unreachable!()
            }
        } else {
            MoveInstruction::STEPS(steps.unwrap())
        }
    }).collect::<Vec<MoveInstruction>>();

    let mut mover = Mover {
        x: maze[0].iter().position(|&v| v == OPEN_TILE).unwrap() as i32,
        y: 0,
        dx: 1,
        dy: 0,
        direction: Direction::RIGHT,
    };

    for instr in move_instructions {
        match instr {
            MoveInstruction::STEPS(steps) => {
                for _ in 0..steps {
                    jump(&mut mover, &maze);
                }
            }
            MoveInstruction::R | MoveInstruction::L => { mover.rotate(&instr); }
        }
        if cfg!(test) {
            println!("{:?} -> {:?}", instr, mover);
        }
    }


    if cfg!(test) {
        dbg!(&mover);
    }
    let result = 1000 * (mover.y + 1) + 4 * (mover.x + 1) + mover.direction as i32;

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
        let result = solve_problem(include_str!("../input.txt"));
        assert_eq!(result, 6032);
    }
}
