use std::collections::VecDeque;

#[derive(Debug)]
enum WorryOperation {
    AddConstant(u64),
    MultiplyByConstant(u64),
    Square,
    SquarePart2,
    MultiplyByConstantPart2(u64),
    AddConstantPart2(u64),
}

impl WorryOperation {
    fn calculate_new_worry_level(&self, old_value: u64, divisor: Option<u64>) -> u64 {
        match self {
            WorryOperation::AddConstant(c) => old_value + c,
            WorryOperation::MultiplyByConstant(c) => old_value * c,
            WorryOperation::Square => old_value * old_value,
            WorryOperation::SquarePart2 => (old_value * old_value) % divisor.unwrap(),
            WorryOperation::MultiplyByConstantPart2(c) => (old_value * c) % divisor.unwrap(),
            WorryOperation::AddConstantPart2(c) => (old_value + c) % divisor.unwrap(),
        }
    }

    fn is_divisable_by(&self, worry_level: u64, divisor: u64) -> bool {
        if worry_level % divisor == 0 {
            return true;
        } else {
            return false;
        }
    }
}

struct ThrowingItem {
    item: Item,
    test_result: bool,
}

#[derive(Debug)]
struct Item {
    worry_level: u64,
    divisor_product: Option<u64>,
}

impl Item {
    fn inspect(&mut self, update_fn: &WorryOperation) {
        self.worry_level =
            update_fn.calculate_new_worry_level(self.worry_level, self.divisor_product);
    }

    fn get_bored(&mut self) {
        self.worry_level = self.worry_level / 3;
    }
}

#[derive(Debug)]
struct Monkey {
    id: u64,
    items: VecDeque<Item>,
    worry_operation: WorryOperation,
    test_divisor: u64,
    throw_to_monkey_id_if_true: u64,
    throw_to_monkey_id_if_false: u64,
    inspection_count: u64,
}

impl Monkey {
    fn new(
        id: u64,
        initial_items: VecDeque<Item>,
        worry_operation: WorryOperation,
        test_divisor: u64,
        throw_to_monkey_id_if_true: u64,
        throw_to_monkey_id_if_false: u64,
    ) -> Monkey {
        return Monkey {
            id,
            items: initial_items,
            worry_operation,
            test_divisor,
            throw_to_monkey_id_if_true,
            throw_to_monkey_id_if_false,
            inspection_count: 0,
        };
    }

    fn catch_item(&mut self, item: Item) {
        self.items.push_back(item);
    }

    fn has_item(&self) -> bool {
        if self.items.len() > 0 {
            true
        } else {
            false
        }
    }

    fn inspect_item(&mut self, is_part1: bool) {
        let item = self.items.get_mut(0).unwrap();

        println!("Monkey {} inspects item {}.", self.id, item.worry_level);
        self.inspection_count += 1;

        item.inspect(&self.worry_operation);
        println!("New worry level after inspection: {}", item.worry_level);

        if is_part1 {
            item.get_bored();
            println!("New worry level after getting bored: {}", item.worry_level);
        }
    }

    fn throw_next_item(&mut self) -> ThrowingItem {
        let item = self.items.pop_front().unwrap();
        let worry_level = item.worry_level;

        ThrowingItem {
            item,
            test_result: self
                .worry_operation
                .is_divisable_by(worry_level, self.test_divisor),
        }
    }
}

struct MonkeyGame {
    monkeys: Vec<Monkey>,
    is_part1: bool,
}

impl MonkeyGame {
    fn new(monkeys: Vec<Monkey>, is_part1: bool) -> MonkeyGame {
        return MonkeyGame { monkeys, is_part1 };
    }

