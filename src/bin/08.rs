use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use primes::{PrimeSet, Sieve};

advent_of_code::solution!(8);

fn base26_to_usize(s: &str) -> usize {
    let mut result = 0;
    for c in s.chars() {
        result *= 26;
        result += (c as u8 - b'A') as usize;
    }
    result
}

fn parse_line(line: &str) -> (usize, (usize, usize)) {
    (
        base26_to_usize(&line[0..3]),
        (
            base26_to_usize(&line[7..10]),
            base26_to_usize(&line[12..15]),
        ),
    )
}

fn find_cycle_with_steps_for_end_nodes(
    start_node: usize,
    nodes: &HashMap<usize, Node>,
    directions: impl Iterator<Item = char>,
    direction_count: usize,
) -> Vec<u64> {
    let mut current_node_address = start_node;
    let mut steps = 0;
    let mut visited_nodes = HashSet::new();
    let mut end_steps = Vec::new();
    for direction in directions {
        let node_position_in_direction_cycle = steps % direction_count;
        steps += 1;
        visited_nodes.insert((current_node_address, node_position_in_direction_cycle));
        let current_node = nodes.get(&current_node_address).unwrap();
        current_node_address = match direction {
            'L' => current_node.0,
            'R' => current_node.1,
            _ => panic!("Invalid direction"),
        };
        if current_node_address % 26 == 25 {
            // End node
            end_steps.push(steps as u64);
        }
        if visited_nodes.contains(&(current_node_address, node_position_in_direction_cycle + 1)) {
            // Cycle
            break;
        }
    }
    end_steps
}

fn least_common_multiple(mut numbers: Vec<u64>) -> u64 {
    let mut prime_set = Sieve::new();
    let mut primes = prime_set.iter();
    let mut factors = Vec::new();
    loop {
        let next_prime = primes.next().unwrap();
        loop {
            let mut found = false;
            for number in &mut numbers {
                if *number % next_prime == 0 {
                    *number /= next_prime;
                    found = true;
                }
            }
            if found {
                factors.push(next_prime);
            } else {
                break;
            }
        }
        if numbers.iter().all(|n| *n == 1) {
            break;
        }
    }
    factors.iter().product()
}

pub fn partial_cartesian<T: Clone>(a: Vec<Vec<T>>, b: &[T]) -> Vec<Vec<T>> {
    a.into_iter()
        .flat_map(|xs| {
            b.iter()
                .cloned()
                .map(|y| {
                    let mut vec = xs.clone();
                    vec.push(y);
                    vec
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn cartesian_product<T: Clone>(lists: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    match lists.split_first() {
        Some((first, rest)) => {
            let init: Vec<Vec<T>> = first.iter().cloned().map(|n| vec![n]).collect();

            rest.iter()
                .cloned()
                .fold(init, |vec, list| partial_cartesian(vec, &list))
        }
        None => {
            vec![]
        }
    }
}

#[derive(Debug)]
struct Node(usize, usize);
pub fn part_one(input: &str) -> Option<i32> {
    let mut lines = input.lines();
    let directions = lines.next().unwrap().chars().cycle();
    let mut nodes: HashMap<usize, Node> = HashMap::new();

    for line in lines.filter(|l| !l.is_empty()) {
        let (from, (to1, to2)) = parse_line(line);
        nodes.insert(from, Node(to1, to2));
    }
    let mut current = base26_to_usize("AAA");
    let target = base26_to_usize("ZZZ");
    let mut steps = 0;
    for direction in directions {
        steps += 1;
        let current_node = nodes.get(&current).unwrap();
        current = match direction {
            'L' => current_node.0,
            'R' => current_node.1,
            _ => panic!("Invalid direction"),
        };
        if current == target {
            break;
        }
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let directions = lines.next().unwrap().chars();
    let direction_count = directions.clone().count();
    let directions = directions.cycle();
    let mut nodes: HashMap<usize, Node> = HashMap::new();
    let mut starting_nodes: Vec<usize> = Vec::new();

    for line in lines.filter(|l| !l.is_empty()) {
        let (from, (to1, to2)) = parse_line(line);
        nodes.insert(from, Node(to1, to2));
        if from % 26 == 0 {
            starting_nodes.push(from);
        }
    }
    let potential_step_multiples = starting_nodes
        .iter()
        .map(|n| {
            find_cycle_with_steps_for_end_nodes(*n, &nodes, directions.clone(), direction_count)
        })
        .collect_vec();
    let result = cartesian_product(&potential_step_multiples)
        .into_iter()
        .map(least_common_multiple)
        .min()
        .unwrap();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_base26_to_usize() {
        assert_eq!(base26_to_usize("AAA"), 0);
        assert_eq!(base26_to_usize("ZZZ"), 17575);
    }
}
