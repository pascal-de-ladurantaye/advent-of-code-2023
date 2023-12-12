use std::cmp::Ordering;
use std::iter::zip;

use itertools::Itertools;

advent_of_code::solution!(7);

fn card_to_i8(card: char, j_value: i8) -> i8 {
    match card {
        'A' => 14,
        '2'..='9' => card.to_digit(10).unwrap() as i8,
        'T' => 10,
        'J' => j_value,
        'Q' => 12,
        'K' => 13,
        _ => panic!("Invalid card: {}", card),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPairs = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<i8>,
    hand_type: HandType,
    bid: i32,
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self.hand_type) as u8 == other.hand_type as u8 {
            for (mine, theirs) in zip(&self.cards, &other.cards) {
                if mine != theirs {
                    return mine.cmp(theirs);
                }
            }
        }
        self.hand_type.cmp(&other.hand_type)
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Debug, Eq, PartialEq)]
struct HandParseError;

fn hand_from_str(s: &str, with_jokers: bool) -> Result<Hand, HandParseError> {
    if let Some((cards, bid)) = s.split_once(' ') {
        let cards = cards
            .chars()
            .map(|c| card_to_i8(c, if with_jokers { 1 } else { 11 }))
            .collect_vec();
        let bid = bid.parse::<i32>().unwrap();
        // count cards by value
        let mut counts = cards.iter().counts();
        let mut joker_count = 0;
        if with_jokers {
            joker_count = counts.remove(&1).unwrap_or(0);
        }
        let mut hand_type = HandType::HighCard;
        match counts.len() {
            0 | 1 => hand_type = HandType::FiveOfAKind,
            2 => {
                let (&size1, &size2) = counts.values().collect_tuple().unwrap();
                if size1 == 4 || size2 == 4 || size1 + joker_count == 4 || size2 + joker_count == 4
                {
                    hand_type = HandType::FourOfAKind;
                } else {
                    hand_type = HandType::FullHouse;
                }
            }
            3 => {
                let (&size1, &size2, &size3) = counts.values().collect_tuple().unwrap();
                if size1 == 3
                    || size2 == 3
                    || size3 == 3
                    || size1 + joker_count == 3
                    || size2 + joker_count == 3
                    || size3 + joker_count == 3
                {
                    hand_type = HandType::ThreeOfAKind;
                } else {
                    hand_type = HandType::TwoPairs;
                }
            }
            4 => hand_type = HandType::OnePair,
            _ => {}
        }
        Ok(Hand {
            cards,
            hand_type,
            bid,
        })
    } else {
        Err(HandParseError)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let hands = input
        .lines()
        .map(|line| hand_from_str(line, false).unwrap())
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid as u32);

    Some(hands.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands = input
        .lines()
        .map(|line| hand_from_str(line, true).unwrap())
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid as u32);

    Some(hands.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_hand_ordering() {
        let hand1 = Hand {
            cards: vec![14, 14, 14, 14, 10],
            hand_type: HandType::FourOfAKind,
            bid: 1,
        };
        let hand2 = Hand {
            cards: vec![14, 14, 14, 14, 9],
            hand_type: HandType::FourOfAKind,
            bid: 1,
        };
        assert!(hand1 > hand2);

        let hand1 = Hand {
            cards: vec![2, 2, 2, 2, 2],
            hand_type: HandType::FiveOfAKind,
            bid: 1,
        };
        let hand2 = Hand {
            cards: vec![14, 14, 14, 14, 13],
            hand_type: HandType::FourOfAKind,
            bid: 1,
        };
        assert!(hand1 > hand2);
    }
    #[test]
    fn test_hand_parsing() {
        let input = "AAAAJ 1234";
        assert_eq!(
            hand_from_str(input, false),
            Ok(Hand {
                cards: vec![14, 14, 14, 14, 11],
                hand_type: HandType::FourOfAKind,
                bid: 1234
            })
        );

        let input = "AAAAJ 1234";
        assert_eq!(
            hand_from_str(input, true),
            Ok(Hand {
                cards: vec![14, 14, 14, 14, 1],
                hand_type: HandType::FiveOfAKind,
                bid: 1234
            })
        );
    }
}
