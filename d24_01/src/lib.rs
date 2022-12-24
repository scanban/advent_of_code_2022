#![allow(dead_code, unused_mut, unused_variables)]

use std::collections::{BTreeMap, HashSet, VecDeque};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {}

struct Expedition {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Blizzard {
    pos: Point,
    dx: i32,
    dy: i32,
}

type Blizzards = Vec<Blizzard>;

#[derive(Debug, Copy, Clone)]
struct MapState {
    size_x: usize,
    size_y: usize,
    entrance: Point,
    exit: Point,
}

const TILE_FREE: u8 = b'.';
const TILE_WALL: u8 = b'#';
const TILE_EXPEDITION: u8 = b'E';
const TILE_BLIZZ_LEFT: u8 = b'<';
const TILE_BLIZZ_RIGHT: u8 = b'>';
const TILE_BLIZZ_UP: u8 = b'^';
const TILE_BLIZZ_DOWN: u8 = b'v';

fn print_map(expedition: &Point, map_state: &MapState, blizzards: &Blizzards) {
    for row in 0..map_state.size_y {
        for col in 0..map_state.size_x {
            if row == 0 || row == map_state.size_y - 1 || col == 0 || col == map_state.size_x - 1 {
                if *expedition == (Point { x: col as i32, y: row as i32 }) { print!("{}", TILE_EXPEDITION as char); } else if map_state.entrance == (Point { x: col as i32, y: row as i32 }) { print!("{}", TILE_FREE as char); } else if map_state.exit == (Point { x: col as i32, y: row as i32 }) { print!("{}", TILE_FREE as char); } else { print!("{}", TILE_WALL as char); }
            } else {
                if *expedition == (Point { x: col as i32, y: row as i32 }) { print!("{}", TILE_EXPEDITION as char); } else {
                    let blizzards = blizzards.iter()
                        .filter(|&b| { b.pos == (Point { x: col as i32, y: row as i32 }) }).collect::<Vec<&Blizzard>>();
                    if blizzards.len() > 1 { print!("{}", blizzards.len()); } else if blizzards.len() != 0 {
                        match (blizzards[0].dx, blizzards[0].dy) {
                            (-1, 0) => print!("{}", TILE_BLIZZ_LEFT as char),
                            (1, 0) => print!("{}", TILE_BLIZZ_RIGHT as char),
                            (0, -1) => print!("{}", TILE_BLIZZ_UP as char),
                            (0, 1) => print!("{}", TILE_BLIZZ_DOWN as char),
                            _ => unreachable!(),
                        }
                    } else { print!("{}", TILE_FREE as char); }
                }
            }
        }
        println!();
    }
}

fn update_blizzards(map_state: &MapState, blizzards: &mut Blizzards) {
    for b in blizzards {
        let mut new_x = b.pos.x + b.dx;
        let mut new_y = b.pos.y + b.dy;

        if new_x == 0 { new_x = map_state.size_x as i32 - 2; } else if new_x == map_state.size_x as i32 - 1 { new_x = 1; }

        if new_y == 0 { new_y = map_state.size_y as i32 - 2; } else if new_y == map_state.size_y as i32 - 1 { new_y = 1 }

        b.pos.x = new_x;
        b.pos.y = new_y;
    }
}

fn solve_problem(input_data: &str) -> i32 {
    let lines = input_data.lines();
    let mut result = 0;

    let input_data = lines.map(|v| { Vec::from(v.as_bytes()) }).collect::<Vec<Vec<u8>>>();
    //dbg!(&input_data);

    let entrance = Point {
        x: input_data[0].iter().position(|&v| v == TILE_FREE).unwrap() as i32,
        y: 0,
    };
    let exit = Point {
        x: input_data[input_data.len() - 1].iter().position(|&v| v == TILE_FREE).unwrap() as i32,
        y: (input_data.len() - 1) as i32,
    };

    let expedition = entrance.clone();

    let mut blizzards = Vec::<Blizzard>::new();

    for row in 1..input_data.len() - 1 {
        for col in 1..input_data[0].len() - 1 {
            match input_data[row][col] {
                TILE_BLIZZ_LEFT => blizzards.push(Blizzard { pos: Point { x: col as i32, y: row as i32 }, dx: -1, dy: 0 }),
                TILE_BLIZZ_RIGHT => blizzards.push(Blizzard { pos: Point { x: col as i32, y: row as i32 }, dx: 1, dy: 0 }),
                TILE_BLIZZ_UP => blizzards.push(Blizzard { pos: Point { x: col as i32, y: row as i32 }, dx: 0, dy: -1 }),
                TILE_BLIZZ_DOWN => blizzards.push(Blizzard { pos: Point { x: col as i32, y: row as i32 }, dx: 0, dy: 1 }),
                _ => {}
            }
        }
    }

    let mut map_state = MapState { size_x: input_data[0].len(), size_y: input_data.len(), entrance, exit};

    #[derive(Debug, Clone, Eq, Hash, PartialEq)]
    struct State {
        expedition: Point,
        steps: i32,
    }

    let mut queue = VecDeque::<State>::new();
    let mut blizzard_states = BTreeMap::<i32, Vec<Blizzard>>::new();
    let mut cache = HashSet::<State>::new();

    queue.push_back(State { expedition, steps: 0 });
    let mut b = blizzards.clone();
    update_blizzards(&map_state, &mut b);
    blizzard_states.insert(0, b);

    while !queue.is_empty() {
        let mut cs = queue.pop_front().unwrap();

        if cache.contains(&cs) {
            continue;
        }
        cache.insert(cs.clone());

        if !blizzard_states.contains_key(&cs.steps) {
            let mut new_b = blizzard_states.get(&(cs.steps - 1)).unwrap().clone();
            update_blizzards(&map_state, &mut new_b);
            blizzard_states.insert(cs.steps, new_b);
        }
        let blizzards = blizzard_states.get(&cs.steps).unwrap();

        if cs.expedition == map_state.exit {
            print_map(&cs.expedition, &map_state, blizzards);
            return cs.steps;
        }


        let mut left_is_free = if cs.expedition.x > 0 && cs.expedition.y != 0
            && cs.expedition.y != map_state.size_y as i32 - 1 { true } else { false };
        let mut right_is_free = if cs.expedition.x < map_state.size_x as i32 - 1
            && cs.expedition.y != 0 && cs.expedition.y != map_state.size_y as i32 - 1 { true } else { false };
        let mut up_is_free = if cs.expedition.y > 1
            || (cs.expedition.y == 1 && cs.expedition.x == map_state.entrance.x) { true } else { false };
        let mut down_is_free = if cs.expedition.y < map_state.size_y as i32 - 2
            || (cs.expedition.y == map_state.size_y as i32 - 2 && cs.expedition.x == map_state.exit.x) { true } else { false };
        let mut current_is_free = true;

        for b in blizzards {
            if left_is_free && b.pos == (Point { x: cs.expedition.x - 1, y: cs.expedition.y }) { left_is_free = false; }
            if right_is_free && b.pos == (Point { x: cs.expedition.x + 1, y: cs.expedition.y }) { right_is_free = false; }
            if up_is_free && b.pos == (Point { x: cs.expedition.x, y: cs.expedition.y - 1 }) { up_is_free = false; }
            if down_is_free && b.pos == (Point { x: cs.expedition.x, y: cs.expedition.y + 1 }) { down_is_free = false; }
            if current_is_free && b.pos == (Point { x: cs.expedition.x, y: cs.expedition.y }) { current_is_free = false; }
        }
        if left_is_free {
            queue.push_back(State { expedition: Point { x: cs.expedition.x - 1, y: cs.expedition.y },
                steps: cs.steps + 1 });
        }
        if right_is_free {
            queue.push_back(State { expedition: Point { x: cs.expedition.x + 1, y: cs.expedition.y },
                steps: cs.steps + 1 });
        }
        if up_is_free {
            queue.push_back(State { expedition: Point { x: cs.expedition.x, y: cs.expedition.y - 1 },
                steps: cs.steps + 1 });
        }
        if down_is_free {
            queue.push_back(State { expedition: Point { x: cs.expedition.x, y: cs.expedition.y + 1 },
                steps: cs.steps + 1 });
        }
        if current_is_free {
            queue.push_back(State { expedition: Point { x: cs.expedition.x, y: cs.expedition.y },
                steps: cs.steps + 1 });
        }
    }

    0
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
        assert_eq!(result, 18);
    }
}