    fn get_mutable_monkeys(
        &mut self,
        main_monkey_id: u64,
        target_monkey_id_true: u64,
        target_monkey_id_false: u64,
    ) -> (
        Option<&mut Monkey>,
        Option<&mut Monkey>,
        Option<&mut Monkey>,
    ) {
        let mut main_monkey: Option<&mut Monkey> = None;
        let mut target_monkey_true: Option<&mut Monkey> = None;
        let mut target_monkey_false: Option<&mut Monkey> = None;

        let mut current_monkey: &mut [Monkey];
        let length = self.monkeys.len() + 1;
        let mut rest: &mut [Monkey] = &mut self.monkeys;

        for _ in 1..length {
            (current_monkey, rest) = rest.split_at_mut(1);
            if current_monkey[0].id == main_monkey_id {
                main_monkey = Some(&mut current_monkey[0]);
            } else if current_monkey[0].id == target_monkey_id_true {
                target_monkey_true = Some(&mut current_monkey[0]);
            } else if current_monkey[0].id == target_monkey_id_false {
                target_monkey_false = Some(&mut current_monkey[0]);
            }
        }

        (main_monkey, target_monkey_true, target_monkey_false)
    }

    fn play(&mut self, rounds: u64) -> u64 {
        let is_part1 = self.is_part1;

        for idx in 0..rounds {
            println!("\nPlaying round {}", format!("{:02}", idx + 1));

            for monkey_id in 0..self.monkeys.len() {
                println!("\nIt's the turn of monkey {}", monkey_id);

                let monkey = &self.monkeys[monkey_id];

                let mutable_monkeys = self.get_mutable_monkeys(
                    monkey.id,
                    monkey.throw_to_monkey_id_if_true,
                    monkey.throw_to_monkey_id_if_false,
                );
                let main_monkey = mutable_monkeys.0.unwrap();
                let target_monkey_true = mutable_monkeys.1.unwrap();
                let target_monkey_false = mutable_monkeys.2.unwrap();

                while main_monkey.has_item() {
                    main_monkey.inspect_item(is_part1);
                    let throwing_item = main_monkey.throw_next_item();

                    if throwing_item.test_result {
                        println!(
                            "Monkey {} throws item {} to monkey {}.",
                            monkey_id, throwing_item.item.worry_level, target_monkey_true.id
                        );
                        target_monkey_true.catch_item(throwing_item.item);
                    } else {
                        println!(
                            "Monkey {} throws item {} to monkey {}.",
                            monkey_id, throwing_item.item.worry_level, target_monkey_false.id
                        );
                        target_monkey_false.catch_item(throwing_item.item);
                    }
                }

                println!("Turn of monkey {} finished.", monkey_id);
                println!("Inspection count: {}", main_monkey.inspection_count);
            }

            println!("\nRound {} finished. Inspection counts:", idx + 1);
            for monkey in &self.monkeys {
                println!(
                    "Monkey {} inspected items {} times.",
                    monkey.id, monkey.inspection_count
                );
            }
        }

        self.multiply_two_highest_inspection_counts()
    }

    fn multiply_two_highest_inspection_counts(&mut self) -> u64 {
        self.monkeys
            .sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

        let result = self.monkeys.get(0).unwrap().inspection_count
            * self.monkeys.get(1).unwrap().inspection_count;

        return result;
    }
}

fn main() {
    let input = include_str!("../input.txt");

    println!(
        "Result of part1: {}",
        calculate_monkey_business(input, 20, true, None)
    );

    // For part 2: As all the divisors are pairwise coprime integers, the Chinese Remainder
    // Theorem can be used.
    println!(
        "Result of part2: {}",
        calculate_monkey_business(input, 10000, false, Some(2 * 3 * 5 * 7 * 11 * 13 * 17 * 19))
    );
}

fn calculate_monkey_business(
    input: &str,
    rounds: u64,
    is_part1: bool,
    divisor: Option<u64>,
) -> u64 {
    let monkeys = parse_monkeys_from_input(input, is_part1, divisor);
    let mut monkey_game = MonkeyGame::new(monkeys, is_part1);

    monkey_game.play(rounds)
}

fn parse_monkeys_from_input(input: &str, is_part1: bool, divisor: Option<u64>) -> Vec<Monkey> {
    return input
        .split_terminator("\n\n")
        .map(|monkey| parse_monkey_from_input(monkey, is_part1, divisor))
        .collect();
}

