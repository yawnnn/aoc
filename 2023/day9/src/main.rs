fn derive(v: &[i64]) -> Vec<i64> {
    v.iter()
        .zip(v.iter().skip(1))
        .map(|(v1, v2)| v2 - v1)
        .collect::<Vec<_>>()
}

fn predict_next_part1(v: Vec<i64>) -> Option<i64> {
    let diffs = derive(&v);

    if diffs.iter().all(|&i| i == 0) {
        v.last().copied()
    } else {
        Some(v.last()? + predict_next_part1(diffs)?)
    }
}

fn predict_next_part2(v: Vec<i64>) -> Option<i64> {
    let diffs = derive(&v);

    if diffs.iter().all(|&i| i == 0) {
        v.first().copied()
    } else {
        Some(v.first()? - predict_next_part2(diffs)?)
    }
}

fn parse_line(s: &str) -> Vec<i64> {
    s.split(' ')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<_>>()
}

fn main() {
    let input = include_str!("input1.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> Option<i64> {
    let mut count = 0;

    for line in input.lines() {
        count += predict_next_part1(parse_line(line))?;
    }

    Some(count)
}

fn part2(input: &str) -> Option<i64> {
    let mut count = 0;

    for line in input.lines() {
        count += predict_next_part2(parse_line(line))?;
    }

    Some(count)
}
