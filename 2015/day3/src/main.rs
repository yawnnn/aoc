use std::collections::HashSet;

fn main() {
    let input = include_str!("input1.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(s: &str) -> usize {
    let mut pos = (0, 0);
    let mut houses: HashSet::<(i32, i32)> = HashSet::from([pos]);

    for c in s.chars() {
        match c {
            '>' => pos.0 += 1,
            '^' => pos.1 += 1,
            '<' => pos.0 -= 1,
            'v' => pos.1 -= 1,
            _ => (),
        }

        houses.insert(pos);
    }

    houses.len()
}

fn part2(s: &str) -> usize {
    let mut santa_pos = (0, 0);
    let mut robo_pos = (0, 0);
    let mut current_pos = &mut santa_pos;
    let mut houses: HashSet::<(i32, i32)> = HashSet::from([*current_pos]);

    for (index, c) in s.chars().enumerate() {
        current_pos = match index % 2 {
            0 => &mut santa_pos,
            _ => &mut robo_pos,
        };

        match c {
            '>' => current_pos.0 += 1,
            '^' => current_pos.1 += 1,
            '<' => current_pos.0 -= 1,
            'v' => current_pos.1 -= 1,
            _ => (),
        }

        houses.insert(*current_pos);
    }

    houses.len()
}
