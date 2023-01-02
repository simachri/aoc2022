use std::collections::HashSet;

#[derive(Clone, Debug)]
enum KnotRelationToFollower {
    Overlapping,
    Touching,
    Left,
    Right,
    Above,
    Below,
    AboveRight,
    AboveLeft,
    BelowRight,
    BelowLeft,
}

#[derive(Clone)]
struct Knot {
    pos_x: i32,
    pos_y: i32,
    next: Box<Option<Knot>>,
    is_tail: bool,
    relation: KnotRelationToFollower,
}

impl Knot {
    fn set_next(&mut self, next: Knot) -> &mut Knot {
        self.next = Box::new(Some(next));
        let next: &mut Knot;
        match self.next.as_mut() {
            Some(n) => next = n,
            None => unreachable!(),
        }
        return next;
    }

    fn get_next(&mut self) -> Option<&mut Knot> {
        match *self.next {
            Some(ref mut next) => Some(next),
            None => None,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let result_part1 = calculate_visited_tail_pos(input, 2);
    let result_part2 = calculate_visited_tail_pos(input, 10);

    println!("Result of part 1: {}", result_part1);
    println!("Result of part 2: {}", result_part2);
}

fn calculate_visited_tail_pos(input: &str, rope_length: u8) -> u32 {
    let mut head = Knot {
        pos_x: 0,
        pos_y: 0,
        next: Box::new(None),
        is_tail: false,
        relation: KnotRelationToFollower::Overlapping,
    };

    let mut knot: &mut Knot = &mut head;
    println!("Creating a rope of length {}.", rope_length);
    for idx in 1..rope_length {
        let mut new_knot = Knot {
            pos_x: 0,
            pos_y: 0,
            next: Box::new(None),
            is_tail: false,
            relation: KnotRelationToFollower::Overlapping,
        };

        if idx == rope_length - 1 {
            new_knot.is_tail = true;
        }

        knot = knot.set_next(new_knot);
    }

    let mut tail_visited_positions = HashSet::new();
    tail_visited_positions.insert((0, 0));

    for instruction_direction_stepsize in input.lines().map(|line| line.split_once(" ").unwrap()) {
        head = apply_movement_instruction(
            instruction_direction_stepsize.0,
            u32::from_str_radix(instruction_direction_stepsize.1, 10).unwrap(),
            head,
            &mut tail_visited_positions,
        );
    }

    return tail_visited_positions.len() as u32;
}

fn apply_movement_instruction(
    direction: &str,
    step_size: u32,
    mut rope_head: Knot,
    tail_visited_positions: &mut HashSet<(i32, i32)>,
) -> Knot {
    for _ in 0..step_size {
        move_rope_one_step(&mut rope_head, direction, tail_visited_positions);
    }

    return rope_head;
}

fn move_rope_one_step(
    rope_head: &mut Knot,
    direction: &str,
    tail_visited_positions: &mut HashSet<(i32, i32)>,
) {
    let mut knot = rope_head;
    let mut follower_moved = false;
    let mut knot_id: u32 = 1;

    println!("Moving rope one step {}.", direction);
    update_state_from_rope_head_movement(knot, direction);

    while !knot.is_tail {
        knot_id += 1;

        follower_moved = move_follower(knot);
        update_state_after_follower_movement(knot);

        if !follower_moved {
            println!(
                "Follower (knot {}) didn't move. Rope has moved entirely by one step.",
                knot_id
            );
            break;
        }

        let follower = knot.get_next().unwrap();
        println!(
            "Follower (knot {}) moved to position ({}, {}).",
            knot_id, follower.pos_x, follower.pos_y
        );
        if !follower.is_tail {
            match follower.relation {
                KnotRelationToFollower::Overlapping => {
                    follower.relation = KnotRelationToFollower::Touching
                }
                KnotRelationToFollower::Touching => {
                    set_next_relation(follower);
                }
                _ => unreachable!(),
            }
        }

        knot = follower;
    }

    if follower_moved && knot.is_tail {
        tail_visited_positions.insert((knot.pos_x, knot.pos_y));
        println!("Tail visited position: {}, {}", knot.pos_x, knot.pos_y);
    }
}

fn update_state_after_follower_movement(knot: &mut Knot) {
    match knot.relation {
        KnotRelationToFollower::Overlapping => return,
        KnotRelationToFollower::Touching => return,
        KnotRelationToFollower::Left => knot.relation = KnotRelationToFollower::Touching,
        KnotRelationToFollower::Right => knot.relation = KnotRelationToFollower::Touching,
        KnotRelationToFollower::Above => knot.relation = KnotRelationToFollower::Touching,
        KnotRelationToFollower::Below => knot.relation = KnotRelationToFollower::Touching,
        KnotRelationToFollower::AboveRight => knot.relation = KnotRelationToFollower::Touching,
        KnotRelationToFollower::AboveLeft => knot.relation = KnotRelationToFollower::Touching,
        KnotRelationToFollower::BelowRight => knot.relation = KnotRelationToFollower::Touching,
        KnotRelationToFollower::BelowLeft => knot.relation = KnotRelationToFollower::Touching,
    }
}

fn move_follower(knot: &mut Knot) -> bool {
    let mut follower: &mut Knot;
    match knot.next.as_mut() {
        Some(f) => follower = f,
        None => unreachable!(),
    }
    let mut moved = false;

    match knot.relation {
        KnotRelationToFollower::Overlapping => {}
        KnotRelationToFollower::Touching => {}
        KnotRelationToFollower::Left => {
            follower.pos_x -= 1;
            moved = true
        }
        KnotRelationToFollower::Right => {
            follower.pos_x += 1;
            moved = true;
        }
        KnotRelationToFollower::Above => {
            follower.pos_y -= 1;
            moved = true
        }
        KnotRelationToFollower::Below => {
            follower.pos_y += 1;
            moved = true;
        }
        KnotRelationToFollower::AboveRight => {
            follower.pos_x += 1;
            follower.pos_y -= 1;
            moved = true;
        }
        KnotRelationToFollower::AboveLeft => {
            follower.pos_x -= 1;
            follower.pos_y -= 1;
            moved = true;
        }
        KnotRelationToFollower::BelowRight => {
            follower.pos_x += 1;
            follower.pos_y += 1;
            moved = true;
        }
        KnotRelationToFollower::BelowLeft => {
            follower.pos_x -= 1;
            follower.pos_y += 1;
            moved = true;
        }
    }

    return moved;
}

fn update_state_from_rope_head_movement<'a>(knot: &'a mut Knot, direction: &str) {
    match direction {
        "U" => knot.pos_y -= 1,
        "D" => knot.pos_y += 1,
        "L" => knot.pos_x -= 1,
        "R" => knot.pos_x += 1,
        _ => panic!("Unknown direction"),
    }
    println!("Knot moved to position ({}, {}).", knot.pos_x, knot.pos_y);

    match knot.relation {
        KnotRelationToFollower::Overlapping => knot.relation = KnotRelationToFollower::Touching,
        KnotRelationToFollower::Touching => {
            set_next_relation(knot);
        }
        _ => unreachable!(),
    }
}

fn set_next_relation(knot: &mut Knot) {
    let follower: &Knot;
    match knot.next.as_ref() {
        Some(n) => {
            follower = n;
        }
        None => unreachable!(),
    }

    println!(
        "Calculating relation. Position of knot: ({}, {}). Position of follower: ({}, {})",
        knot.pos_x, knot.pos_y, follower.pos_x, follower.pos_y
    );
    if knot.pos_x == follower.pos_x && knot.pos_y == follower.pos_y {
        knot.relation = KnotRelationToFollower::Overlapping;
    } else if knot.pos_x == follower.pos_x {
        if knot.pos_y < follower.pos_y - 1 {
            knot.relation = KnotRelationToFollower::Above;
        } else if knot.pos_y > follower.pos_y + 1 {
            knot.relation = KnotRelationToFollower::Below;
        } else {
            knot.relation = KnotRelationToFollower::Touching;
        }
    } else if knot.pos_y == follower.pos_y {
        if knot.pos_x > follower.pos_x + 1 {
            knot.relation = KnotRelationToFollower::Right;
        } else if knot.pos_x < follower.pos_x - 1 {
            knot.relation = KnotRelationToFollower::Left;
        } else {
            knot.relation = KnotRelationToFollower::Touching;
        }
    } else if knot.pos_x > follower.pos_x + 1 {
        if knot.pos_y > follower.pos_y {
            knot.relation = KnotRelationToFollower::BelowRight;
        } else {
            knot.relation = KnotRelationToFollower::AboveRight;
        }
    } else if knot.pos_x < follower.pos_x - 1 {
        if knot.pos_y > follower.pos_y {
            knot.relation = KnotRelationToFollower::BelowLeft;
        } else {
            knot.relation = KnotRelationToFollower::AboveLeft;
        }
    } else if knot.pos_y > follower.pos_y + 1 {
        if knot.pos_x > follower.pos_x {
            knot.relation = KnotRelationToFollower::BelowRight;
        } else {
            knot.relation = KnotRelationToFollower::BelowLeft;
        }
    } else if knot.pos_y < follower.pos_y - 1 {
        if knot.pos_x > follower.pos_x {
            knot.relation = KnotRelationToFollower::AboveRight;
        } else {
            knot.relation = KnotRelationToFollower::AboveLeft;
        }
    } else {
        knot.relation = KnotRelationToFollower::Touching;
    }

    println!("New knot relation to follower: {:?}", knot.relation);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test1.txt");

        assert_eq!(calculate_visited_tail_pos(input, 2), 13);
    }

    #[test]
    fn test_part2_smaller_set() {
        let input = include_str!("../test1.txt");

        assert_eq!(calculate_visited_tail_pos(input, 10), 1);
    }

    #[test]
    fn test_part2_bigger_set() {
        let input = include_str!("../test2.txt");

        assert_eq!(calculate_visited_tail_pos(input, 10), 36);
    }
}
