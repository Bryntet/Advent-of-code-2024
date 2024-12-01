use prse::{parse, Parse};

advent_of_code::solution!(1);

#[derive(Parse)]
#[prse = "{0} {1}"]
struct ListPair(u32, u32);

fn get_sides(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| ListPair::from_str(line).unwrap())
        .map(|ListPair(a, b)| (a, b))
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_side, mut right_side) = get_sides(input);
    left_side.sort();
    right_side.sort();
    let out = left_side
        .into_iter()
        .zip(right_side.into_iter())
        .map(|(left, right)| left.abs_diff(right))
        .sum();
    Some(out)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut left_side, mut right_side) = get_sides(input);
    let out = left_side
        .into_iter()
        .map(|left_num| {
            let count = right_side
                .iter()
                .filter(|right_num| **right_num == left_num)
                .count();
            left_num * count as u32
        })
        .sum();
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
