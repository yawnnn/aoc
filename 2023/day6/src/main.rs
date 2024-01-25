use std::iter::zip;

fn main() {
    let input = include_str!("input1.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn parse_input_line(line: &str) -> Option<Vec<u32>> {
    Some(
        line.split(':')
            .nth(1)?
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect(),
    )
}

fn parse_input_line_part2(line: &str) -> Option<u64> {
    let data = line.split(':').nth(1)?;
    let number = data
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    number.parse().ok()
}

fn part1(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times = parse_input_line(lines.next()?)?;
    let distances = parse_input_line(lines.next()?)?;

    let mut total = 1;
    for (time, dist) in zip(times, distances) {
        let is_odd = time % 2 != 0;
        let unique_permutations = (time as f32 / 2.).floor() as u32;

        let mut count = 0;
        for velocity in 1..=unique_permutations {
            let dist_traveled = velocity * (time - velocity);

            if dist_traveled > dist {
                if is_odd || velocity != unique_permutations {
                    count += 2;
                } else {
                    count += 1;
                }
            }
        }

        total *= count;
    }

    Some(total)
}

fn part2(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let record_time = parse_input_line_part2(lines.next()?)?;
    let record_distance = parse_input_line_part2(lines.next()?)?;

    let mut total = 1;
    let is_odd = record_time % 2 != 0;
    let unique_permutations = (record_time as f64 / 2.).floor() as u64;

    let mut count = 0;
    for velocity in 1..=unique_permutations {
        let dist_traveled = velocity * (record_time - velocity);

        if dist_traveled > record_distance {
            if is_odd || velocity != unique_permutations {
                count += 2;
            } else {
                count += 1;
            }
        }
    }

    total *= count;

    Some(total)
}
