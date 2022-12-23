#![allow(dead_code, unused_mut, unused_variables)]

const ELVEN_ROUNDS: usize = 10;
const FREE_TILE: u8 = b'.';
const ELF_TILE: u8 = b'#';

#[derive(Debug, Copy, Clone)]
struct Direction {
    dx: i32,
    dy: i32,
}

const DIRECTION_NORTH: Direction = Direction { dx: 0, dy: -1 };
const DIRECTION_SOUTH: Direction = Direction { dx: 0, dy: 1 };
const DIRECTION_WEST: Direction = Direction { dx: -1, dy: 0 };
const DIRECTION_EAST: Direction = Direction { dx: 1, dy: 0 };
//
const DIRECTION_NE: Direction = Direction { dx: 1, dy: -1 };
const DIRECTION_NW: Direction = Direction { dx: -1, dy: -1 };
const DIRECTION_SE: Direction = Direction { dx: 1, dy: 1 };
const DIRECTION_SW: Direction = Direction { dx: -1, dy: 1 };

const ALL_DIRECTIONS: [Direction; 8] = [
    DIRECTION_NORTH, DIRECTION_SOUTH, DIRECTION_WEST, DIRECTION_EAST,
    DIRECTION_NE, DIRECTION_NW, DIRECTION_SE, DIRECTION_SW];

#[derive(Debug, Copy, Clone)]
struct ValidDirection {
    check: [Direction; 3],
    dir: Direction,
}

const VALID_DIRECTION_NORTH: ValidDirection = ValidDirection { check: [DIRECTION_NORTH, DIRECTION_NE, DIRECTION_NW], dir: DIRECTION_NORTH };
const VALID_DIRECTION_SOUTH: ValidDirection = ValidDirection { check: [DIRECTION_SOUTH, DIRECTION_SE, DIRECTION_SW], dir: DIRECTION_SOUTH };
const VALID_DIRECTION_WEST: ValidDirection = ValidDirection { check: [DIRECTION_WEST, DIRECTION_NW, DIRECTION_SW], dir: DIRECTION_WEST };
const VALID_DIRECTION_EAST: ValidDirection = ValidDirection { check: [DIRECTION_EAST, DIRECTION_NE, DIRECTION_SE], dir: DIRECTION_EAST };

fn print_crater_map(crater_map: &Vec<Vec<u8>>) {
    crater_map.iter().for_each(|l|{
        l.iter().for_each(|&v| print!("{}", v as char));
        println!();
    })
}

fn round(crater_map: &mut Vec<Vec<u8>>, valid_directions: &Vec<ValidDirection>) {
    #[derive(Debug)]
    struct ProposedDirection {
        x: usize,
        y: usize,
        dir: Direction,
    }

    let tile = |row: usize, col: usize, dir: &Direction| -> u8 {
        crater_map[(row as i32 + dir.dy) as usize][(col as i32 + dir.dx) as usize]
    };

    let mut proposed_moves = Vec::<ProposedDirection>::new();

    for row in 0..crater_map.len() {
        for col in 0..crater_map[0].len() {
            if crater_map[row][col] == ELF_TILE {
                if ALL_DIRECTIONS.iter().all(|v| tile(row, col, &v) == FREE_TILE) { continue; }
                for pm in valid_directions {
                    if pm.check.iter().all(|v| tile(row, col, &v) == FREE_TILE) {
                        proposed_moves.push(ProposedDirection { x: col, y: row, dir: pm.dir });
                        break;
                    }
                }
            }
        }
    }

    for mv in &proposed_moves {
        if ! proposed_moves.iter().any(|v|{
            !(mv.x == v.x && mv.y == v.y)
            && mv.x as i32 + mv.dir.dx == v.x as i32 + v.dir.dx
            && mv.y as i32 + mv.dir.dy == v.y as i32 + v.dir.dy
        }) {
            crater_map[mv.y][mv.x] = FREE_TILE;
            crater_map[(mv.y as i32 + mv.dir.dy) as usize][(mv.x as i32 + mv.dir.dx) as usize] = ELF_TILE;
        }
    }
}


fn solve_problem(input_data: &str) -> i32 {
    let lines = input_data.lines();
    let mut result = 0;

    let mut crater_map = lines.map(|s| {
        let mut v = Vec::<u8>::new();
        v.resize(ELVEN_ROUNDS, FREE_TILE);
        v.extend_from_slice(s.as_bytes());
        v.resize(v.len() + ELVEN_ROUNDS, FREE_TILE);
        v
    }).collect::<Vec<Vec<u8>>>();

    let mut empty_line = Vec::<u8>::new();
    empty_line.resize(crater_map[0].len(), FREE_TILE);

    for _ in 0..ELVEN_ROUNDS {
        crater_map.insert(0, empty_line.to_vec());
        crater_map.push(empty_line.to_vec());
    }

    let mut proposed_directions = vec![VALID_DIRECTION_NORTH, VALID_DIRECTION_SOUTH,
                                       VALID_DIRECTION_WEST, VALID_DIRECTION_EAST];

    if cfg!(test) {
        print_crater_map(&crater_map);
        println!("+++++++++++++++++++++++");
    }
    for _ in 0..10 {
        round(&mut crater_map, &proposed_directions);
        proposed_directions.as_mut_slice().rotate_left(1);
        if cfg!(test) {
            println!("=========================");
            print_crater_map(&crater_map);
        }
    }

    let mut top_y = 0;
    let mut bot_y = crater_map.len() - 1;
    let mut top_x = 0;
    let mut bot_x = crater_map[0].len() - 1;

    for i in 0..crater_map.len() {
        if crater_map[i].iter().any(|&v| v != FREE_TILE) { break; }
        top_y += 1;
    }
    for i in (0..crater_map.len()).rev() {
        if crater_map[i].iter().any(|&v| v != FREE_TILE) { break; }
        bot_y -= 1;
    }
    for i in 0..crater_map[0].len() {
        if (0..crater_map.len()).any(|n|{ crater_map[n][i] != FREE_TILE }) { break; }
        top_x += 1;
    }
    for i in (0..crater_map[0].len()).rev() {
        if (0..crater_map.len()).any(|n|{ crater_map[n][i] != FREE_TILE }) { break; }
        bot_x -= 1;
    }

    for row in top_y..=bot_y {
        for col in top_x..=bot_x {
            if crater_map[row][col] == FREE_TILE { result += 1; }
        }
    }


    println!("ty{}:by{}, tx{}:bx:{}", top_y, bot_y, top_x, bot_x);

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
        assert_eq!(result, 110);
    }
}
