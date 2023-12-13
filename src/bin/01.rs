use once_cell::sync::Lazy;
use std::collections::HashMap;

advent_of_code::solution!(1);

static DIGIT_MAP: Lazy<HashMap<&str, u32>> = Lazy::new(|| {
    HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ])
});

static REVERSE_DIGIT_MAP: Lazy<HashMap<&str, u32>> = Lazy::new(|| {
    HashMap::from([
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9),
    ])
});

trait ExtractDigitFromProse {
    fn extract_digit(&self, reverse: bool) -> Option<u32>;
}

impl ExtractDigitFromProse for [u8] {
    fn extract_digit(&self, reverse: bool) -> Option<u32> {
        let mut buffer = Vec::with_capacity(5);
        let iterator: Box<dyn Iterator<Item = usize>> = if reverse {
            Box::new((0..self.len()).rev())
        } else {
            Box::new(0..self.len())
        };

        for i in iterator {
            if let Some(&byte) = self.get(i) {
                if (byte as char).is_ascii_digit() {
                    return (byte as char).to_digit(10);
                }

                let mut j = 0;
                while let Some(&byte) = if reverse {
                    self.get(i.saturating_sub(j))
                } else {
                    self.get(i + j)
                } {
                    if j >= 5 {
                        break;
                    }
                    buffer.push(byte);

                    let lookup_key = std::str::from_utf8(&buffer).unwrap();
                    let map = if reverse {
                        &REVERSE_DIGIT_MAP
                    } else {
                        &DIGIT_MAP
                    };
                    if let Some(&num) = map.get(lookup_key) {
                        return num.into();
                    }

                    j += 1;
                }
                buffer.clear();
            }
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let first_digit = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .map_or(0u32, |c| c.to_digit(10).unwrap());

            let last_digit = line
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .map_or(0u32, |c| c.to_digit(10).unwrap());

            first_digit * 10 + last_digit
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let line_bytes = line.as_bytes();

            let first_digit = line_bytes.extract_digit(false).unwrap_or(0);
            let last_digit = line_bytes.extract_digit(true).unwrap_or(0);

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
