#[derive(Clone, Copy, Debug)]
enum Op {
    On,
    Off,
    Toggle,
}

type Range = (usize, usize);
type Instruction = (Op, Range, Range);

fn parse_range(line: &str) -> Option<Range> {
    let (w, h) = line.trim().split_once(',')?;

    Some((w.parse().ok()?, h.parse().ok()?))
}

fn parse_op(line: &str) -> Option<(&str, Op)> {
    let table = [
        ("turn on", Op::On),
        ("turn off", Op::Off),
        ("toggle", Op::Toggle),
    ];

    table.iter().find_map(|t| {
        let leftover = line.trim().strip_prefix(t.0)?;
        Some((leftover, t.1))
    })
}

fn parse_input(input: &str) -> Option<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("through")?;
            let (left, op) = parse_op(left)?;
            let beg = parse_range(left)?;
            let end = parse_range(right)?;

            Some((op, beg, end))
        })
        .collect()
}

fn part1(input: &str) -> Option<u32> {
    let mut lights = vec![[false; 1000]; 1000];
    let instructions = parse_input(input)?;

    for i in instructions {
        let (op, (bx, by), (ex, ey)) = i;
        for x in bx..=ex {
            let row = &mut lights.get_mut(x)?[by..=ey];
            match op {
                Op::On => row.fill(true),
                Op::Off => row.fill(false),
                Op::Toggle => row.iter_mut().for_each(|b| *b = !*b),
            }
        }
    }

    Some(
        lights
            .iter()
            .map(|rows| rows.iter().map(|b| *b as u32).sum::<u32>())
            .sum(),
    )
}

fn part2(input: &str) -> Option<u32> {
    let mut lights = vec![[0u32; 1000]; 1000];
    let instructions = parse_input(input)?;

    for i in instructions {
        let (op, (bx, by), (ex, ey)) = i;
        for x in bx..=ex {
            let row = &mut lights.get_mut(x)?[by..=ey];
            let op_fn: fn(&mut u32) = match op {
                Op::On => |b| *b += 1,
                Op::Off => |b| *b = b.saturating_sub(1),
                Op::Toggle => |b| *b += 2,
            };
            row.iter_mut().for_each(op_fn);
        }
    }

    Some(lights.iter().map(|rows| rows.iter().sum::<u32>()).sum())
}

fn main() {
    let input0 = include_str!("input0.txt");
    let input1 = include_str!("input1.txt");

    println!("{:?}", part1(input0));
    println!("{:?}", part1(input1));

    println!("{:?}", part2(input0));
    println!("{:?}", part2(input1));
}
