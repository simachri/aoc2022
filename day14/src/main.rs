#[derive(Copy, Clone, Debug, PartialEq)]
enum CaveElement {
    Sand,
    Air,
    Rock,
}

#[derive(PartialEq)]
enum SandState {
    Falling,
    Settled,
    Vanished,
}

struct GrainOfSand {
    state: SandState,
    pos: (usize, usize),
}

type CaveSlice = [[CaveElement; 1000]; 170];
type Pos = (u32, u32);

const POURING_SAND_SOURCE_POS: Pos = (0, 500);

fn main() {
    let input = include_str!("../input.txt");

    println!(
        "Result of part 1: {}",
        calculate_amount_of_resting_sand(input)
    );
}

fn calculate_amount_of_resting_sand(input: &str) -> u32 {
    let mut amount_of_resting_sand = 0;

    let mut cave = scan_cave(input);

    loop {
        let sand_state = let_sand_pour(&mut cave, POURING_SAND_SOURCE_POS);

        if sand_state == SandState::Vanished {
            break;
        }

        amount_of_resting_sand += 1;
    }

    return amount_of_resting_sand;
}

fn let_sand_pour(cave: &mut CaveSlice, sand_grain_curr_pos: Pos) -> SandState {
    match calculcate_next_sand_grain_pos(cave, sand_grain_curr_pos) {
        Some(next_pos) => {
            if next_pos == sand_grain_curr_pos {
                cave[sand_grain_curr_pos.0 as usize][sand_grain_curr_pos.1 as usize] =
                    CaveElement::Sand;
                return SandState::Settled;
            } else {
                return let_sand_pour(cave, next_pos);
            }
        }
        None => {
            println!("Sand grain vanished.");
            SandState::Vanished
        }
    }
}

fn calculcate_next_sand_grain_pos(cave: &CaveSlice, sand_grain_curr_pos: Pos) -> Option<Pos> {
    let mut next_row = (sand_grain_curr_pos.0 + 1) as usize;
    let mut next_col = sand_grain_curr_pos.1 as usize;

    if next_row == cave.len() {
        return None;
    }

    (next_row, next_col) = match cave[next_row][next_col] {
        CaveElement::Sand | CaveElement::Rock => {
            if cave[next_row][next_col - 1] == CaveElement::Air {
                (next_row, next_col - 1)
            } else if cave[next_row][next_col + 1] == CaveElement::Air {
                (next_row, next_col + 1)
            } else {
                (
                    sand_grain_curr_pos.0 as usize,
                    sand_grain_curr_pos.1 as usize,
                )
            }
        }
        CaveElement::Air => (next_row, next_col),
    };

    return Some((next_row.try_into().unwrap(), next_col.try_into().unwrap()));
}

fn scan_cave(input: &str) -> CaveSlice {
    let mut cave = [[CaveElement::Air; 1000]; 170];

    for line in input.lines() {
        let mut curr_pos: Option<Pos> = None;
        let mut next_pos: Pos;
        for pos in line.split_terminator("->").map(|coord_raw| {
            let (coord_x, coord_y) = coord_raw.trim().split_once(",").unwrap();
            let pos: Pos = (coord_y.parse().unwrap(), coord_x.parse().unwrap());
            pos
        }) {
            if curr_pos.is_none() {
                curr_pos = Some(pos);
                continue;
            }

            next_pos = pos;

            add_rocks_to_cave(&mut cave, curr_pos.unwrap(), next_pos);

            curr_pos = Some(next_pos);
        }
    }

    return cave;
}

fn add_rocks_to_cave(cave: &mut CaveSlice, start_pos: Pos, end_pos: Pos) {
    let row_from = start_pos.0.min(end_pos.0);
    let row_to = start_pos.0.max(end_pos.0);
    let col_from = start_pos.1.min(end_pos.1);
    let col_to = start_pos.1.max(end_pos.1);

    if row_from == row_to {
        for col in col_from..=col_to {
            cave[row_from as usize][col as usize] = CaveElement::Rock;
        }
    }

    if col_from == col_to {
        for row in row_from..=row_to {
            cave[row as usize][col_from as usize] = CaveElement::Rock;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");

        assert_eq!(24, calculate_amount_of_resting_sand(input));
    }

    #[test]
    fn test_scan_cave() {
        let input = include_str!("../test.txt");

        let mut want = [[CaveElement::Air; 1000]; 170];
        want[4][498] = CaveElement::Rock;
        want[5][498] = CaveElement::Rock;
        want[6][498] = CaveElement::Rock;
        want[6][497] = CaveElement::Rock;
        want[6][496] = CaveElement::Rock;
        want[4][503] = CaveElement::Rock;
        want[4][502] = CaveElement::Rock;
        want[5][502] = CaveElement::Rock;
        want[6][502] = CaveElement::Rock;
        want[7][502] = CaveElement::Rock;
        want[8][502] = CaveElement::Rock;
        want[9][502] = CaveElement::Rock;
        want[9][501] = CaveElement::Rock;
        want[9][500] = CaveElement::Rock;
        want[9][499] = CaveElement::Rock;
        want[9][498] = CaveElement::Rock;
        want[9][497] = CaveElement::Rock;
        want[9][496] = CaveElement::Rock;
        want[9][495] = CaveElement::Rock;
        want[9][494] = CaveElement::Rock;

        let got = scan_cave(input);
        for col_idx in 494..=503 {
            for row_idx in 0..=9 {
                assert_eq!(
                    want[row_idx][col_idx], got[row_idx][col_idx],
                    "row: {}, col: {}",
                    row_idx, col_idx
                );
            }
        }
    }
}
