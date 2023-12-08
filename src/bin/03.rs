use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

advent_of_code::solution!(3);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position(i32, i32);
impl Position {
    fn search_coordinates(&self) -> impl Iterator<Item = Position> + '_ {
        let mut coordinates = Vec::new();
        // previous row
        for i in 0..3 {
            coordinates.push(Position(self.0 - 1 + i as i32, self.1 - 1));
        }
        // current row
        coordinates.push(Position(self.0 - 1, self.1));
        coordinates.push(Position(self.0 + 1, self.1));
        // next row
        for i in 0..3 {
            coordinates.push(Position(self.0 - 1 + i as i32, self.1 + 1));
        }
        coordinates.into_iter().filter(|p| p.0 >= 0 && p.1 >= 0)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Number {
    position: Position,
    value: u32,
    length: u32,
}
impl Number {
    fn search_coordinates(&self) -> impl Iterator<Item = Position> + '_ {
        let mut coordinates = Vec::new();
        let Position(x, y) = self.position;
        // previous row
        for i in 0..self.length + 2 {
            coordinates.push(Position(x - 1 + i as i32, y - 1));
        }
        // current row
        coordinates.push(Position(x - 1, y));
        coordinates.push(Position(x + self.length as i32, y));
        // next row
        for i in 0..self.length + 2 {
            coordinates.push(Position(x - 1 + i as i32, y + 1));
        }
        coordinates.into_iter().filter(|p| p.0 >= 0 && p.1 >= 0)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Schematic {
    symbols: HashMap<Position, char>,
    numbers: HashMap<Position, Number>,
    all_number_positions: HashMap<Position, Number>,
}

#[derive(Debug)]
struct SchematicParseError;

impl FromStr for Schematic {
    type Err = SchematicParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut symbols = HashMap::new();
        let mut numbers = HashMap::new();
        let mut all_number_positions = HashMap::new();
        let mut digit_accumulator = [' '; 10];
        let mut digit_accumulator_index: usize = 0;
        let mut parsing_digit = false;
        let mut current_position = Position(0, 0);
        for (y, line) in s.lines().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                current_position = Position(x as i32, y as i32);
                match symbol {
                    '0'..='9' => {
                        if !parsing_digit {
                            digit_accumulator = [' '; 10];
                            digit_accumulator_index = 0;
                            parsing_digit = true;
                        }
                        digit_accumulator[digit_accumulator_index] = symbol;
                        digit_accumulator_index += 1;
                    }
                    _ => {
                        if symbol != '.' {
                            symbols.insert(current_position, symbol);
                        }
                        if parsing_digit {
                            let value = digit_accumulator
                                .iter()
                                .take_while(|&&c| c != ' ')
                                .collect::<String>()
                                .parse::<u32>()
                                .unwrap();
                            let number_position = Position(
                                current_position.0 as i32 - digit_accumulator_index as i32,
                                current_position.1 as i32,
                            );
                            numbers.insert(
                                number_position,
                                Number {
                                    position: number_position,
                                    value,
                                    length: digit_accumulator_index as u32,
                                },
                            );
                            // add the number to all of its positions
                            for i in 0..digit_accumulator_index {
                                let position = Position(
                                    current_position.0 as i32 - 1 - i as i32,
                                    current_position.1 as i32,
                                );
                                all_number_positions.insert(
                                    position,
                                    Number {
                                        position: number_position,
                                        value,
                                        length: digit_accumulator_index as u32,
                                    },
                                );
                            }
                            parsing_digit = false;
                        }
                    }
                }
            }
            if parsing_digit {
                let value = digit_accumulator
                    .iter()
                    .take_while(|&&c| c != ' ')
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                let number_position = Position(
                    current_position.0 as i32 - digit_accumulator_index as i32 + 1,
                    current_position.1 as i32,
                );
                numbers.insert(
                    number_position,
                    Number {
                        position: number_position,
                        value,
                        length: digit_accumulator_index as u32,
                    },
                );
                // add the number to all of its positions
                for i in 0..digit_accumulator_index {
                    let position = Position(
                        current_position.0 as i32 - i as i32,
                        current_position.1 as i32,
                    );
                    all_number_positions.insert(
                        position,
                        Number {
                            position: number_position,
                            value,
                            length: digit_accumulator_index as u32,
                        },
                    );
                }
                parsing_digit = false;
            }
        }
        Ok(Schematic {
            symbols,
            numbers,
            all_number_positions,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = input.parse::<Schematic>().unwrap();
    Some(
        schematic
            .numbers
            .iter()
            .filter(|(_, number)| {
                number
                    .search_coordinates()
                    .any(|position| schematic.symbols.contains_key(&position))
            })
            .map(|(_, number)| number.value)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic = input.parse::<Schematic>().unwrap();
    Some(
        schematic
            .symbols
            .iter()
            .filter(|(_, symbol)| **symbol == '*')
            .filter_map(|(position, _)| {
                // find all numbers that overlap with this position
                let mut overlapping_numbers = HashSet::new();
                for p in position.search_coordinates() {
                    if let Some(number) = schematic.all_number_positions.get(&p) {
                        if !overlapping_numbers.contains(number) {
                            overlapping_numbers.insert(number);
                        }
                    }
                }
                if overlapping_numbers.len() == 2 {
                    Some(
                        overlapping_numbers
                            .iter()
                            .map(|number| number.value)
                            .fold(1, |acc, value| acc * value),
                    )
                } else {
                    None
                }
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_coordinates_position() {
        let position = Position(1, 1);

        let expected_coordinates: Vec<Position> = [
            Position(0, 0),
            Position(1, 0),
            Position(2, 0),
            Position(0, 1),
            Position(2, 1),
            Position(0, 2),
            Position(1, 2),
            Position(2, 2),
        ]
        .iter()
        .cloned()
        .collect();

        let coordinates: Vec<Position> = position.search_coordinates().collect();

        assert_eq!(coordinates, expected_coordinates);
    }

    #[test]
    fn test_search_coordinates_number() {
        let number = Number {
            position: Position(1, 1),
            value: 1,
            length: 1,
        };

        let expected_coordinates: Vec<Position> = [
            Position(0, 0),
            Position(1, 0),
            Position(2, 0),
            Position(0, 1),
            Position(2, 1),
            Position(0, 2),
            Position(1, 2),
            Position(2, 2),
        ]
        .iter()
        .cloned()
        .collect();

        let coordinates: Vec<Position> = number.search_coordinates().collect();

        assert_eq!(coordinates, expected_coordinates);
    }

    #[test]
    fn test_search_coordinates_longer_number() {
        let number = Number {
            position: Position(1, 1),
            value: 1,
            length: 2,
        };

        let expected_coordinates: Vec<Position> = [
            Position(0, 0),
            Position(1, 0),
            Position(2, 0),
            Position(3, 0),
            Position(0, 1),
            Position(3, 1),
            Position(0, 2),
            Position(1, 2),
            Position(2, 2),
            Position(3, 2),
        ]
        .iter()
        .cloned()
        .collect();

        let coordinates: Vec<Position> = number.search_coordinates().collect();

        assert_eq!(coordinates, expected_coordinates);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4432));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(453825));
    }
}
