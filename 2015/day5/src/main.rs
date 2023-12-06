fn main() {
    let input = include_str!("input1.txt");
    println!("{:?}", part1(input));
    //println!("{:?}", part2(input));
}

fn part1(input: &str) -> u32 {
    let bad_ss = ["ab", "cd", "pq", "xy"];
    let vowels = "aeiouAEIOU";
    let mut good_lines = 0;

    let has_bad_ss = |line: &str| {
        bad_ss.iter()
            .any(|s| line.contains(s))
    };
    let has_duplicate = |line: &str| {
        line.chars()
            .zip(line.chars().skip(1))
            .any(|(c1, c2)| c1 == c2)
    };
    let has_vowels = |line: &str| {
        line.chars().filter(|&c| vowels.contains(c)).count() >= 3
    };

    for line in input.lines() {
        if !has_bad_ss(line) && has_duplicate(line) && has_vowels(line) {
            good_lines += 1;
        }
    }
    good_lines
}

fn part2(input: &str) -> Option<()> {
    todo!()
}
