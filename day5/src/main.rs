fn main() {
    let input = include_str!("../input.txt");

    let result_part_1 = move_crates_by_crane(input);

    println!("Result part 1: {}", result_part_1);
}

fn move_crates_by_crane(input: &str) -> String {
    let result: String;

    let (stacks_raw, movements) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(&stacks_raw);

    apply_movements(movements, &mut stacks);

    result = get_top_crates(&stacks);

    return result;
}

fn get_top_crates(stacks: &Vec<Vec<char>>) -> String {
    let mut result = String::new();

    for stack in stacks.iter() {
        result.push(*stack.last().unwrap());
    }

    return result;
}

fn apply_movements(movements: &str, stacks: &mut Vec<Vec<char>>) -> () {
    for movement_raw in movements.lines() {
        // move 1 from 2 to 1
        let mut move_instr_parts = movement_raw.split_whitespace();

        let zero_based = 1;

        let crates_count: u8 = str::parse(move_instr_parts.nth(1).unwrap()).unwrap();
        let source = str::parse::<usize>(move_instr_parts.nth(1).unwrap()).unwrap() - zero_based;
        let dest = str::parse::<usize>(move_instr_parts.nth(1).unwrap()).unwrap() - zero_based;

        move_crates(stacks, crates_count, source, dest);
    }
}

fn move_crates(stacks: &mut Vec<Vec<char>>, crates_count: u8, src: usize, dest: usize) -> () {
    for _ in 0..crates_count {
        let crate_item = stacks[src].pop().unwrap();
        stacks[dest].push(crate_item);
    }
}

fn parse_stacks(stacks_raw: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    let input_reversed: Vec<&str> = stacks_raw.rsplit('\n').collect();
    for stack_no in input_reversed[0].split_whitespace() {
        let char_pos = input_reversed[0].find(stack_no).unwrap();

        let mut stack: Vec<char> = Vec::new();

        for crates in input_reversed.iter().skip(1) {
            match crates.chars().nth(char_pos) {
                Some(char) => {
                    if !char.is_whitespace() {
                        stack.push(char);
                    }
                }
                None => unreachable!(),
            };
        }

        result.push(stack);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_top_crates_is_cmz_for_test_input() {
        let want = "CMZ";

        let input = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
        let got = move_crates_by_crane(input);

        assert_eq!(want, got);
    }

    #[test]
    fn parse_stacks_returns_expected() {
        let want: Vec<Vec<char>> = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

        let stacks_raw = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3";
        let got = parse_stacks(stacks_raw);

        assert_eq!(want, got);
    }
}
