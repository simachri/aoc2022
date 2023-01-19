use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

type BeaconAndSensorPos = Vec<Pos>;

#[derive(Debug, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Sensor {
    pos: Pos,
    range: u32,
}

fn main() {
    let input = include_str!("../input.txt");

    println!(
        "Result of part 1: {}",
        calculate_impossible_pos_count(input, 2_000_000)
    );
}

fn calculate_impossible_pos_count(input: &str, y: u32) -> u32 {
    let mut impossible_positions_x: HashSet<i32> = HashSet::new();

    let (sensors, beacon_and_sensor_pos) = parse_sensors(input);

    for sensor in sensors {
        let found_positions_x = calculate_impossible_positions_x(&sensor.pos, sensor.range, y);

        if found_positions_x.is_none() {
            continue;
        }

        for position_x in found_positions_x.unwrap() {
            if beacon_and_sensor_pos.contains(&Pos {
                x: position_x,
                y: y.try_into().unwrap(),
            }) {
                continue;
            }

            impossible_positions_x.insert(position_x);
        }
    }

    return impossible_positions_x.len() as u32;
}

/// Returns the x coordinates of all impossible positions for a given sensor and a given baseline
/// (y coordinate).
///  7 .#########S#######S#........
///  8 ..#################.........
///  9 ...###############..........
/// 10 ....B############...........
/// 11 ..S..###########............
/// 12 ......#########.............
/// 13 -------#######-------------- baseline
/// 14 ........#####.S.......S.....
/// 15 B........###................
/// 16 ..........#SB...............
fn calculate_impossible_positions_x(pos: &Pos, range: u32, baseline_y: u32) -> Option<Vec<i32>> {
    let mut impossible_positions_x: Vec<i32> = Vec::new();

    let distance: i32 =
        range as i32 - (pos.y as i32).abs_diff(baseline_y.try_into().unwrap()) as i32;

    if distance < 0 {
        return None;
    }

    for impossible_position_x in pos.x - distance as i32..=pos.x + distance as i32 {
        impossible_positions_x.push(impossible_position_x);
    }

    return Some(impossible_positions_x);
}

fn parse_sensors(input: &str) -> (Vec<Sensor>, BeaconAndSensorPos) {
    let mut sensors: Vec<Sensor> = Vec::new();
    let mut beacon_and_sensor_pos: BeaconAndSensorPos = Vec::new();

    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
        )
        .unwrap();
    }

    for line in input.lines() {
        let matches = RE.captures(line).unwrap();

        let sensor_pos_x = matches.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let sensor_pos_y = matches.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let beacon_pos_x = matches.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let beacon_pos_y = matches.get(4).unwrap().as_str().parse::<i32>().unwrap();

        sensors.push(Sensor {
            pos: Pos {
                x: sensor_pos_x,
                y: sensor_pos_y,
            },
            range: ((beacon_pos_x - sensor_pos_x).abs() + (beacon_pos_y - sensor_pos_y).abs())
                .try_into()
                .unwrap(),
        });

        beacon_and_sensor_pos.push(Pos {
            x: beacon_pos_x,
            y: beacon_pos_y,
        });
        beacon_and_sensor_pos.push(Pos {
            x: sensor_pos_x,
            y: sensor_pos_y,
        });
    }

    return (sensors, beacon_and_sensor_pos);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");

        assert_eq!(calculate_impossible_pos_count(input, 10), 26);
    }

    #[test]
    fn test_parse_sensor_line() {
        let input = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15";

        let want = vec![Sensor {
            pos: Pos { x: 2, y: 18 },
            range: 4 + 3,
        }];

        let (sensors, _) = parse_sensors(input);

        assert_eq!(want, sensors);
    }

    #[test]
    fn test_calculate_impossible_positions_x() {
        //  0 .#########S#########........
        //  1 .###################........
        //  2 ..#################.........
        //  3 ...###############..........
        //  4 .....############...........
        //  5 .....###########............
        //  6 ......#########.............
        //  7 .......#######..............
        //  8 --------#####--------------- baseline
        //  9 .........###................
        // 10 ..........#.................
        let pos = Pos { x: 2, y: 0 };
        let range = 10;
        let baseline_y = 8;

        let want = vec![0, 1, 2, 3, 4];

        assert_eq!(
            calculate_impossible_positions_x(&pos, range, baseline_y),
            Some(want)
        );
    }
}
