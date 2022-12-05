fn main() {
    let input = include_str!("../input.txt");

    let result_day_1 = calculate_day_1(input);
    let result_day_2 = calculate_day_2(input);

    println!("Result day 1: {}", result_day_1);
    println!("Result day 2: {}", result_day_2);
}

enum GameOutcome {
    Loss,
    Draw,
    Win,
}

impl GameOutcome {
    fn answer_with(&self, given_pick: GamePick) -> GamePick {
        match given_pick {
            GamePick::Rock => match &self {
                GameOutcome::Loss => GamePick::Scissors,
                GameOutcome::Draw => given_pick,
                GameOutcome::Win => GamePick::Paper,
            },
            GamePick::Paper => match &self {
                GameOutcome::Loss => GamePick::Rock,
                GameOutcome::Draw => given_pick,
                GameOutcome::Win => GamePick::Scissors,
            },
            GamePick::Scissors => match &self {
                GameOutcome::Loss => GamePick::Paper,
                GameOutcome::Draw => given_pick,
                GameOutcome::Win => GamePick::Rock,
            },
        }
    }
}

enum GamePick {
    Rock,
    Paper,
    Scissors,
}

fn calculate_day_1(input: &str) -> u32 {
    return input.lines().fold(0, |mut total_score, game_strategy| {
        let picks: (&str, &str) = game_strategy.split_once(' ').unwrap();
        let outcome_score = match play_strategy_1(picks.0, picks.1) {
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

fn calculate_day_2(input: &str) -> u32 {
    return input.lines().fold(0, |mut total_score, game_strategy| {
        let picks_and_expected_outcome: (&str, &str) = game_strategy.split_once(' ').unwrap();
        let selected_shape =
            match play_strategy_2(picks_and_expected_outcome.0, picks_and_expected_outcome.1) {
                GamePick::Rock => 1,
                GamePick::Paper => 2,
                GamePick::Scissors => 3,
            };

        let outcome_score = match picks_and_expected_outcome.1 {
            "X" => 0, // Loss
            "Y" => 3, // Draw
            "Z" => 6, // Win
            _ => unreachable!(),
        };

        total_score = total_score + selected_shape + outcome_score;

        return total_score;
    });
}

fn play_strategy_1(theirs: &str, my: &str) -> GameOutcome {
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

fn play_strategy_2(theirs: &str, expected_outcome: &str) -> GamePick {
    match theirs {
        "A" => match expected_outcome {
            // Rock
            "X" => GameOutcome::Loss.answer_with(GamePick::Rock), // Loss
            "Y" => GameOutcome::Draw.answer_with(GamePick::Rock), // Draw
            "Z" => GameOutcome::Win.answer_with(GamePick::Rock),  // Win
            _ => unreachable!(),
        },
        "B" => match expected_outcome {
            // Paper
            "X" => GameOutcome::Loss.answer_with(GamePick::Paper), // Loss
            "Y" => GameOutcome::Draw.answer_with(GamePick::Paper), // Draw
            "Z" => GameOutcome::Win.answer_with(GamePick::Paper),  // Win
            _ => unreachable!(),
        },
        "C" => match expected_outcome {
            // Scissors
            "X" => GameOutcome::Loss.answer_with(GamePick::Scissors), // Loss
            "Y" => GameOutcome::Draw.answer_with(GamePick::Scissors), // Draw
            "Z" => GameOutcome::Win.answer_with(GamePick::Scissors),  // Win
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
        let got = calculate_day_1(input);
        assert_eq!(want, got);
    }

    #[test]
    fn test_day_2() {
        let input = r"A Y
B X
C Z
";

        let want = 12;
        let got = calculate_day_2(input);
        assert_eq!(want, got);
    }
}
