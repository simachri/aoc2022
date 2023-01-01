use std::collections::HashSet;

enum HeadsRelationToTail {
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

struct State {
    pos_h: (i32, i32),
    pos_t: (i32, i32),
    relation: HeadsRelationToTail,
}

fn main() {
    let input = include_str!("../input.txt");

    let result_part1 = calculate_visited_tail_pos(input);

    println!("Result of part 1: {}", result_part1);
}

fn calculate_visited_tail_pos(input: &str) -> u32 {
    let mut state = State {
        pos_h: (0, 0),
        pos_t: (0, 0),
        relation: HeadsRelationToTail::Overlapping,
    };

    let mut tail_visited_positions = HashSet::new();
    tail_visited_positions.insert(state.pos_t);

    for instruction_direction_stepsize in input.lines().map(|line| line.split_once(" ").unwrap()) {
        state = apply_movement_instruction(
            instruction_direction_stepsize.0,
            u32::from_str_radix(instruction_direction_stepsize.1, 10).unwrap(),
            state,
            &mut tail_visited_positions,
        );
    }

    return tail_visited_positions.len() as u32;
}

fn apply_movement_instruction(
    direction: &str,
    step_size: u32,
    mut state: State,
    tail_visited_positions: &mut HashSet<(i32, i32)>,
) -> State {
    for _ in 0..step_size {
        update_state_from_head_movement(&mut state, direction);

        if move_tail(&mut state) {
            tail_visited_positions.insert(state.pos_t);
        };

        update_state_after_tail_movement(&mut state);
    }

    return state;
}

fn update_state_after_tail_movement(state: &mut State) {
    match state.relation {
        HeadsRelationToTail::Overlapping => return,
        HeadsRelationToTail::Touching => return,
        HeadsRelationToTail::Left => state.relation = HeadsRelationToTail::Touching,
        HeadsRelationToTail::Right => state.relation = HeadsRelationToTail::Touching,
        HeadsRelationToTail::Above => state.relation = HeadsRelationToTail::Touching,
        HeadsRelationToTail::Below => state.relation = HeadsRelationToTail::Touching,
        HeadsRelationToTail::AboveRight => state.relation = HeadsRelationToTail::Touching,
        HeadsRelationToTail::AboveLeft => state.relation = HeadsRelationToTail::Touching,
        HeadsRelationToTail::BelowRight => state.relation = HeadsRelationToTail::Touching,
        HeadsRelationToTail::BelowLeft => state.relation = HeadsRelationToTail::Touching,
    }
}

fn move_tail(state: &mut State) -> bool {
    let mut moved = false;

    match state.relation {
        HeadsRelationToTail::Overlapping => {}
        HeadsRelationToTail::Touching => {}
        HeadsRelationToTail::Left => {
            state.pos_t.0 -= 1;
            moved = true
        }
        HeadsRelationToTail::Right => {
            state.pos_t.0 += 1;
            moved = true;
        }
        HeadsRelationToTail::Above => {
            state.pos_t.1 -= 1;
            moved = true
        }
        HeadsRelationToTail::Below => {
            state.pos_t.1 += 1;
            moved = true;
        }
        HeadsRelationToTail::AboveRight => {
            state.pos_t.0 += 1;
            state.pos_t.1 -= 1;
            moved = true;
        }
        HeadsRelationToTail::AboveLeft => {
            state.pos_t.0 -= 1;
            state.pos_t.1 -= 1;
            moved = true;
        }
        HeadsRelationToTail::BelowRight => {
            state.pos_t.0 += 1;
            state.pos_t.1 += 1;
            moved = true;
        }
        HeadsRelationToTail::BelowLeft => {
            state.pos_t.0 -= 1;
            state.pos_t.1 += 1;
            moved = true;
        }
    }

    return moved;
}

fn update_state_from_head_movement(state: &mut State, direction: &str) {
    match direction {
        "U" => state.pos_h.1 -= 1,
        "D" => state.pos_h.1 += 1,
        "L" => state.pos_h.0 -= 1,
        "R" => state.pos_h.0 += 1,
        _ => panic!("Unknown direction"),
    }

    match state.relation {
        HeadsRelationToTail::Overlapping => state.relation = HeadsRelationToTail::Touching,
        HeadsRelationToTail::Touching => set_next_relation(state),
        _ => unreachable!(),
    }
}

fn set_next_relation(state: &mut State) {
    let (head_x, head_y) = state.pos_h;
    let (tail_x, tail_y) = state.pos_t;

    if head_x == tail_x && head_y == tail_y {
        state.relation = HeadsRelationToTail::Overlapping;
    } else if head_x == tail_x {
        if head_y < tail_y - 1 {
            state.relation = HeadsRelationToTail::Above;
        } else if head_y > tail_y + 1 {
            state.relation = HeadsRelationToTail::Below;
        } else {
            state.relation = HeadsRelationToTail::Touching;
        }
    } else if head_y == tail_y {
        if head_x > tail_x + 1 {
            state.relation = HeadsRelationToTail::Right;
        } else if head_x < tail_x - 1 {
            state.relation = HeadsRelationToTail::Left;
        } else {
            state.relation = HeadsRelationToTail::Touching;
        }
    } else if head_x > tail_x + 1 {
        if head_y > tail_y {
            state.relation = HeadsRelationToTail::BelowRight;
        } else {
            state.relation = HeadsRelationToTail::AboveRight;
        }
    } else if head_x < tail_x - 1 {
        if head_y > tail_y {
            state.relation = HeadsRelationToTail::BelowLeft;
        } else {
            state.relation = HeadsRelationToTail::AboveLeft;
        }
    } else if head_y > tail_y + 1 {
        if head_x > tail_x {
            state.relation = HeadsRelationToTail::BelowRight;
        } else {
            state.relation = HeadsRelationToTail::BelowLeft;
        }
    } else if head_y < tail_y - 1 {
        if head_x > tail_x {
            state.relation = HeadsRelationToTail::AboveRight;
        } else {
            state.relation = HeadsRelationToTail::AboveLeft;
        }
    } else {
        state.relation = HeadsRelationToTail::Touching;
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");

        assert_eq!(calculate_visited_tail_pos(input), 13);
    }
}
