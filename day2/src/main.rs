fn main() {
    let input = include_str!("../input.txt");

    let result_day_1 = calculate_total_score(input);

    println!("Result day 1: {}", result_day_1);
}

enum GameOutcome {
    Loss,
    Draw,
    Win,
}

fn calculate_total_score(input: &str) -> u32 {
    return input.lines().fold(0, |mut total_score, game_strategy| {
        let picks: (&str, &str) = game_strategy.split_once(' ').unwrap();
        let outcome_score = match play(picks.0, picks.1) {
            GameOutcome::Loss => 0,
            GameOutcome::Draw => 3,
            GameOutcome::Win => 6,
        };

        let selected_shape = match picks.1 {
            "X" => 1, // Rock
            "Y" => 2, // Paper
            "Z" => 3, // Scissors
            _ => unreachable!(),
        };

        total_score = total_score + selected_shape + outcome_score;

        return total_score;
    });
}

fn play(theirs: &str, my: &str) -> GameOutcome {
    match theirs {
        "A" => match my {
            // Rock
            "X" => GameOutcome::Draw, // Rock
            "Y" => GameOutcome::Win,  // Paper
            "Z" => GameOutcome::Loss, // Scissors
            _ => unreachable!(),
        },
        "B" => match my {
            // Paper
            "X" => GameOutcome::Loss, // Rock
            "Y" => GameOutcome::Draw, // Paper
            "Z" => GameOutcome::Win,  // Scissors
            _ => unreachable!(),
        },
        "C" => match my {
            // Scissors
            "X" => GameOutcome::Win,  // Rock
            "Y" => GameOutcome::Loss, // Paper
            "Z" => GameOutcome::Draw, // Scissors
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        let input = r"A Y
B X
C Z
";

        let want = 15;
        let got = calculate_total_score(input);
        assert_eq!(want, got);
    }
}
