fn main() {
    let input = include_str!("input1.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> Option<u32> {
    let mut total = 0;

    for line in input.lines() {
        let first = line.chars().filter_map(|c| c.to_digit(10)).next()?;
        let last = line.chars().filter_map(|c| c.to_digit(10)).next_back()?;
        total += first * 10 + last;
    }
    Some(total)
}

fn part2(input: &str) -> Option<u32> {
    let spelled_digits_mapping = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut total = 0;

    for line in input.lines() {
        let mut first = line
            .char_indices()
            .filter_map(|(p, c)| c.to_digit(10).map(|n| (p, n)))
            .next()?;

        let mut last = line
            .char_indices()
            .filter_map(|(p, c)| c.to_digit(10).map(|n| (p, n)))
            .next_back()?;

        for (letters, digit) in spelled_digits_mapping {
            if let Some(p) = line.find(letters) {
                if first.0 > p {
                    first = (p, digit);
                }
            }

            if let Some(p) = line.rfind(letters) {
                if last.0 < p {
                    last = (p, digit);
                }
            }
        }

        total += first.1 * 10 + last.1;
    }
    Some(total)
}
