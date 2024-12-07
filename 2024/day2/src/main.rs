fn main() {
    let input = include_str!("input0.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));

    let input = include_str!("input1.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn parse_input(input: &str) -> Option<Vec<Vec<u32>>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().ok())
                .collect::<Option<_>>()
        })
        .collect::<Option<_>>()
}

fn check(row: &[u32]) -> bool {
    let mut dir = None;

    row.iter()
        .zip(row.iter().skip(1))
        .map(|(&curr, &next)| {
            let diff: i32 = curr as i32 - next as i32;
            let dist = diff.unsigned_abs();
            let sign = diff.signum();
            let safe = dir.unwrap_or(sign) == sign && (1..=3).contains(&dist);
            dir = Some(sign);

            safe
        })
        .all(|b| b)
}

fn part1(input: &str) -> Option<u32> {
    let rows = parse_input(input)?;

    Some(rows.iter().fold(0, |acc, row| acc + check(row) as u32))
}

/// TODO: without bruteforce
fn part2(input: &str) -> Option<u32> {
    let rows = parse_input(input)?;

    let total = rows.iter().fold(0, |acc, row| {
        let mut safe = check(row);

        let mut it = 0..row.len();
        while !safe {
            if let Some(i) = it.next() {
                let split = [&row[..i], &row[i + 1..]].concat();
                safe = check(&split);
            } else {
                break;
            }
        }

        acc + safe as u32
    });

    Some(total)
}