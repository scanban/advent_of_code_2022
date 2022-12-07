use std::cmp::min;

const DISK_SIZE_TOTAL: usize = 70_000_000;
const DISK_SIZE_REQUIRED_FREE: usize = 30_000_000;
const OCCUPIED_SIZE_SIZE_PROD: usize = 45_349_983;

fn calculate_dir(result: &mut usize, req_size: usize, lines: &mut dyn Iterator<Item=&str>) -> usize {
    let mut sz: usize = 0;

    loop {
        match lines.next() {
            Some(l) => {
                if l.starts_with("$ cd") {
                    let dir = l.split(' ').skip(2).next().unwrap();
                    if dir.eq("..") {
                        break;
                    } else {
                        sz += calculate_dir(result, req_size, lines);
                    }
                } else if !l.starts_with("dir") && !l.starts_with("$ ls") {
                    sz += l.split(' ').next().unwrap().parse::<usize>().unwrap();
                }
            }
            _ => {
                break;
            }
        }
    }
    if sz >= req_size {
        *result = min(sz, *result);
    }
    sz
}

fn solve_problem(input_data: &str, req_size: usize) -> usize {
    let mut result = usize::MAX;

    println!("total occupied size: {}", calculate_dir(&mut result, req_size,
                                                      &mut input_data.lines().into_iter()));
    result
}

pub fn solve() -> usize {
    solve_problem(include_str!("../input.txt"),
                  DISK_SIZE_REQUIRED_FREE - (DISK_SIZE_TOTAL - OCCUPIED_SIZE_SIZE_PROD))
}


#[cfg(test)]
mod tests {
    use super::*;

    const OCCUPIED_SIZE_SIZE_TEST: usize = 48_381_165;

    #[test]
    fn solve_test() {
        let result = solve_problem(include_str!("../input_test.txt"),
                                   DISK_SIZE_REQUIRED_FREE - (DISK_SIZE_TOTAL - OCCUPIED_SIZE_SIZE_TEST));
        assert_eq!(result, 24933642);
    }
}
