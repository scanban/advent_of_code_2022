#![allow(dead_code, unused_mut, unused_variables)]

use std::cmp::{max, min};
use lazy_static::lazy_static;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Dimension {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Shape {
    dimension: Dimension,
    points: Vec<Point>,
}

lazy_static! {
    static ref SHAPE_MINUS: Shape = Shape {
        dimension: Dimension {x: 4, y: 1},
        points: Vec::from([ Point{x: 0, y: 0}, Point{x: 1, y: 0}, Point{x: 2, y:0}, Point{x: 3, y: 0} ]),
    };
    static ref SHAPE_PLUS: Shape = Shape {
        dimension: Dimension {x: 3, y: 3},
        points: Vec::from([ Point{x: 1, y: 0},
            Point{x: 0, y: 1}, Point{x: 1, y:1}, Point{x: 2, y: 1},
            Point{x: 1, y: 2}]),
    };
    static ref SHAPE_L: Shape = Shape {
        dimension: Dimension {x: 3, y: 3},
        points: Vec::from([ Point{x: 0, y: 0}, Point{x: 1, y:0}, Point{x: 2, y: 0},
            Point{x: 2, y: 1}, Point{x: 2, y: 2} ]),
    };
    static ref SHAPE_VL: Shape = Shape {
        dimension: Dimension {x: 1, y: 4},
        points: Vec::from([ Point{x: 0, y: 0}, Point{x: 0, y:1}, Point{x: 0, y: 2}, Point{x: 0, y: 3} ]),
    };
    static ref SHAPE_CUBE: Shape = Shape {
        dimension: Dimension {x: 2, y: 2},
        points: Vec::from([ Point{x: 0, y: 0}, Point{x: 1, y:0}, Point{x: 0, y: 1}, Point{x: 1, y: 1} ]),
    };
}

#[derive(Debug)]
struct ShapeInPlay<'a> {
    coords: Point,
    shape: &'a Shape,
}

const CHAMBER_WIDTH: usize = 7;
const INITIAL_ROCK_OFFSET_X: usize = 2;
const INITIAL_ROCK_OFFSET_Y: usize = 3;

type ChamberType = Vec<[u8; CHAMBER_WIDTH]>;

/*fn print_chamber_old(shapes: &Vec<ShapeInPlay>) {
    let max_y =
        shapes.iter().map(|v| v.coords.y + v.shape.dimension.y).max().unwrap();

    for chamber_y in (0..max_y).rev() {
        let mut scan_line = [b'.'; CHAMBER_WIDTH];

        for shape in shapes {
            shape.shape.points.iter().for_each(|p| {
                if chamber_y == shape.coords.y + p.y {
                    scan_line[(p.x + shape.coords.x) as usize] = b'#';
                }
            });
        }
        println!("|{}|", std::str::from_utf8(&scan_line).unwrap());
    }
    println!("+-------+");
}
*/

fn print_chamber(chamber: &ChamberType, current_shape: Option<(usize, usize, &Shape)>) {
    let mut non_empty_seen = false;
    for y_coord in (0..chamber.len()).rev() {
        let mut pline = chamber[y_coord].clone();

        if current_shape.is_some() {
            let (shape_y, shape_x, shape) = current_shape.unwrap();
            for p in &shape.points {
                if shape_y + p.y == y_coord { pline[shape_x + p.x] = 1; }
            }
        }

        if non_empty_seen || pline.contains(&1) {
            println!("|{}|", pline.iter().map(|v| if *v != 0 { '#' } else { '.' }).collect::<String>());
            non_empty_seen = true;
        }
    }
    println!("+-------+\n");
}

fn lcp<'a>(lhs: &'a [u64], rhs: &'a [u64]) -> &'a [u64] {
    let sz = min(lhs.len(), rhs.len());
    for i in 0..sz {
        if lhs[i] != rhs[i] {
            return &lhs[0..i];
        }
    }
    &lhs[0..sz]
}

