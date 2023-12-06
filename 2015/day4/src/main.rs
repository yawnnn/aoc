fn main() {
    let input = "yzbqklnj";
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> Option<u64> {
    for n in 0..=u64::MAX {
        let data = format!("{}{}", input, n);
        let digest = md5::compute(data);

        if format!("{:x}", digest).starts_with("00000") {
            return Some(n);
        }
    }
    None
}

fn part2(input: &str) -> Option<u64> {
    for n in 0..=u64::MAX {
        let data = format!("{}{}", input, n);
        let digest = md5::compute(data);

        if format!("{:x}", digest).starts_with("000000") {
            return Some(n);
        }
    }
    None
}