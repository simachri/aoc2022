use core::ops::Range;

fn main() {
    let input = include_str!("../input.txt");

    let result_day_1 = calculate_range_inclusion_count(input);
    let result_day_2 = calculate_range_overlap_count(input);

    println!("Result of day 1: {}", result_day_1);
    println!("Result of day 2: {}", result_day_2);
}

#[derive(Debug, PartialEq)]
struct BinaryRanges {
    left: u128,
    right: u128,
}

#[derive(Debug, PartialEq)]
struct Ranges {
    left: Range<u8>,
    right: Range<u8>,
}

impl Ranges {
    fn convert_to_binary_representation(&self, max_bit_count: u8) -> BinaryRanges {
        let mut range_1_binary = 0;
        let mut range_2_binary = 0;

        let mut mask: u128;
        for n in 0..max_bit_count {
            let bit_idx = n + 1;
            if bit_idx >= self.left.start && bit_idx <= self.left.end {
                mask = 1 << bit_idx - 1;
                range_1_binary |= mask;
            }
            if bit_idx >= self.right.start && bit_idx <= self.right.end {
                mask = 1 << bit_idx - 1;
                range_2_binary |= mask;
            }
        }

        return BinaryRanges {
            left: range_1_binary,
            right: range_2_binary,
        };
    }
}

impl BinaryRanges {
    fn one_includes_the_other(&self) -> bool {
        let and = self.left & self.right;
        let left_xor = self.left ^ and;
        let right_xor = self.right ^ and;

        if left_xor == 0 || right_xor == 0 {
            return true;
        }
        return false;
    }

    fn one_overlaps_with_the_other(&self) -> bool {
        let and = self.left & self.right;

        if and != 0 {
            return true;
        }
        return false;
    }
}

fn parse_ranges(line: &str) -> Ranges {
    let ranges: (&str, &str);
    ranges = line.split_once(',').unwrap();

    let range_1_begin_end: (&str, &str);
    let range_2_begin_end: (&str, &str);

    range_1_begin_end = ranges.0.split_once('-').unwrap();
    range_2_begin_end = ranges.1.split_once('-').unwrap();

    let range_1 = (
        u8::from_str_radix(range_1_begin_end.0, 10).unwrap(),
        u8::from_str_radix(range_1_begin_end.1, 10).unwrap(),
    );
    let range_2 = (
        u8::from_str_radix(range_2_begin_end.0, 10).unwrap(),
        u8::from_str_radix(range_2_begin_end.1, 10).unwrap(),
    );

    return Ranges {
        left: Range {
            start: range_1.0,
            end: range_1.1,
        },
        right: Range {
            start: range_2.0,
            end: range_2.1,
        },
    };
}

fn calculate_range_overlap_count(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| {
            let ranges = parse_ranges(line);

            let max_bit_count = ranges.left.end.max(ranges.right.end);

            ranges.convert_to_binary_representation(max_bit_count)
        })
        .fold(0, |mut range_overlap_count, binary_ranges| {
            if binary_ranges.one_overlaps_with_the_other() {
                range_overlap_count += 1;
            }
            range_overlap_count
        });
}

fn calculate_range_inclusion_count(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| {
            let ranges = parse_ranges(line);

            let max_bit_count = ranges.left.end.max(ranges.right.end);

            ranges.convert_to_binary_representation(max_bit_count)
        })
        .fold(0, |mut range_inclusion_count, binary_ranges| {
            if binary_ranges.one_includes_the_other() {
                range_inclusion_count += 1;
            }
            range_inclusion_count
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ranges() {
        let want = Ranges {
            left: Range { start: 2, end: 5 },
            right: Range { start: 10, end: 52 },
        };
        let input = "2-5,10-52";
        let got = parse_ranges(input);

        assert_eq!(want, got)
    }

    #[test]
    fn test_convert_range_to_binary() {
        let input = Ranges {
            left: Range { start: 2, end: 5 },
            right: Range { start: 4, end: 8 },
        };
        let want = BinaryRanges {
            left: 0b00011110, // binary number is created from right to left
            right: 0b11111000,
        };

        let got = input.convert_to_binary_representation(8);

        assert_eq!(want, got)
    }

    #[test]
    fn test_part_1() {
        let input = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
        let got = calculate_range_inclusion_count(input);
        let want = 2;

        assert_eq!(want, got);
    }

    #[test]
    fn test_part_2() {
        let input = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
        let got = calculate_range_overlap_count(input);
        let want = 4;

        assert_eq!(want, got);
    }
}
