fn main() {
    let input = include_str!("input1.txt");
    println!("{}", part1(input));
    println!("{}", part2(input).unwrap_or(0));
}

fn part1(s: &str) -> i32 {
    s.chars().fold(0, |acc, c: char| {
        acc + match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    })
}

fn part2(s: &str) -> Option<usize> {
    let mut sum = 0;

    for (index, c) in s.chars().enumerate() {
        match c {
            '(' => sum += 1,
            ')' => {
                sum -= 1;
                if sum == -1 {
                    return Some(index + 1);
                }
            }
            _ => (),
        }
    }
    None
}
