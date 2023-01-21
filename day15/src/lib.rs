use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;

type BeaconAndSensorPos = Vec<Pos>;
type ImpossibleBeaconPos = bool;

struct SearchArea {
    impossible_beacon_pos: Vec<Vec<ImpossibleBeaconPos>>,
}

struct XCoordinatesCoveredBySensors {
    min: i32,
    max: i32,
}

impl fmt::Display for SearchArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.impossible_beacon_pos.iter() {
            for col in row {
                if *col {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Sensor {
    pos: Pos,
    range: u32,
    upper_left_line_segment_coefficient: i32,
    upper_right_line_segment_coefficient: i32,
}

/// General idea: When there is only one position that is not covered by the range of any of the
/// sensors, then there has to be an interception of two sensor's outer line segments (range + 1)
/// that is not covered by any of the sensor's ranges.
#[inline]
pub fn calculate_tuning_frequency(input: &str, boundary: u32) -> u64 {
    let mut distress_beacon: Pos = Pos { x: 0, y: 0 };

    let (sensors, beacon_and_sensor_pos, _) = parse_sensors(input);

    let intersections_of_upper_line_segments =
        calculate_intersections_of_upper_line_segments(&sensors, &beacon_and_sensor_pos, boundary);

    for pos in intersections_of_upper_line_segments {
        if !is_covered_by_any_sensor_range(&pos, &sensors) {
            distress_beacon = pos;
            break;
        }
    }

    return distress_beacon.x as u64 * 4_000_000 + distress_beacon.y as u64;
}

fn calculate_intersections_of_upper_line_segments(
    sensors: &[Sensor],
    beacon_and_sensor_pos: &[Pos],
    boundary: u32,
) -> Vec<Pos> {
    let mut intersections = Vec::new();

    for sensor_a in sensors {
        for sensor_b in sensors {
            if sensor_a == sensor_b {
                continue;
            }

            // The intersection of two lines with slope -1 and -1 is at (x, y) = ((b-a)/2, (b+a)/2)
            // where a and b are the coefficients of the lines in the form of y = mx + a and
            // y = mx + b.
            let intersect_pos_x = (sensor_b.upper_right_line_segment_coefficient
                - sensor_a.upper_left_line_segment_coefficient)
                / 2;

            let intersect_pos_y = (sensor_b.upper_right_line_segment_coefficient
                + sensor_a.upper_left_line_segment_coefficient)
                / 2;

            if !beacon_and_sensor_pos.contains(&Pos {
                x: intersect_pos_x,
                y: intersect_pos_y,
            }) && intersect_pos_x >= 0
                && intersect_pos_x <= boundary as i32
                && intersect_pos_y >= 0
                && intersect_pos_y <= boundary as i32
            {
                intersections.push(Pos {
                    x: intersect_pos_x,
                    y: intersect_pos_y,
                });
            }
        }
    }

    return intersections;
}

pub fn calculate_impossible_pos_count(input: &str, y: u32) -> u32 {
    let mut impossible_beacon_pos_count = 0;

    let (sensors, beacon_and_sensor_pos, x_coordinates_covered_by_sensors) = parse_sensors(input);

    for x in x_coordinates_covered_by_sensors.min..=x_coordinates_covered_by_sensors.max {
        if beacon_and_sensor_pos.contains(&Pos {
            x: x as i32,
            y: y as i32,
        }) {
            continue;
        }

        if is_covered_by_any_sensor_range(&Pos { x, y: y as i32 }, &sensors) {
            impossible_beacon_pos_count += 1;
        }
    }

    return impossible_beacon_pos_count;
}

fn is_covered_by_any_sensor_range(pos: &Pos, sensors: &Vec<Sensor>) -> bool {
    for sensor in sensors {
        let distance_to_sensor = (sensor.pos.x).max(pos.x) - (sensor.pos.x).min(pos.x)
            + (sensor.pos.y).max(pos.y)
            - (sensor.pos.y).min(pos.y);
        if distance_to_sensor <= sensor.range as i32 {
            return true;
        }
    }

    return false;
}

fn parse_sensors(
    input: &str,
) -> (
    Vec<Sensor>,
    BeaconAndSensorPos,
    XCoordinatesCoveredBySensors,
) {
    let mut sensors: Vec<Sensor> = Vec::new();
    let mut beacon_and_sensor_pos: BeaconAndSensorPos = Vec::new();
    let mut x_coordinates_covered_by_sensors = XCoordinatesCoveredBySensors {
        min: i32::MAX,
        max: i32::MIN,
    };

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

        let range = ((beacon_pos_x - sensor_pos_x).abs() + (beacon_pos_y - sensor_pos_y).abs())
            .try_into()
            .unwrap();
        let sensor = Sensor {
            pos: Pos {
                x: sensor_pos_x,
                y: sensor_pos_y,
            },
            range,
            upper_left_line_segment_coefficient: calculate_upper_left_line_segment_coefficient(
                &Pos {
                    x: sensor_pos_x,
                    y: sensor_pos_y,
                },
                range,
            ),
            upper_right_line_segment_coefficient: calculate_upper_right_line_segment_coefficient(
                &Pos {
                    x: sensor_pos_x,
                    y: sensor_pos_y,
                },
                range,
            ),
        };
        let left_range = sensor.pos.x - sensor.range as i32;
        let right_range = sensor.pos.x + sensor.range as i32;

        if left_range < x_coordinates_covered_by_sensors.min {
            x_coordinates_covered_by_sensors.min = left_range;
        }
        if right_range > x_coordinates_covered_by_sensors.max {
            x_coordinates_covered_by_sensors.max = right_range;
        }

        sensors.push(sensor);

        beacon_and_sensor_pos.push(Pos {
            x: beacon_pos_x,
            y: beacon_pos_y,
        });
        beacon_and_sensor_pos.push(Pos {
            x: sensor_pos_x,
            y: sensor_pos_y,
        });
    }

    return (
        sensors,
        beacon_and_sensor_pos,
        x_coordinates_covered_by_sensors,
    );
}

/// Calculates the coefficient of the line segment that is the upper left line segment of the
/// sensor's range. The upper left line segment is marked with + on the diagram.
///
/// The general formula to calculate a line with a known slope (it is always 1) and a known
/// point, is y = m(x-x1) + y1. With x1 := Sx and y1 := Sy + r + 1, we get y = x - Sx + Sy + r + 1.
///
/// The intersection of two lines with slope 1 and -1 is at (x, y) = ((b-a)/2, (b+a)/2)
/// where a and b are the coefficients. The coefficient of the upper left line segment is
/// -Sx + Sy + r + 1.
///
/// ..........+.................
/// .........+#.................
/// ........+###................
/// ....S..+#####...............
/// ......+#######........S.....
/// .....+#########S............
/// ....+###########SB..........
/// ...+#############...........
/// ..+###############..........
/// .+#################.........
/// +#########S#######S#.......
fn calculate_upper_left_line_segment_coefficient(sensor_pos: &Pos, range: u32) -> i32 {
    return -sensor_pos.x + sensor_pos.y + range as i32 + 1;
}

/// Calculates the coefficient of the line segment that is the upper right line segment of the
/// sensor's range. The upper right line segment is marked with + on the diagram.
///
/// The general formula to calculate a line with a known slope (it is always -1) and a known
/// point, is y = m(x-x1) + y1. With x1 := Sx and y1 := Sy + r + 1, we get y = -x + Sx + Sy + r + 1.
///
/// The intersection of two lines with slope 1 and -1 is at (x, y) = ((b-a)/2, (b+a)/2)
/// where a and b are the coefficients. The coefficient of the upper right line segment is
/// Sx + Sy + r + 1.
///
/// ..........+.................
/// ..........#+................
/// .........###+...............
/// ....S...#####+..............
/// .......#######+.......S.....
/// ......#########+............
/// .....###########+B..........
/// ....#############+..........
/// ...###############+.........
/// ..#################+........
/// .#########S#######S#+......
fn calculate_upper_right_line_segment_coefficient(sensor_pos: &Pos, range: u32) -> i32 {
    return sensor_pos.x + sensor_pos.y + range as i32 + 1;
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
    fn test_part2() {
        let input = include_str!("../test.txt");

        assert_eq!(calculate_tuning_frequency(input, 20), 56000011);
    }

    #[test]
    fn test_parse_sensor_line() {
        let input = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15";

        let sensor_pos = Pos { x: 2, y: 18 };
        let want = vec![Sensor {
            pos: sensor_pos,
            range: 4 + 3,
            upper_left_line_segment_coefficient: -2 + 18 + (4 + 3) + 1,
            upper_right_line_segment_coefficient: 2 + 18 + (4 + 3) + 1,
        }];

        let (sensors, _, _) = parse_sensors(input);

        assert_eq!(want, sensors);
    }
}
