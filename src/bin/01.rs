advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut digits = line.chars().filter(|char| char.is_ascii_digit());
                let first = digits.next().unwrap();
                let mut value = first.to_string();
                if let Some(last) = digits.last() {
                    value.push(last);
                } else {
                    value.push(first);
                }
                value
            })
            .map(|strings| {
                return strings.parse::<u32>().unwrap();
            })
            .sum(),
    )
}

const ONE: [char; 3] = ['o', 'n', 'e'];
const TWO: [char; 3] = ['t', 'w', 'o'];
const THREE: [char; 5] = ['t', 'h', 'r', 'e', 'e'];
const FOUR: [char; 4] = ['f', 'o', 'u', 'r'];
const FIVE: [char; 4] = ['f', 'i', 'v', 'e'];
const SIX: [char; 3] = ['s', 'i', 'x'];
const SEVEN: [char; 5] = ['s', 'e', 'v', 'e', 'n'];
const EIGHT: [char; 5] = ['e', 'i', 'g', 'h', 't'];
const NINE: [char; 4] = ['n', 'i', 'n', 'e'];
const DIGIT_LENGTHS: [usize; 9] = [
    ONE.len(),
    TWO.len(),
    THREE.len(),
    FOUR.len(),
    FIVE.len(),
    SIX.len(),
    SEVEN.len(),
    EIGHT.len(),
    NINE.len(),
];

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut first: char = ' ';
                let mut last: char = ' ';
                let mut digit_indices: [usize; 9] = [0; 9];
                for character in line.chars() {
                    if character.is_ascii_digit() {
                        if first == ' ' {
                            first = character;
                        }
                        last = character;
                        digit_indices = [0; 9];
                    } else {
                        for (index, digit_len) in DIGIT_LENGTHS.iter().enumerate() {
                            if character
                                == match index {
                                    0 => ONE[digit_indices[index]],
                                    1 => TWO[digit_indices[index]],
                                    2 => THREE[digit_indices[index]],
                                    3 => FOUR[digit_indices[index]],
                                    4 => FIVE[digit_indices[index]],
                                    5 => SIX[digit_indices[index]],
                                    6 => SEVEN[digit_indices[index]],
                                    7 => EIGHT[digit_indices[index]],
                                    8 => NINE[digit_indices[index]],
                                    _ => panic!("Invalid index"),
                                }
                            {
                                digit_indices[index] += 1;
                                if digit_indices[index] == *digit_len {
                                    digit_indices[index] = 0;
                                    let digit = (index + 1).to_string().chars().next().unwrap();

                                    if first == ' ' {
                                        first = digit;
                                    }
                                    last = digit;
                                }
                            } else {
                                digit_indices[index] = 0;
                                if character
                                    == match index {
                                        0 => ONE[digit_indices[index]],
                                        1 => TWO[digit_indices[index]],
                                        2 => THREE[digit_indices[index]],
                                        3 => FOUR[digit_indices[index]],
                                        4 => FIVE[digit_indices[index]],
                                        5 => SIX[digit_indices[index]],
                                        6 => SEVEN[digit_indices[index]],
                                        7 => EIGHT[digit_indices[index]],
                                        8 => NINE[digit_indices[index]],
                                        _ => panic!("Invalid index"),
                                    }
                                {
                                    digit_indices[index] += 1;
                                }
                            }
                        }
                    }
                }
                let mut value = first.to_string();
                value.push(last);
                value
            })
            .map(|strings| {
                return strings.parse::<u32>().unwrap();
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(99));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(93));
    }
}
