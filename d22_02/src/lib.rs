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


fn jump(mover: & mut Mover, m_next_x: i32, m_next_y: i32, maze: &Vec<Vec<u8>>) {
    struct Side {
        top_y: i32,
        top_x: i32,
        bot_y: i32,
        bot_x: i32,
    }
    impl Side {
        const SIDE_0: Side = Side{top_y: 0, top_x: 50, bot_y: 50, bot_x: 100};
        const SIDE_1: Side = Side{top_y: 50, top_x: 50, bot_y: 100, bot_x: 100};
        const SIDE_2: Side = Side{top_y: 100, top_x: 50, bot_y: 150, bot_x: 100};
        const SIDE_3: Side = Side{top_y: 150, top_x: 0, bot_y: 200, bot_x: 50};
        const SIDE_4: Side = Side{top_y: 0, top_x: 100, bot_y: 50, bot_x: 150};
        const SIDE_5: Side = Side{top_y: 100, top_x: 0, bot_y: 150, bot_x: 50};

        fn x_in_side(&self, x: i32) -> bool {
            x >= self.top_x && x < self.bot_x
        }
        fn y_in_side(&self, y: i32) -> bool {
            y >= self.top_y && y < self.bot_y
        }
    }

    let mut next_x = 0;
    let mut next_y = 0;
    let mut next_direction = (0, 0); // dx, dy

    if next_y < 0 || mover.direction_id == Mover::UP && maze[next_y as usize][mover.x as usize] == EMPTY_TILE {
        // side 0:top:up -> side 3:left:right
        if Side::SIDE_0.x_in_side(mover.x) {
            next_y = mover.x + 100;
            next_x = 0;
            next_direction = Mover::DIRECTION[Mover::RIGHT];
        }
        // side 4:top:up -> side 3:bot:up
        else if Side::SIDE_4.x_in_side(mover.x) {
            next_y = 199;
            next_x = mover.x - 100;
            next_direction = Mover::DIRECTION[Mover::UP];
        }
        // side 5:top:up -> side 1:left:right
        else if Side::SIDE_5.x_in_side(mover.x) {
            next_y = mover.x + 50;
            next_x = 50;
            next_direction = Mover::DIRECTION[Mover::RIGHT];
        }
    } else if next_y >= maze.len() as i32 || mover.direction_id == Mover::DOWN && maze[next_y as usize][mover.x as usize] == EMPTY_TILE {
        // side 4:bot:down -> side 1:right:left
        if Side::SIDE_4.x_in_side(mover.x) {
            next_y = mover.x - 100 + 50;
            next_x = 99;
            next_direction = Mover::DIRECTION[Mover::LEFT];
        }
        // side 2:bot:down -> side 3:right:left
        else if Side::SIDE_2.x_in_side(mover.x) {
            next_y = mover.x - 50 + 150;
            next_x = 49;
            next_direction = Mover::DIRECTION[Mover::LEFT];
        }
        // side 3:bot:down -> side 4:top:down
        else if Side::SIDE_3.x_in_side(mover.x) {
            next_y = 0;
            next_x = mover.x + 100;
            next_direction = Mover::DIRECTION[Mover::DOWN];
        }
    } else if next_x < 0 || mover.direction_id == Mover::LEFT && maze[next_y as usize][mover.x as usize] == EMPTY_TILE {
        // side 0:left:left -> side 5:left:left
        if Side::SIDE_0.y_in_side(mover.y) {
            next_y = 149 - mover.y;
            next_x = 49;
            next_direction = Mover::DIRECTION[Mover::RIGHT];
        }
        // side 1:left:left -> side 5:top:down
        else if Side::SIDE_1.y_in_side(mover.y) {
            next_y = 100;
            next_x = mover.y - 50;
            next_direction = Mover::DIRECTION[Mover::DOWN];
        }
        // side 5:left:left -> side 0:right:left
        else if Side::SIDE_5.y_in_side(mover.y) {
            next_y = mover.y - 100;
            next_x = 99;
            next_direction = Mover::DIRECTION[Mover::LEFT];
        }
        // side 3:left:left -> side 0:down:up
        else if Side::SIDE_3.y_in_side(mover.y) {
            next_y = ;
            next_x = mover.y - 150 ;
            next_direction = Mover::DIRECTION[Mover::UP];
        }
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
    let cube_side_len = maze.iter().map(|v| v.iter().filter(|&&c|c != b' ').count()).min().unwrap();
    dbg!(cube_side_len);

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
