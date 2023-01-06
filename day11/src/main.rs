use std::collections::VecDeque;

#[derive(Debug)]
enum Operand {
    Old,
    Digit(u32),
}

#[derive(Debug)]
struct WorryOperation {
    operator: char,
    right_operand: Operand,
}

impl WorryOperation {
    fn execute(&self, old_value: u32) -> u32 {
        let resolved_right_operand: u32;
        match self.right_operand {
            Operand::Old => resolved_right_operand = old_value,
            Operand::Digit(digit) => resolved_right_operand = digit,
        }

        match self.operator {
            '+' => old_value + resolved_right_operand,
            '*' => old_value * resolved_right_operand,
            _ => panic!("Unknown operator"),
        }
    }
}

struct ThrowingItem {
    item: Item,
    test_result: bool,
}

#[derive(Debug)]
struct Item {
    worry_level: u32,
}

impl Item {
    fn inspect(&mut self, update_fn: &WorryOperation) {
        self.worry_level = update_fn.execute(self.worry_level);
    }

    fn get_bored(&mut self) {
        self.worry_level = self.worry_level / 3;
    }
}

#[derive(Debug)]
struct Monkey {
    id: u32,
    items: VecDeque<Item>,
    worry_operation: WorryOperation,
    test_divisor: u32,
    throw_to_monkey_id_if_true: u32,
    throw_to_monkey_id_if_false: u32,
    inspection_count: u32,
}

impl Monkey {
    fn new(
        id: u32,
        initial_items: VecDeque<Item>,
        worry_operation: WorryOperation,
        test_divisor: u32,
        throw_to_monkey_id_if_true: u32,
        throw_to_monkey_id_if_false: u32,
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

    fn inspect_item(&mut self) {
        let item = self.items.get_mut(0).unwrap();

        println!("Monkey {} inspects item {}.", self.id, item.worry_level);

        item.inspect(&self.worry_operation);
        self.inspection_count += 1;

        println!("New worry level after inspection: {}", item.worry_level);

        item.get_bored();

        println!("New worry level after getting bored: {}", item.worry_level);
    }

    fn throw_next_item(&mut self) -> ThrowingItem {
        let item = self.items.pop_front().unwrap();

        match item.worry_level % self.test_divisor {
            0 => ThrowingItem {
                item,
                test_result: true,
            },
            _ => ThrowingItem {
                item,
                test_result: false,
            },
        }
    }
}

struct MonkeyGame {
    monkeys: Vec<Monkey>,
}

impl MonkeyGame {
    fn new(monkeys: Vec<Monkey>) -> MonkeyGame {
        return MonkeyGame { monkeys };
    }

    fn get_mutable_monkeys(
        &mut self,
        main_monkey_id: u32,
        target_monkey_id_true: u32,
        target_monkey_id_false: u32,
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

    fn play(&mut self, rounds: u32) -> u32 {
        for idx in 0..rounds {
            println!("Playing round {}", idx + 1);

            for monkey_id in 0..self.monkeys.len() {
                println!("It's the turn of monkey {}", monkey_id);

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
                    main_monkey.inspect_item();
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
            }
        }

        self.multiply_two_highest_inspection_counts()
    }

    fn multiply_two_highest_inspection_counts(&mut self) -> u32 {
        self.monkeys
            .sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

        let sum = self.monkeys.get(0).unwrap().inspection_count
            * self.monkeys.get(1).unwrap().inspection_count;

        return sum;
    }
}

fn main() {
    let input = include_str!("../input.txt");

    println!("Result of part1: {}", calculate_monkey_business(input, 20));
}

fn calculate_monkey_business(input: &str, rounds: u32) -> u32 {
    let monkeys = parse_monkeys_from_input(input);
    let mut monkey_game = MonkeyGame::new(monkeys);

    monkey_game.play(rounds)
}

fn parse_monkeys_from_input(input: &str) -> Vec<Monkey> {
    return input
        .split_terminator("\n\n")
        .map(|monkey| parse_monkey_from_input(monkey))
        .collect();
}

fn parse_monkey_from_input(monkey: &str) -> Monkey {
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
            };
        })
        .collect();

    let worry_operation = parse_worry_operation(input_iter.next().unwrap());

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
        monkey_id,
        initial_items,
        worry_operation,
        divisor,
        target_monkey_id_true,
        target_monkey_id_false,
    );
}

fn parse_worry_operation(input: &str) -> WorryOperation {
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
    let right_operand: Operand;
    if operand_str == "old" {
        right_operand = Operand::Old;
    } else {
        right_operand = Operand::Digit(operand_str.parse().unwrap());
    }

    return WorryOperation {
        operator,
        right_operand,
    };
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
            VecDeque::from(vec![Item { worry_level: 79 }, Item { worry_level: 98 }]),
            WorryOperation {
                operator: '*',
                right_operand: Operand::Digit(19),
            },
            23,
            2,
            3,
        );

        let got = parse_monkey_from_input(monkey_input);

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
        assert_eq!(calculate_monkey_business(input, 20), 10605);
    }
}
