use std::{collections::HashMap, iter};

fn main() {
    let input = include_str!("input0.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));

    let input = include_str!("input1.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn parse_input(input: &str) -> Option<(Vec<u32>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            line.split_once("   ").and_then(|(l, r)| {
                l.parse::<u32>()
                    .ok()
                    .and_then(|l| r.parse::<u32>().ok().map(|r| (l, r)))
            })
        })
        .collect()
}

fn part1(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_input(input)?;
    left.sort();
    right.sort();

    let total = left
        .into_iter()
        .zip(right)
        .fold(0, |acc, (left, right)| acc + left.abs_diff(right));

    Some(total)
}

fn part2(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_input(input)?;
    left.sort();
    right.sort();

    let mut right_reps = HashMap::new();

    let mut count = 0;
    right.iter().zip(right.iter().skip(1).chain(iter::once(&0))).for_each(|(&curr, &next)| {
        if curr == next {
            count += 1;
        }
        else {
            right_reps.insert(curr, count + 1);
            count = 0;
        }
    });

    let total = left.iter().fold(0, |acc, left| {
        right_reps.get(left).map_or(acc, |count| acc + left * count)
    });

    Some(total)
}