fn longest_seq(input: &Vec<u64>) -> Vec<u64> {
    let mut sub: &[u64] = &[0u64; 0];

    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let new_sub = lcp(&input[i..input.len()], &input[j..input.len()]);
            /*            new_sub.iter().for_each(|v| print!("{v},"));
                        println!();
            */            if new_sub.len() > sub.len() {
                sub = new_sub;
                sub.iter().for_each(|v| print!("{v},"));
                println!();
            }
        }
    }
    Vec::from(sub)
}

fn longest_seq2(input: &Vec<u64>) -> Vec<u64> {
    let mut dp = Vec::<Vec<usize>>::new();

    dp.resize(input.len(), Vec::new());
    for i in 0..input.len() {
        dp[i].resize(input.len(), 0);
    }

    let mut index = 0usize;
    let mut max = 0usize;

    for i in 1..input.len() {
        for j in i + 1..input.len() {
            if input[j - 1] == input[i - 1] && j - i > dp[i - 1][j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
                if dp[i][j] > max {
                    max = dp[i][j];
                    index = i - max;
                }
            }
        }
    }

    Vec::from(&input[index..max])
}

fn longest_seq3(input: &Vec<u64>) -> (usize, Vec<u64>) {
    let mut idx: usize;
    let mut sz: usize;

    for i in (0..=(input.len()/2)).rev() {
        for j in 0..=(input.len() - 2*i) {
            let lhs_start = j;
            let lhs_end = j + i;
            let rhs_start = j + i;
            let rhs_end = j + i + i;

/*            println!("i: {}, j:{}", i, j);
            input[lhs_start..lhs_end].iter().for_each(|v| print!("{v},"));
            print!(":");
            input[rhs_start..rhs_end].iter().for_each(|v| print!("{v},"));
            println!();
*/            if input[lhs_start..lhs_end].eq(&input[rhs_start..rhs_end]) {
                return (lhs_start, Vec::from(&input[lhs_start..lhs_end]));
            }
        }
    }


    (0, Vec::new())
}


