#![allow(dead_code, unused_mut, unused_variables)]

use lazy_static::lazy_static;
use regex::Regex;

const OPEN_TILE: u8 = b'.';
const WALL_TILE: u8 = b'#';

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
    direction_id: usize,
}

impl Mover {
    const DIRECTION: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    const RIGHT: usize = 0;
    const DOWN: usize = 1;
    const LEFT: usize = 2;
    const UP: usize = 3;

    fn rotate(&mut self, direction: &MoveInstruction) {
        match direction {
            MoveInstruction::R => {
                self.direction_id = if self.direction_id < 3 { self.direction_id + 1 } else { 0 };
            }
            MoveInstruction::L => {
                self.direction_id = if self.direction_id > 0 { self.direction_id - 1 } else { 3 };
            }
            _ => unreachable!()
        }
        (self.dx, self.dy) = Mover::DIRECTION[self.direction_id];
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
        direction_id: Mover::RIGHT,
    };

    for instr in move_instructions {
        match instr {
            MoveInstruction::STEPS(steps) => {
                for _ in 0..steps {
                    let mut next_x = mover.x;
                    let mut next_y = mover.y;

                    loop {
                        next_x = (next_x + mover.dx) % maze[0].len() as i32;
                        if next_x < 0 { next_x = maze[0].len() as i32 - 1; }

                        next_y = (next_y + mover.dy) % maze.len() as i32;
                        if next_y < 0 { next_y = maze.len() as i32 - 1; }

                        if maze[next_y as usize][next_x as usize] != b' ' { break; }
                    }
                    if maze[next_y as usize][next_x as usize] == WALL_TILE { break; }
                    mover.x = next_x;
                    mover.y = next_y;
                }
            }
            MoveInstruction::R | MoveInstruction::L => { mover.rotate(&instr); }
        }
        //if cfg!(test) {
        println!("{:?} -> {:?}", instr, mover);
        // }
    }


    //if cfg!(test) {
    dbg!(&mover);
    //}
    let result = 1000 * (mover.y + 1) + 4 * (mover.x + 1) + mover.direction_id as i32;

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
        assert_eq!(result, 6032);
    }
}
