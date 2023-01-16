struct PacketDataPair {
    left: PacketDataList,
    right: PacketDataList,
}

#[derive(Debug, PartialEq, Clone)]
enum RawDataEntity {
    SquareBracketOpen,
    SquareBracketClose,
    Integer(u32),
}

type PacketDataList = Vec<PacketData>;

#[derive(Debug, PartialEq, Clone)]
enum PacketData {
    PacketDataList(PacketDataList),
    Integer(u32),
    None,
}

fn main() {
    let input = include_str!("../input.txt");

    println!("Result of day 1: {}", sum_of_indices_in_right_order(input));
}

fn sum_of_indices_in_right_order(input: &str) -> u32 {
    let mut sum: u32 = 0;

    let data_packets = parse_input(input);

    for (idx, data_packet) in data_packets.iter().enumerate() {
        if compare_both_lists(&data_packet.left, &data_packet.right).unwrap() {
            sum += (idx + 1) as u32;
        }
    }

    return sum;
}

fn is_in_right_order(left: &PacketData, right: &PacketData) -> Option<bool> {
    match left {
        PacketData::PacketDataList(list) => compare_left_list(list, right),
        PacketData::Integer(int) => compare_left_integer(*int, right),
        PacketData::None => compare_left_none(right),
    }
}

fn compare_left_none(right_packet: &PacketData) -> Option<bool> {
    match right_packet {
        PacketData::PacketDataList(_) | PacketData::Integer(_) => Some(true),
        PacketData::None => None,
    }
}

fn compare_left_integer(left_int: u32, right_packet: &PacketData) -> Option<bool> {
    match right_packet {
        PacketData::PacketDataList(list) => {
            let left_int_as_list = vec![PacketData::Integer(left_int)];
            return compare_both_lists(&left_int_as_list, list);
        }
        PacketData::Integer(right_int) => {
            println!("Comparing integers {} and {}", left_int, right_int);
            if left_int == *right_int {
                return None;
            } else {
                return Some(left_int < *right_int);
            }
        }
        PacketData::None => {
            // Right list ran out of elements.
            return Some(false);
        }
    }
}

fn compare_left_list(left_list: &PacketDataList, right_packet: &PacketData) -> Option<bool> {
    match right_packet {
        PacketData::PacketDataList(right_list) => compare_both_lists(left_list, right_list),
        PacketData::Integer(right_int) => {
            let right_int_as_list = vec![PacketData::Integer(*right_int)];
            return compare_both_lists(left_list, &right_int_as_list);
        }
        PacketData::None => {
            // Right list ran out of elements.
            return Some(false);
        }
    }
}

fn compare_both_lists(left: &PacketDataList, right: &PacketDataList) -> Option<bool> {
    println!("\nComparing lists:");
    println!("{:?}", left);
    println!("{:?}", right);

    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    loop {
        let left_item = left_iter.next();
        let right_item = right_iter.next();

        if left_item.is_none() && right_item.is_some() {
            return Some(true);
        } else if left_item.is_some() && right_item.is_none() {
            return Some(false);
        } else if left_item.is_none() && right_item.is_none() {
            return None;
        }

        match is_in_right_order(left_item.unwrap(), right_item.unwrap()) {
            Some(true) => return Some(true),
            Some(false) => return Some(false),
            None => continue,
        }
    }
}

fn parse_input(input: &str) -> Vec<PacketDataPair> {
    let mut result: Vec<PacketDataPair> = Vec::new();

    for pair in input.split_terminator("\n\n") {
        let (left_raw, right_raw) = pair.split_once("\n").unwrap();
        let prepared_data_left = prepare_raw_packet_data(left_raw);
        let prepared_data_right = prepare_raw_packet_data(right_raw);

        result.push(PacketDataPair {
            left: scan_packet_data_list(&mut prepared_data_left.iter(), true),
            right: scan_packet_data_list(&mut prepared_data_right.iter(), true),
        });
    }

    return result;
}

fn prepare_raw_packet_data(data: &str) -> Vec<RawDataEntity> {
    let mut result = Vec::new();

    for entity in data.split_terminator(",") {
        let mut skip_next = false;

        for (idx, char) in entity.chars().enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }

            match char {
                '1' => {
                    let next_char = entity.chars().nth(idx + 1);
                    match next_char {
                        Some(char) => {
                            if char == '0' {
                                result.push(RawDataEntity::Integer(10));
                                skip_next = true;
                            } else {
                                result.push(RawDataEntity::Integer(1));
                            }
                        }
                        None => result.push(RawDataEntity::Integer(1)),
                    }
                }
                '0' | '2'..='9' => result.push(RawDataEntity::Integer(char.to_digit(10).unwrap())),
                '[' => result.push(RawDataEntity::SquareBracketOpen),
                ']' => result.push(RawDataEntity::SquareBracketClose),
                _ => (),
            }
        }
    }

    return result;
}

fn scan_packet_data_list<'a>(
    data_iter: &mut impl Iterator<Item = &'a RawDataEntity>,
    is_root_call: bool,
) -> PacketDataList {
    let mut result_list: PacketDataList = Vec::new();

    loop {
        let next_item = data_iter.next();

        match next_item {
            Some(RawDataEntity::SquareBracketOpen) => {
                if result_list.is_empty() && is_root_call {
                    result_list = scan_packet_data_list(data_iter, false);
                } else {
                    result_list.push(PacketData::PacketDataList(scan_packet_data_list(data_iter, false)));
                }
            }
            Some(RawDataEntity::SquareBracketClose) => {
                if result_list.is_empty() {
                    result_list.push(PacketData::None);
                }
                return result_list;
            }
            Some(RawDataEntity::Integer(int)) => {
                result_list.push(PacketData::Integer(*int));
            }
            None => {
                return result_list;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_raw_packet_data() {
        let input = "[1,2,[5,1,[7]],[],4]";

        let want = vec![
            RawDataEntity::SquareBracketOpen,
            RawDataEntity::Integer(1),
            RawDataEntity::Integer(2),
            RawDataEntity::SquareBracketOpen,
            RawDataEntity::Integer(5),
            RawDataEntity::Integer(1),
            RawDataEntity::SquareBracketOpen,
            RawDataEntity::Integer(7),
            RawDataEntity::SquareBracketClose,
            RawDataEntity::SquareBracketClose,
            RawDataEntity::SquareBracketOpen,
            RawDataEntity::SquareBracketClose,
            RawDataEntity::Integer(4),
            RawDataEntity::SquareBracketClose,
        ];

        assert_eq!(want, prepare_raw_packet_data(input));
    }

    #[test]
    fn test_parse_packet_data() {
        let input = "[1,2,[5,1,[7]],[],4]";

        let want = vec![
            PacketData::Integer(1),
            PacketData::Integer(2),
            PacketData::PacketDataList(vec![
                PacketData::Integer(5),
                PacketData::Integer(1),
                PacketData::PacketDataList(vec![PacketData::Integer(7)]),
            ]),
            PacketData::PacketDataList(vec![PacketData::None]),
            PacketData::Integer(4),
        ];

        let prepared_data = prepare_raw_packet_data(input);

        assert_eq!(want, scan_packet_data_list(&mut prepared_data.iter(), true));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");

        assert_eq!(13, sum_of_indices_in_right_order(input));
    }
}
