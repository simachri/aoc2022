struct Tree {
    size: u8,
    visible: bool,
}

fn main() {
    let input = include_str!("../input.txt");

    let result_part1 = calculate_visible_trees(input);

    println!("Result part 1: {}", result_part1);
}

fn calculate_visible_trees(input: &str) -> u32 {
    let mut visible_tree_count = 0;

    let mut tree_map = init_tree_map(input);

    tree_map = scan_visible_trees_rowwise(tree_map);
    tree_map = scan_visible_trees_columnwise(tree_map);

    for row in tree_map {
        for tree in row {
            if tree.visible {
                visible_tree_count += 1;
            }
        }
    }

    return visible_tree_count;
}

fn scan_visible_trees_rowwise(mut tree_map: Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
    for mut row in tree_map.iter_mut() {
        row = scan_visible_trees_row_lefttoright(row);
        scan_visible_trees_row_righttoleft(row);
    }

    tree_map
}

fn scan_visible_trees_row_righttoleft(trees: &mut Vec<Tree>) -> &mut Vec<Tree> {
    let mut tree_size_to_be_visible = 0;
    let mut is_edge = true;

    for tree in trees.iter_mut().rev() {
        if is_edge {
            tree.visible = true;
            is_edge = false;
            tree_size_to_be_visible = tree.size;
            continue;
        }

        if tree.size > tree_size_to_be_visible {
            tree.visible = true;
            tree_size_to_be_visible = tree.size;
        }
    }

    trees
}

fn scan_visible_trees_row_lefttoright(trees: &mut Vec<Tree>) -> &mut Vec<Tree> {
    let mut tree_size_to_be_visible = 0;
    let mut is_edge = true;

    for tree in trees.iter_mut() {
        if is_edge {
            tree.visible = true;
            is_edge = false;
            tree_size_to_be_visible = tree.size;
            continue;
        }

        if tree.size > tree_size_to_be_visible {
            tree.visible = true;
            tree_size_to_be_visible = tree.size;
        }
    }

    trees
}

fn scan_visible_trees_columnwise(mut tree_map: Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
    for idx in 0..tree_map[0].len() {
        tree_map = scan_visible_trees_column_downward(tree_map, idx);
        tree_map = scan_visible_trees_column_upward(tree_map, idx);
    }

    tree_map
}

fn scan_visible_trees_column_upward(
    mut tree_map: Vec<Vec<Tree>>,
    col_idx: usize,
) -> Vec<Vec<Tree>> {
    let mut tree_size_to_be_visible = 0;
    let mut is_edge = true;

    for tree_row in tree_map.iter_mut().rev() {
        let tree = &mut tree_row[col_idx];

        if is_edge {
            tree.visible = true;
            is_edge = false;
            tree_size_to_be_visible = tree.size;
            continue;
        }

        if tree.size > tree_size_to_be_visible {
            tree.visible = true;
            tree_size_to_be_visible = tree.size;
        }
    }

    tree_map
}

fn scan_visible_trees_column_downward(
    mut tree_map: Vec<Vec<Tree>>,
    col_idx: usize,
) -> Vec<Vec<Tree>> {
    let mut tree_size_to_be_visible = 0;
    let mut is_edge = true;

    for tree_row in tree_map.iter_mut() {
        let tree = &mut tree_row[col_idx];

        if is_edge {
            tree.visible = true;
            is_edge = false;
            tree_size_to_be_visible = tree.size;
            continue;
        }

        if tree.size > tree_size_to_be_visible {
            tree.visible = true;
            tree_size_to_be_visible = tree.size;
        }
    }

    tree_map
}

fn init_tree_map(input: &str) -> Vec<Vec<Tree>> {
    let mut tree_map: Vec<Vec<Tree>> = Vec::new();

    for line in input.lines() {
        let mut tree_line: Vec<Tree> = Vec::new();

        for tree_size in line.chars().map(|c| c.to_digit(10).unwrap()) {
            let tree = Tree {
                size: tree_size as u8,
                visible: false,
            };

            tree_line.push(tree);
        }

        tree_map.push(tree_line);
    }

    return tree_map;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(21, calculate_visible_trees(input));
    }
}
