fn parse_mul(input: &str) -> Option<(usize, (u32, u32))> {
    let start = "mul(";
    let end = ")";
    let mut pos = 0;

    if input.starts_with(start) {
        pos += start.len();

        let pcomma = input[pos..].find(',')?;
        let num1 = &input[pos..pos + pcomma];

        if num1.len() <= 3 {
            let num1 = num1.parse().ok()?;
            pos += pcomma + 1;

            let pend = input[pos..].find(end)?;
            let num2 = &input[pos..pos + pend];

            if num2.len() <= 3 {
                let num2 = num2.parse().ok()?;
                pos += pend + end.len();

                return Some((pos, (num1, num2)));
            }
        }
    }

    None
}

fn parse_do(input: &str) -> Option<usize> {
    let do_ = "do()";

    if input.starts_with(do_) {
        return Some(do_.len());
    }

    None
}

fn parse_dont(input: &str) -> Option<usize> {
    let dont = "don't()";

    if input.starts_with(dont) {
        return Some(dont.len());
    }

    None
}

fn part1(input: &str) -> Option<u32> {
    let mut total = 0;
    let mut cursor = input;

    while !cursor.is_empty() {
        cursor = if let Some((pos, nums)) = parse_mul(cursor) {
            total += nums.0 * nums.1;
            &cursor[pos..]
        } else {
            &cursor[1..]
        };
    }

    Some(total)
}

fn part2(input: &str) -> Option<u32> {
    let mut total = 0;
    let mut cursor = input;
    let mut ok_mul = true;

    while !cursor.is_empty() {
        let pos = if let Some(pos) = parse_dont(cursor) {
            ok_mul = false;
            pos
        } else if let Some(pos) = parse_do(cursor) {
            ok_mul = true;
            pos
        } else if let Some((pos, nums)) = parse_mul(cursor) {
            if ok_mul {
                total += nums.0 * nums.1;
            }
            pos
        } else {
            1
        };

        cursor = &cursor[pos..];
    }

    Some(total)
}

fn main() {
    let input = include_str!("input0.txt");
    println!("{:?}", part1(input));
    let input = include_str!("input0_2.txt");
    println!("{:?}", part2(input));

    let input = include_str!("input1.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}