fn parse_monkey_from_input(monkey: &str, is_part1: bool, divisor: Option<u64>) -> Monkey {
    // Monkey 0:
    //   Starting items: 79, 98
    //   Operation: new = old * 19
    //   Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3
    let mut input_iter = monkey.lines();

    let monkey_id = input_iter
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .chars()
        .nth(0)
        .unwrap()
        .to_digit(10)
        .unwrap();

    let initial_items = input_iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(2)
        .map(|mut item| {
            match item.strip_suffix(",") {
                Some(v) => item = v,
                None => (),
            };

            return Item {
                worry_level: item.parse().unwrap(),
                divisor_product: divisor,
            };
        })
        .collect();

    let worry_operation = parse_worry_operation(input_iter.next().unwrap(), is_part1);

    let divisor = input_iter
        .next()
        .unwrap()
        .split_whitespace()
        .nth(3)
        .unwrap()
        .parse()
        .unwrap();

    let target_monkey_id_true = input_iter
        .next()
        .unwrap()
        .split_whitespace()
        .nth(5)
        .unwrap()
        .parse()
        .unwrap();
    let target_monkey_id_false = input_iter
        .next()
        .unwrap()
        .split_whitespace()
        .nth(5)
        .unwrap()
        .parse()
        .unwrap();

    return Monkey::new(
        monkey_id.into(),
        initial_items,
        worry_operation,
        divisor,
        target_monkey_id_true,
        target_monkey_id_false,
    );
}

fn parse_worry_operation(input: &str, is_part1: bool) -> WorryOperation {
    let mut worry_operation_input = input
        .split_once(" = ")
        .unwrap()
        .1
        .split_whitespace()
        .skip(1);

    let operator = worry_operation_input
        .next()
        .unwrap()
        .chars()
        .nth(0)
        .unwrap();

    let operand_str = worry_operation_input.next().unwrap();

    if operator == '+' {
        if is_part1 {
            return WorryOperation::AddConstant(operand_str.parse().unwrap());
        } else {
            return WorryOperation::AddConstantPart2(operand_str.parse().unwrap());
        }
    } else if operator == '*' {
        if operand_str == "old" {
            if is_part1 {
                return WorryOperation::Square;
            } else {
                return WorryOperation::SquarePart2;
            }
        } else {
            if is_part1 {
                return WorryOperation::MultiplyByConstant(operand_str.parse().unwrap());
            } else {
                return WorryOperation::MultiplyByConstantPart2(operand_str.parse().unwrap());
            }
        }
    } else {
        panic!("Unknown operator: {}", operator);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monkey() {
        let monkey_input = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

        let want = Monkey::new(
            0,
            VecDeque::from(vec![
                Item {
                    worry_level: 79,
                    divisor_product: None,
                },
                Item {
                    worry_level: 98,
                    divisor_product: None,
                },
            ]),
            WorryOperation::MultiplyByConstant(19),
            23,
            2,
            3,
        );

        let got = parse_monkey_from_input(monkey_input, false, None);

        assert_eq!(got.id, want.id);
        assert_eq!(
            got.items.get(0).unwrap().worry_level,
            want.items.get(0).unwrap().worry_level
        );
        assert_eq!(
            got.items.get(1).unwrap().worry_level,
            want.items.get(1).unwrap().worry_level
        );
        assert_eq!(
            got.throw_to_monkey_id_if_false,
            want.throw_to_monkey_id_if_false
        );
        assert_eq!(
            got.throw_to_monkey_id_if_true,
            want.throw_to_monkey_id_if_true
        );
        assert_eq!(got.test_divisor, want.test_divisor);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(calculate_monkey_business(input, 20, true, None), 10605);
    }

    #[test]
    fn test_part2_one_round() {
        let input = include_str!("../test.txt");
        assert_eq!(
            calculate_monkey_business(input, 1, false, Some(13 * 17 * 19 * 23)),
            4 * 6
        );
    }

    #[test]
    fn test_part2_twenty_rounds() {
        let input = include_str!("../test.txt");
        assert_eq!(
            calculate_monkey_business(input, 20, false, Some(13 * 17 * 19 * 23)),
            99 * 103
        );
    }

    #[test]
    fn test_part2_all_rounds() {
        let input = include_str!("../test.txt");
        assert_eq!(
            calculate_monkey_business(input, 10000, false, Some(13 * 17 * 19 * 23)),
            2713310158
        );
    }
}
