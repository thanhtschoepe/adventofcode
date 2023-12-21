#![feature(if_let_guard)]
use std::{ops::AddAssign, str::FromStr};
advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    power: u32,
    is_possible: bool,
}

#[derive(Default, Debug, PartialEq, PartialOrd)]
struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}
impl GameSet {
    // I looked into implementing max method via Ord trait but that's over implementing
    // so I make this custom method to do that max-merge
    fn max_and_merge(self, other: Self) -> Self {
        GameSet {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
    fn power(self) -> u32 {
        self.red * self.blue * self.green
    }
}
#[derive(Debug, PartialEq)]
enum Color {
    R,
    G,
    B,
}

#[derive(Debug, PartialEq)]
struct ColorNSize(Color, u32);

#[derive(Debug, PartialEq)]
enum GameError {
    ParseError,
}
impl FromStr for Color {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Color::{B, G, R};
        match s {
            "red" => Ok(R),
            "blue" => Ok(B),
            "green" => Ok(G),
            _ => Err(GameError::ParseError),
        }
    }
}
impl AddAssign<ColorNSize> for GameSet {
    fn add_assign(&mut self, rhs: ColorNSize) {
        let ColorNSize(color, size) = rhs;
        match color {
            Color::R => self.red += size,
            Color::G => self.green += size,
            Color::B => self.blue += size,
        };
    }
}
impl FromStr for ColorNSize {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let size_parse = split.next().map(u32::from_str);
        let color_parse = split.next().map(Color::from_str);

        if let (Some(Ok(color)), Some(Ok(size))) = (color_parse, size_parse) {
            Ok(ColorNSize(color, size))
        } else {
            Err(GameError::ParseError)
        }
    }
}
impl FromStr for Game {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, rest) = s.split_once(": ").ok_or(GameError::ParseError)?;
        let id = match game.split_once(' ').ok_or(GameError::ParseError)? {
            (_, id) if let Ok(id) = id.parse::<u32>() => id,
            _ => return Err(GameError::ParseError),
        };
        // Normally I wouldn't overload parsing logic with these sort of computations. Parsing should be about deserializing, i.e give back GameSet
        // But... I wanted to do short-circuiting. It's wasteful to keep parsing more Game set if we know the game is already invalid.
        // For part 2 I can't short-circuit anymore. Oh well! Too late...
        let gameset @ GameSet { red, green, blue } = rest
            .split("; ")
            .map(GameSet::from_str)
            .try_fold(GameSet::default(), |acc, next| {
                next.map(|next| next.max_and_merge(acc))
            })?;
        let is_possible = !(red > 12 || green > 13 || blue > 14);

        Ok(Game {
            id,
            power: gameset.power(),
            is_possible,
        })
    }
}
impl FromStr for GameSet {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(", ")
            .map(ColorNSize::from_str)
            .try_fold(GameSet::default(), |mut acc, next| {
                acc += next?;
                Ok(acc)
            })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    Some(
        lines
            .map(Game::from_str)
            .filter_map(|game| match game {
                Ok(Game {
                    power: _,
                    id,
                    is_possible: true,
                }) => Some(id),
                _ => None,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    Some(
        lines
            .map(Game::from_str)
            .filter_map(|game| game.map(|game| game.power).ok())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        assert_eq!(Color::from_str("red"), Ok(Color::R));
        assert_eq!(Color::from_str("green"), Ok(Color::G));
        assert_eq!(Color::from_str("blue"), Ok(Color::B));
        assert_eq!(Color::from_str("something"), Err(GameError::ParseError))
    }

    #[test]
    fn test_parse_color_n_size() {
        assert_eq!("12 red".parse::<ColorNSize>(), Ok(ColorNSize(Color::R, 12)));
        assert_eq!("-1 red".parse::<ColorNSize>(), Err(GameError::ParseError));
    }

    #[test]
    fn test_parse_gameset() {
        assert_eq!(
            "1 green, 3 red, 6 blue".parse::<GameSet>(),
            Ok(GameSet {
                green: 1,
                red: 3,
                blue: 6
            })
        );
        assert_eq!(
            "3 red, 6 blue, 1 green".parse::<GameSet>(),
            Ok(GameSet {
                green: 1,
                red: 3,
                blue: 6
            })
        );
        assert_eq!(
            "16 blue".parse::<GameSet>(),
            Ok(GameSet {
                green: 0,
                red: 0,
                blue: 16
            })
        );
        assert_eq!(
            "16 yellow, 1 blue, 1 green".parse::<GameSet>(),
            Err(GameError::ParseError)
        );
    }

    #[test]
    fn test_parse_game() {
        let result_pass: Result<Game, GameError> =
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".parse();
        assert_eq!(
            result_pass,
            Ok(Game {
                id: 1,
                power: 48,
                is_possible: true
            })
        );

        let impossible_game: Result<Game, GameError> =
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".parse();
        assert_eq!(
            impossible_game,
            Ok(Game {
                id: 3,
                power: 20 * 13 * 6,
                is_possible: false
            })
        )
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
