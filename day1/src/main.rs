fn main() {
    let input = include_str!("../input.txt");

    println!("Result part 1: {}", calculate_max_calories(input));
    println!("Result part 2: {}", calculate_top_three_sum(input));
}

fn calculate_top_three_sum(input: &str) -> u32 {
    let top_three: [u32; 3] = input
        .split("\n\n")
        .fold([0; 3], |mut top_three, calories_block| {
            let calories_block_sum = calories_block
                .split("\n")
                .map(|calories_str| {
                    if calories_str.is_empty() {
                        // very last item in input ends with \n
                        return 0;
                    }
                    calories_str.parse::<u32>().unwrap()
                })
                .sum();

            let mut lowest_calories: (usize, u32) = (usize::MAX, u32::MAX);
            let mut is_among_top_three = false;
            for (i, calories) in top_three.into_iter().enumerate() {
                if top_three[i] == 0 {
                    top_three[i] = calories_block_sum;
                    is_among_top_three = false;
                    break;
                }

                if !is_among_top_three {
                    if calories_block_sum > calories {
                        lowest_calories = (i, calories);
                        is_among_top_three = true
                    }
                } else {
                    if lowest_calories.1 > calories {
                        lowest_calories = (i, calories)
                    }
                }
            }

            if is_among_top_three {
                // order of values is irrelevant; replace the lowest value with the new value
                top_three[lowest_calories.0] = calories_block_sum;
            }

            top_three
        });

    return top_three.iter().sum();
}

fn calculate_max_calories(input: &str) -> u32 {
    return input
        .split("\n\n")
        .fold(0, |mut max_calories, calories_block| {
            let calories_block_sum = calories_block
                .split("\n")
                .map(|calories_str| {
                    if calories_str.is_empty() {
                        // very last item in input ends with \n
                        return 0;
                    }
                    calories_str.parse::<u32>().unwrap()
                })
                .sum();

            if calories_block_sum > max_calories {
                max_calories = calories_block_sum
            }
            max_calories
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_demo_input() {
        let input = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let want = 24000;
        assert_eq!(want, calculate_max_calories(input));
    }

    #[test]
    fn test_part2_demo_input() {
        let input = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let want = 45000;
        assert_eq!(want, calculate_top_three_sum(input));
    }
}
