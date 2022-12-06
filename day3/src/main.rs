fn main() {
    let input = include_str!("../input.txt");

    let result_day_1 = calculate_sum_day_1(input);

    println!("Result of day 1: {}", result_day_1);
}

fn calculate_sum_day_1(input: &str) -> u32 {
    return input.lines().fold(0, |mut sum, rucksack| {
        let mut double_item = '1';

        let items_per_compartment = rucksack.len() / 2;
        for item_left in rucksack.chars().take(items_per_compartment) {

            for item_right in rucksack.chars().skip(items_per_compartment) {
                if item_left == item_right {
                    double_item = item_left;
                    break;
                }
            }

            if !double_item.is_numeric() {
                break;
            }
        }
        assert!(!double_item.is_numeric());

        sum += get_priority(double_item);

        return sum;
    });
}

fn get_priority(char: char) -> u32 {
    if char.is_lowercase() {
        // 'a' starts at unicode scalar value 96.
        // Needs to be mapped to base 1.
        return char as u32 - 96;
    } else {
        // 'A' starts at unicode scalar value 65.
        // Needs to be mapped to base 27.
        return char as u32 - 38;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        let input = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

        let got = calculate_sum_day_1(input);
        let want = 157;

        assert_eq!(want, got);
    }
}
