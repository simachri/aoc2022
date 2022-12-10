struct DuplicateResult {
    has_duplicate: bool,
    offset_for_next_window: usize,
}

fn main() {
    let input = include_str!("../input.txt");

    let part_1_result = get_first_marker_pos(input, 4);
    let part_2_result = get_first_marker_pos(input, 14);

    println!("Result of part 1: {}", part_1_result);
    println!("Result of part 2: {}", part_2_result);
}

fn get_first_marker_pos(stream: &str, threshold_unique_chars: usize) -> usize {
    let result: usize;

    let mut windows_end_idx_excl: usize = threshold_unique_chars;
    let mut chars_iter = stream.chars();

    loop {
        let window_iter = chars_iter.clone();
        let window = &String::from_iter(window_iter.take(threshold_unique_chars));

        let duplicate_check_result = has_duplicate(&window);
        if !duplicate_check_result.has_duplicate {
            result = windows_end_idx_excl;
            break;
        }

        for _ in 0..duplicate_check_result.offset_for_next_window {
            chars_iter.next();
        }
        windows_end_idx_excl = windows_end_idx_excl + duplicate_check_result.offset_for_next_window;
    }

    return result;
}

fn has_duplicate(chars: &str) -> DuplicateResult {
    let mut has_duplicate = false;
    let mut offset_for_next_window = 0;

    for i in 0..chars.len() {
        for j in i + 1..chars.len() {
            if chars.chars().nth(i).unwrap() == chars.chars().nth(j).unwrap() {
                has_duplicate = true;
                offset_for_next_window = i + 1;
                break;
            }
        }
    }

    return DuplicateResult {
        has_duplicate,
        offset_for_next_window,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_packet_marker_pos_is_ok() {
        assert_eq!(7, get_first_marker_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
        assert_eq!(5, get_first_marker_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(6, get_first_marker_pos("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(
            10,
            get_first_marker_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)
        );
        assert_eq!(
            11,
            get_first_marker_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4)
        );
    }

    #[test]
    fn test_get_message_marker_pos_is_ok() {
        assert_eq!(
            19,
            get_first_marker_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14)
        );
        assert_eq!(23, get_first_marker_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(23, get_first_marker_pos("nppdvjthqldpwncqszvftbrmjlhg", 14));
        assert_eq!(
            29,
            get_first_marker_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14)
        );
        assert_eq!(
            26,
            get_first_marker_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14)
        );
    }
}
