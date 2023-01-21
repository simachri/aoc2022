use day15::{calculate_impossible_pos_count, calculate_tuning_frequency};

fn main() {
    let input = include_str!("../input.txt");

    println!(
        "Result of part 1: {}",
        calculate_impossible_pos_count(input, 2_000_000)
    );
    println!(
        "Result of part 2: {}",
        calculate_tuning_frequency(input, 4_000_000,)
    );
}