fn solve_problem(input_data: &str) -> usize {
    let mut chamber = ChamberType::new();
    chamber.resize(5000, [0; CHAMBER_WIDTH]);

    let mut gas_commands = input_data.as_bytes().iter().cycle();
    let rock_order = [&*SHAPE_MINUS, &*SHAPE_PLUS, &*SHAPE_L, &*SHAPE_VL, &*SHAPE_CUBE];
    let mut rock_producer = rock_order.iter().cycle();

    let mut rock_fell = 0;
    let mut top_level = 0usize;

    let mut shape_x = INITIAL_ROCK_OFFSET_X;
    let mut shape_y = top_level + INITIAL_ROCK_OFFSET_Y;
    let mut shape = *rock_producer.next().unwrap();
    let mut pss = Vec::<u64>::new();

    loop {
        // gas
        //print_chamber(&chamber, Some((shape_y, shape_x, shape)));

        let &gas_command = gas_commands.next().unwrap();
        //println!("gas: {}", gas_command as char);
        match gas_command {
            b'>' => {
                if shape.points.iter().all(|p| shape_x + p.x + 1 < CHAMBER_WIDTH
                    && chamber[shape_y + p.y][shape_x + p.x + 1] == 0) { shape_x += 1; }
            }
            b'<' => {
                if shape.points.iter().all(|p| shape_x + p.x > 0
                    && chamber[shape_y + p.y][shape_x + p.x - 1] == 0) { shape_x -= 1; }
            }
            _ => unreachable!(),
        }
        //print_chamber(&chamber, Some((shape_y, shape_x, shape)));

        if shape_y > 0 && shape.points.iter().all(|p| {
            chamber[shape_y + p.y - 1][shape_x + p.x] == 0
        }) {
            shape_y -= 1;
        } else {
            let k = top_level;
            top_level = max(top_level, shape_y + shape.dimension.y);
            shape.points.iter().for_each(|p| {
                chamber[shape_y + p.y][shape_x + p.x] = 1;
            });
            rock_fell += 1;
            // new rock
            shape_x = INITIAL_ROCK_OFFSET_X;
            shape_y = top_level + INITIAL_ROCK_OFFSET_Y;
            shape = *rock_producer.next().unwrap();
            //println!("rock: {} stopped", rock_fell);
            //print_chamber(&chamber, Some((shape_y, shape_x, shape)));
            //println!("rr: {}:\t{}", rock_fell, top_level - k);
            pss.push((top_level - k) as u64);
        }
        if rock_fell >= 2022 { break; }
    }

    let (longest_seq_start, longest_seq) = longest_seq3(&pss);
    let non_seq_sum: u64 = pss[0..longest_seq_start].iter().sum();
    let seq_sum: u64 = longest_seq.iter().sum();
    let non_seq_len: u64 = longest_seq_start as u64;
    let elements = 1_000_000_000_000u64;

    pss.iter().for_each(|v| print!("{},", v));
    println!();
    println!("{}", longest_seq_start);
    longest_seq.iter().for_each(|v| print!("{},", v));
    println!();


    let v1 = (elements - non_seq_len as u64) / longest_seq.len() as u64;
    let v2 = (elements - non_seq_len as u64) % longest_seq.len() as u64;

    println!("{}", non_seq_sum + seq_sum * v1 + longest_seq.iter().take(v2 as usize).sum::<u64>());


    top_level
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
        assert_eq!(result, 3068);
    }

    #[test]
    fn sub_test() {
        let test:Vec<u64> = vec![1,2,1,2,2,0,0,2,0,0,1,2,2,0,0,1,3,3,2,0,1,0,3,1,1,1,3,2,2,2,0,0,3,0,2,1,3,3,2,2,1,3,3,0,0,1,3,3,0,2,1,3,2,2,0,1,2,2,0,0,1,2,2,0,2,1,3,3,0,0,1,3,2,1,0,1,0,3,0,2,1,3,2,2,0,1,3,2,2,0,1,3,3,0,0,1,3,3,0,0,0,2,2,0,0,1,2,2,4,0,1,3,0,2,0,1,3,2,0,2,1,3,3,2,0,1,3,2,4,0,1,3,2,0,0,1,3,2,0,0,1,2,2,4,2,0,3,2,0,2,1,2,1,4,0,1,2,2,2,2,1,2,1,2,0,1,3,2,2,0,1,3,3,2,2,1,2,3,4,0,1,3,3,4,2,0,0,3,2,0,1,3,3,0,2,1,2,2,2,2,1,3,2,4,2,1,2,1,3,2,1,3,3,2,2,1,3,0,4,0,1,3,3,0,0,0,2,3,0,2,1,3,2,0,0,1,2,3,4,2,1,3,3,2,0,1,3,0,2,0,1,3,3,0,0,1,3,2,4,2,1,3,3,2,0,1,2,3,4,0,0,2,2,0,0,1,3,2,0,2,1,3,3,0,0,1,3,0,4,0,1,2,3,4,2,1,2,2,4,2,1,3,3,4,0,1,3,2,4,0,1,3,3,2,0,1,2,2,4,0,1,2,2,4,0,1,3,3,2,2,1,3,0,3,0,1,3,3,0,0,1,3,3,2,0,1,3,3,2,0,1,3,3,4,0,1,3,3,2,2,1,2,1,2,0,1,2,3,0,1,1,2,3,0,0,1,3,3,2,2,1,3,0,3,2,1,3,2,4,0,1,2,3,0,1,1,3,0,2,0,1,3,2,2,0,1,3,2,4,0,1,3,2,2,2,1,2,3,0,0,0,3,2,2,0,1,3,3,4,0,1,3,3,2,0,1,3,0,4,0,1,2,3,0,0,1,3,3,2,0,1,2,1,3,0,1,3,3,2,0,1,2,3,4,0,1,3,3,0,0,1,3,2,0,0,1,3,3,0,2,1,0,0,4,2,0,0,1,4,0,1,3,3,4,2,1,3,2,2,0,1,2,3,0,0,1,3,2,2,0,1,2,3,0,0,1,3,2,1,2,1,3,0,4,2,1,2,3,2,0,0,2,3,0,2,1,2,1,2,2,1,3,2,2,2,1,3,3,4,0,1,3,2,0,0,1,3,3,0,2,1,3,3,0,0,1,3,2,0,0,1,3,2,1,0,1,3,2,2,2,1,3,2,0,0,1,3,3,0,0,1,2,3,0,0,0,3,2,0,0,1,3,3,0,2,1,3,3,0,0,1,3,2,2,2,1,2,3,0,2,1,2,2,2,0,1,2,1,3,0,1,3,3,0,0,0,2,3,2,0,1,3,0,4,2,1,1,2,1,1,1,3,3,0,0,0,2,3,2,0,1,3,0,4,0,1,3,3,2,0,1,3,3,2,0,1,3,3,2,0,1,3,3,2,0,1,2,2,2,0,1,3,3,4,0,1,2,3,0,0,0,3,2,4,0,1,2,2,4,2,1,3,0,2,0,1,3,3,2,0,1,2,3,0,2,1,3,3,2,0,1,3,2,2,2,1,2,1,1,0,1,2,2,2,2,1,3,2,0,0,1,3,3,0,0,1,3,3,0,0,1,3,3,4,0,1,2,1,2,0,1,3,3,2,0,1,3,3,4,0,1,3,2,0,0,0,2,2,2,0,0,2,2,2,0,1,2,1,2,0,0,2,2,0,0,1,2,2,1,0,0,3,2,2,0,1,2,2,0,2,1,3,0,4,2,1,3,2,1,0,0,2,1,2,0,1,3,3,2,0,1,3,0,2,2,1,3,3,4,2,1,3,3,0,0,1,1,2,4,0,1,2,3,0,1,1,3,2,2,0,1,3,3,2,2,1,3,3,0,2,1,3,3,0,2,1,3,2,2,0,1,3,0,2,0,1,3,3,0,0,1,3,3,2,0,1,3,3,0,0,1,2,3,0,0,1,3,3,0,0,1,3,3,4,0,1,3,0,2,2,1,3,2,2,0,1,0,0,4,0,1,3,3,0,0,1,2,1,0,2,1,3,3,2,2,1,1,2,4,0,1,3,3,0,0,1,2,3,2,2,1,2,1,3,0,0,3,0,1,1,1,3,3,2,2,1,3,3,0,2,1,3,2,0,0,1,3,0,0,2,1,3,2,4,2,1,2,2,2,2,1,3,3,0,0,1,3,3,2,2,1,3,3,0,2,1,3,3,4,0,1,2,3,4,0,1,3,3,2,2,1,3,0,2,2,1,2,3,2,0,1,2,3,2,0,0,2,0,0,2,1,3,3,4,0,1,3,0,4,2,1,3,2,4,0,1,3,0,4,0,1,3,3,0,0,0,3,2,2,0,1,3,2,4,0,0,0,3,0,0,1,3,2,2,2,0,3,3,2,0,0,2,2,0,0,1,3,3,2,2,1,3,0,3,0,1,3,0,2,0,1,3,2,0,0,1,3,0,4,2,1,3,3,0,0,1,3,3,4,0,0,0,3,4,0,1,2,1,3,0,1,2,1,2,2,1,3,2,4,2,1,3,3,0,0,1,3,3,2,0,1,2,2,2,0,1,2,2,2,2,1,3,3,4,0,0,0,3,0,0,1,3,0,2,2,1,3,3,0,2,1,3,0,2,0,1,3,3,0,0,1,1,3,0,2,1,3,3,0,2,1,2,1,2,2,1,3,3,0,2,0,2,3,0,2,0,3,3,2,0,1,3,3,4,2,1,3,3,0,0,1,3,2,2,2,1,2,2,2,0,1,3,2,4,0,0,0,3,2,0,0,2,2,0,0,1,3,3,0,0,1,3,3,0,0,1,3,3,0,0,1,2,2,1,2,1,2,2,2,0,1,3,3,2,0,1,3,2,2,2,1,3,3,2,0,0,2,0,2,0,1,2,1,2,0,1,3,3,2,0,0,0,3,2,0,1,3,2,2,0,1,2,3,2,0,1,3,3,0,2,1,3,3,4,2,0,0,3,0,0,0,2,3,0,2,1,3,0,3,0,1,3,3,2,0,1,2,1,3,0,0,1,3,0,0,1,2,1,2,0,1,3,3,4,0,1,3,2,0,2,1,2,2,0,0,1,3,0,2,0,1,3,3,2,0,1,3,2,4,0,1,3,3,2,0,1,3,3,0,0,1,2,2,2,0,1,2,3,0,1,1,2,3,0,1,0,3,2,1,0,1,2,2,2,0,1,3,3,4,0,0,2,2,0,0,1,3,3,0,0,1,3,0,0,1,1,0,3,2,2,1,3,2,2,0,1,3,3,0,0,1,2,1,3,0,1,3,0,4,0,0,1,3,0,0,1,3,2,0,0,1,3,2,0,0,0,3,0,1,1,1,3,2,0,2,1,3,3,0,0,1,2,3,0,2,0,2,1,3,0,1,3,3,0,0,1,2,2,2,2,1,2,3,0,0,0,3,3,2,2,1,3,3,0,0,1,2,1,2,0,1,2,1,2,2,1,3,3,4,2,1,3,3,0,0,0,2,0,0,1,1,3,3,0,2,1,3,3,4,0,1,3,3,0,2,1,2,2,2,0,1,3,2,0,0,1,3,2,4,0,1,3,0,4,0,0,0,3,0,2,1,3,2,4,0,1,2,3,4,2,1,3,2,2,0,1,2,3,2,2,1,2,3,0,1,1,3,3,4,2,1,3,2,2,2,1,3,3,2,0,1,3,3,4,0,1,3,0,2,0,1,3,0,1,2,1,3,3,4,2,0,3,2,2,2,1,2,3,2,2,1,2,1,3,2,1,3,2,0,0,1,3,0,3,2,1,0,3,0,0,0,3,3,4,0,1,0,0,4,0,1,3,3,4,0,1,3,2,0,0,0,3,3,0,0,1,1,2,2,0,1,3,2,2,0,0,0,3,1,1,1,3,2,1,0,1,3,3,2,2,1,2,3,0,1,0,3,0,2,0,1,3,3,4,2,0,0,3,2,0,1,3,0,1,1,1,3,3,2,2,1,3,3,0,2,1,3,0,2,2,1,3,3,0,2,1,2,3,0,1,1,2,2,2,0,0,2,3,2,0,1,2,1,2,2,0,0,2,2,0,1,2,3,2,0,1,3,3,2,0,1,2,3,0,0,0,3,2,4,0,1,2,2,4,2,1,3,2,4,0,1,3,2,0,0,1,3,2,0,0,1,2,2,2,2,1,3,3,0,0,1,0,3,0,0,0,3,0,2,0,1,3,2,2,0,1,3,3,2,2,1,2,3,4,0,1,3,3,4,2,0,0,3,2,0,1,3,3,0,2,1,2,2,2,2,1,3,2,4,2,1,2,1,3,2,1,3,3,2,2,1,3,0,4,0,1,3,3,0,0,0,2,3,0,2,1,3,2,0,0,1,2,3,4,2,1,3,3,2,0,1,3,0,2,0,1,3,3,0,0,1,3,2,4,2,1,3,3,2,0,1,2,3,4,0,0,2,2,0,0,1,3];
/*        let test:Vec<u64> = vec![1, 2, 1, 2, 0, 1, 2, 1, 2, 0,   1, 3, 2, 0, 0, 1, 3, 3, 4, 0, 1, 2, 3, 0, 1, 1, 3, 2, 2, 0, 0, 2, 3, 4, 0, 1, 2, 1, 2, 0, 1, 2, 1, 2, 0,
                                 1, 3, 2, 0, 0, 1, 3, 3, 4, 0, 1, 2, 3, 0, 1, 1, 3, 2, 2, 0, 0, 2, 3, 4, 0, 1, 2, 1, 2, 0, 1, 2, 1, 2, 0,    0, 0 , 0];
*/
        //let test: Vec<u64> = vec![1, 2, 2, 1, 2, 3, 4, 1, 2, 3, 4];
        let (_, r) = longest_seq3(&test);
        r.iter().for_each(|v| print!("{v},"));
        println!();
        test.iter().for_each(|v| print!("{v},"));
        println!();
    }
}
