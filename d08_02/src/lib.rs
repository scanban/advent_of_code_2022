use std::cmp::max;

fn solve_problem(input_data: &str) -> usize {
    let mut trees: Vec<Vec<u8>> = Vec::new();

    for line in input_data.lines() {
        trees.push(line.as_bytes().to_vec());
    }

    let mut result = 0usize;

    for tree_row in 1..trees.len() - 1 {
        for tree_column in 1..trees[tree_row].len() - 1 {

            // go top
            let mut distance_top = 0usize;
            for r in (0..tree_row).rev() {
                distance_top += 1;
                if trees[r][tree_column] >= trees[tree_row][tree_column] {
                    break;
                }
            }

            // go left
            let mut distance_left = 0usize;
            for r in (0..tree_column).rev() {
                distance_left += 1;
                if trees[tree_row][r] >= trees[tree_row][tree_column] {
                    break;
                }
            }

            // go right
            let mut distance_right = 0usize;
            for r in tree_column + 1..trees[tree_row].len() {
                distance_right += 1;
                if trees[tree_row][r] >= trees[tree_row][tree_column] {
                    break;
                }
            }

            // go down
            let mut distance_bottom = 0usize;
            for r in tree_row + 1..trees.len() {
                distance_bottom += 1;
                if trees[r][tree_column] >= trees[tree_row][tree_column] {
                    break;
                }
            }
            result = max(result, distance_top * distance_left * distance_right * distance_bottom)
        }
    }
    result
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
        assert_eq!(result, 8);
    }
}
