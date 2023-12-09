use std::collections::HashSet;
use std::str::FromStr;
advent_of_code::solution!(4);


#[derive(Debug, PartialEq, Eq, Clone)]
struct Card {
    index: usize,
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
}

impl Card {
    fn score(&self) -> usize {
        let winning_number_count = self.winning_numbers.intersection(&self.numbers).count();
        match winning_number_count {
            0 => 0,
            _ => usize::pow(2, (winning_number_count - 1) as u32),
        }
    }

    fn winning_count(&self) -> usize {
        self.winning_numbers.intersection(&self.numbers).count()
    }
}

#[derive(Debug)]
struct CardParseError;

impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut winning_numbers_hash = HashSet::with_capacity(10);
        let mut numbers_hash = HashSet::with_capacity(25);
        let mut card_index = 0;
        if let Some((card_id_str, numbers)) = s.split_once(":") {
            card_index = card_id_str.split_whitespace().filter(|s| !s.is_empty()).last().unwrap().parse::<usize>().unwrap() - 1;
            if let Some((winning_numbers, numbers)) = numbers.split_once("|") {
                for number in winning_numbers.split_whitespace().filter(|s| !s.is_empty()) {
                    winning_numbers_hash.insert(number.parse::<usize>().unwrap());
                }
                for number in numbers.split_whitespace().filter(|s| !s.is_empty()) {
                    numbers_hash.insert(number.parse::<usize>().unwrap());
                }
            }
        }
        Ok(Card {
            index: card_index,
            winning_numbers: winning_numbers_hash,
            numbers: numbers_hash,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| line.parse::<Card>().unwrap())
            .map(|card| card.score())
            .sum::<usize>() as u32
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .collect::<Vec<Card>>();
    let card_count = cards.len();
    let mut card_counts_per_id = vec![1u32; card_count];
    for card in cards.iter() {
        if card.winning_count() == 0 {
            continue;
        }
        let card_amount = card_counts_per_id[card.index];
        for additional_card_id in card_counts_per_id[card.index + 1..=card.index + card.winning_count()].iter_mut() {
            *additional_card_id += card_amount;
        }
    }
    Some(card_counts_per_id.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_parse() {
        let card = "Game 1: 1 2 3 4 5 6 7 8 9 10 | 11 12 13 14 15 16 17 18 19 20";
        let card = card.parse::<Card>().unwrap();
        assert_eq!(card.index, 0);
        assert_eq!(card.winning_numbers.len(), 10);
        assert_eq!(card.numbers.len(), 10);
        assert_eq!(card.winning_numbers, HashSet::from_iter(1..=10));
        assert_eq!(card.numbers, HashSet::from_iter(11..=20));
    }

    #[test]
    fn test_card_score() {
        let card = "Game 1: 1 2 3 4 5 6 7 8 9 10 | 11 12 13 14 15 16 17 18 19 20";
        let card = card.parse::<Card>().unwrap();
        assert_eq!(card.score(), 0);
        assert_eq!(card.winning_count(), 0);

        let card = "Game 1: 1 2 3 4 5 6 7 8 9 10 | 1 12 13 14 15 16 17 18 19 20";
        let card = card.parse::<Card>().unwrap();
        assert_eq!(card.score(), 1);
        assert_eq!(card.winning_count(), 1);

        let card = "Game 1: 1 2 3 4 5 6 7 8 9 10 | 1 2 13 14 15 16 17 18 19 20";
        let card = card.parse::<Card>().unwrap();
        assert_eq!(card.score(), 2);
        assert_eq!(card.winning_count(), 2);

        let card = "Game 1: 1 2 3 4 5 6 7 8 9 10 | 1 2 3 14 15 16 17 18 19 20";
        let card = card.parse::<Card>().unwrap();
        assert_eq!(card.score(), 4);
        assert_eq!(card.winning_count(), 3);

        let card = "Game 1: 1 2 3 4 5 6 7 8 9 10 | 1 2 3 4 15 16 17 18 19 20";
        let card = card.parse::<Card>().unwrap();
        assert_eq!(card.score(), 8);
        assert_eq!(card.winning_count(), 4);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
