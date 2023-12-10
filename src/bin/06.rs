use std::iter::zip;
advent_of_code::solution!(6);

// Distance traveled
// d = v * t_release
// v = 1 * t_hold
// t_release = t - t_hold
// d = t_hold * (t - t_hold)
// d = t * t_hold - t_hold^2
// t_hold^2 - t * t_hold - d = 0
// t_hold = (t +- sqrt(t^2 - 4 * d)) / 2
pub fn solve_for_time_to_hold(total_time: i64, distance_to_beat: i64) -> (f64, f64) {
    let sqrt = ((total_time * total_time - 4 * distance_to_beat) as f64).sqrt();
    let t1 = (total_time as f64 - sqrt) / 2.0;
    let t2 = (total_time as f64 + sqrt) / 2.0;
    (t1, t2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .filter(|val| !val.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .filter(|val| !val.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut ways_to_win_product = 1;
    for (time, distance) in zip(times, distances) {
        let (t1, t2) = solve_for_time_to_hold(time, distance);
        // Round up t1 and down t2 to get the number of ways to win.
        // But do so with a small epsilon to avoid rounding up to the same number if answer was a round number
        let t1 = (t1 + 0.00001).ceil() as i64;
        let t2 = (t2 - 0.00001).floor() as i64;
        let ways_to_win = t2 - t1 + 1;
        ways_to_win_product *= ways_to_win;
    }
    Some(ways_to_win_product as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|val| !val.is_whitespace())
        .fold("".to_string(), |mut acc, val| {
            acc.push(val);
            acc
        })
        .parse::<i64>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|val| !val.is_whitespace())
        .fold("".to_string(), |mut acc, val| {
            acc.push(val);
            acc
        })
        .parse::<i64>()
        .unwrap();

    let (t1, t2) = solve_for_time_to_hold(time, distance);
    // Round up t1 and down t2 to get the number of ways to win.
    // But do so with a small epsilon to avoid rounding up to the same number if answer was a round number
    let t1 = (t1 + 0.00001).ceil() as i64;
    let t2 = (t2 - 0.00001).floor() as i64;
    let ways_to_win = t2 - t1 + 1;
    Some(ways_to_win as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
