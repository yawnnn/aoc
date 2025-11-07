use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> Option<u32> {
    let mut total = 0;

    for mut line in input.lines() {
        line = &line["Game ".len()..];
        let semicolon = line.find(':')?;
        let game_id = line[..semicolon].parse::<u32>().ok()?;
        line = &line[semicolon + 1..];

        let mut possible = true;
        for set in line.split(';') {
            for draw in set.split(',').map(|s| s.trim()) {
                let after_num = draw.find(' ')?;
                let num = draw[..after_num].parse::<u32>().ok()?;

                if num
                    > match &draw[after_num + 1..] {
                        "red" => 12,
                        "green" => 13,
                        "blue" => 14,
                        _ => num,
                    }
                {
                    possible = false;
                    break;
                }
            }
        }

        if possible {
            total += game_id;
        }
    }
    Some(total)
}

fn part2(input: &str) -> Option<u32> {
    let mut total = 0;

    for mut line in input.lines() {
        line = &line[line.find(':')? + 1..];

        let mut cube_nums = HashMap::new();
        for set in line.split(';') {
            for draw in set.split(',').map(|s| s.trim()) {
                let after_num = draw.find(' ')?;
                let num = draw[..after_num].parse::<u32>().ok()?;

                cube_nums
                    .entry(&draw[after_num + 1..])
                    .and_modify(|v| {
                        if *v < num {
                            *v = num
                        }
                    })
                    .or_insert(num);
            }
        }

        total += cube_nums.values().product::<u32>();
    }
    Some(total)
}
