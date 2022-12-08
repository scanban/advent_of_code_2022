fn solve_problem(input_data: &str) -> usize {
    let mut trees: Vec<Vec<u8>> = Vec::new();

    for line in input_data.lines() {
        trees.push(line.as_bytes().to_vec());
    }

    let mut visible_trees = trees.len() * 2 + (trees[0].len() - 2) * 2;

    for tree_row in 1..trees.len() - 1 {
        for tree_column in 1..trees[tree_row].len() - 1 {

            // go top
            let mut tree_is_visible = true;
            for r in (0..tree_row).rev() {
                if trees[r][tree_column] >= trees[tree_row][tree_column] {
                    tree_is_visible = false;
                    break;
                }
            }
            if tree_is_visible {
                visible_trees += 1;
                continue;
            }

            // go left
            let mut tree_is_visible = true;
            for r in (0..tree_column).rev() {
                if trees[tree_row][r] >= trees[tree_row][tree_column] {
                    tree_is_visible = false;
                    break;
                }
            }
            if tree_is_visible {
                visible_trees += 1;
                continue;
            }

            // go right
            let mut tree_is_visible = true;
            for r in tree_column + 1..trees[tree_row].len() {
                if trees[tree_row][r] >= trees[tree_row][tree_column] {
                    tree_is_visible = false;
                    break;
                }
            }
            if tree_is_visible {
                visible_trees += 1;
                continue;
            }

            // go down
            let mut tree_is_visible = true;
            for r in tree_row + 1..trees.len() {
                if trees[r][tree_column] >= trees[tree_row][tree_column] {
                    tree_is_visible = false;
                    break;
                }
            }
            if tree_is_visible {
                visible_trees += 1;
            }
        }
    }
    visible_trees
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
        assert_eq!(result, 21);
    }
}
