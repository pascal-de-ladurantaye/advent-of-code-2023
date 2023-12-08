use std::str::FromStr;

advent_of_code::solution!(2);

#[derive(Debug)]
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hand = Hand {
            red: 0,
            green: 0,
            blue: 0,
        };
        for color in s.split(",") {
            let color = color.trim();
            if let Some((value, color)) = color.split_once(" ") {
                let value = value.parse::<u32>().map_err(|_| ParseHandError)?;
                match color {
                    "red" => hand.red = value,
                    "green" => hand.green = value,
                    "blue" => hand.blue = value,
                    _ => return Err(ParseHandError),
                }
            } else {
                return Err(ParseHandError);
            }
        }
        Ok(hand)
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
    max_hand: Hand,
    min_hand: Hand,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Game {
            id: 0,
            hands: Vec::new(),
            max_hand: Hand {
                red: 0,
                green: 0,
                blue: 0,
            },
            min_hand: Hand {
                red: 0,
                green: 0,
                blue: 0,
            },
        };
        if let Some((id_string, hands)) = s.split_once(":") {
            if let Some((_, id)) = id_string.split_once(" ") {
                game.id = id.parse::<u32>().map_err(|_| ParseGameError)?;
            } else {
                return Err(ParseGameError);
            }
            for hand in hands.split(";") {
                let hand = hand.parse::<Hand>().map_err(|_| ParseGameError)?;
                if hand.red > game.max_hand.red {
                    game.max_hand.red = hand.red;
                }
                if hand.green > game.max_hand.green {
                    game.max_hand.green = hand.green;
                }
                if hand.blue > game.max_hand.blue {
                    game.max_hand.blue = hand.blue;
                }
                if hand.red > game.min_hand.red || game.min_hand.red == 0 {
                    game.min_hand.red = hand.red;
                }
                if hand.green > game.min_hand.green || game.min_hand.green == 0 {
                    game.min_hand.green = hand.green;
                }
                if hand.blue > game.min_hand.blue || game.min_hand.blue == 0 {
                    game.min_hand.blue = hand.blue;
                }
                game.hands.push(hand);
            }
        } else {
            return Err(ParseGameError);
        }
        Ok(game)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    const RED_AMOUNT: u32 = 12;
    const GREEN_AMOUNT: u32 = 13;
    const BLUE_AMOUNT: u32 = 14;
    input
        .lines()
        .map(|line| line.parse::<Game>())
        .filter(|game| game.is_ok())
        .map(|game| game.unwrap())
        .filter(|game| {
            game.max_hand.red <= RED_AMOUNT
                && game.max_hand.green <= GREEN_AMOUNT
                && game.max_hand.blue <= BLUE_AMOUNT
        })
        .map(|game| game.id)
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| line.parse::<Game>())
        .filter(|game| game.is_ok())
        .map(|game| game.unwrap())
        .map(|game| game.min_hand.red * game.min_hand.green * game.min_hand.blue)
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

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
