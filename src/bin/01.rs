use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let list = input.lines();
    list.map(|line| {
        // iterate from both end, yield the first encountered digits

        let first_digit = line.chars().find(|c| c.is_digit(10));
        let last_digit = line.chars().rev().find(|c| c.is_digit(10));

        // if both are found, parse them and return the sum
        let line_num = [first_digit, last_digit]
            .into_iter()
            .flatten()
            .collect::<String>()
            .parse::<u32>()
            .unwrap_or(0);

        line_num
    })
    .sum::<u32>()
    .into()
}

trait ExtractDigitFromProse {
    fn extract_digit(&self) -> Option<u32>;
    fn extract_digit_rev(&self) -> Option<u32>;
}

impl ExtractDigitFromProse for [u8] {
    fn extract_digit(&self) -> Option<u32> {
        let mut buffer = Vec::with_capacity(5);
        let map = HashMap::from(
            [
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ]
            .map(|(k, v)| (k.as_bytes(), v)),
        );

        // loop through the bytes, interpret them as ascii char, and check if it is a digit against the map
        // inner loop advances the buffer to the next 5 bytes, checking against the map if it is a digit
        // advances the buffer is already 5 bytes long (the longest digit is 5 bytes long)
        // short-circuit: if the first char is a digit, return early
        for i in 0..self.len() {
            if (self[i] as char).is_ascii_digit() {
                return (self[i] as char).to_digit(10).into();
            }
            let mut j = 0;
            while let Some(&byte) = self.get(i + j) {
                if j >= 5 {
                    break;
                }

                buffer.push(byte);
                if let Some(&num) = map.get(&buffer[..]) {
                    return num.into();
                }

                j += 1;
            }
            buffer.clear();
        }

        None
    }

    fn extract_digit_rev(&self) -> Option<u32> {
        let mut buffer = Vec::with_capacity(5);
        let map = HashMap::from(
            [
                ("eno", 1),
                ("owt", 2),
                ("eerht", 3),
                ("ruof", 4),
                ("evif", 5),
                ("xis", 6),
                ("neves", 7),
                ("thgie", 8),
                ("enin", 9),
            ]
            .map(|(k, v)| (k.as_bytes(), v)),
        );

        for i in (0..self.len()).rev() {
            if (self[i] as char).is_ascii_digit() {
                return (self[i] as char).to_digit(10).into();
            }
            let mut j = 0;

            while let Some(&byte) = self.get(i - j) {
                if j >= 5 {
                    break;
                }
                buffer.push(byte);
                if let Some(&num) = map.get(&buffer[..]) {
                    return num.into();
                }

                j += 1;
            }
            buffer.clear()
        }
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    // looks like for this one I need to use window sliding algorithm to scan a substring and detect if that is a digit or not.
    input
        .lines()
        .map(|line| line.as_bytes())
        .map(|line| {
            let first_digit = line.extract_digit().unwrap_or(0);
            let last_digit = line.extract_digit_rev().unwrap_or(0);

            first_digit * 10 + last_digit
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
