use std::collections::HashMap;

struct Node<'a> {
    name: &'a str,
    links: (&'a str, &'a str),
}

impl<'a> TryFrom<&'a str> for Node<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut data = value.split('=');
        let name = data.next().ok_or(())?.trim();
        let mut links = data.next().ok_or(())?.trim().split(',');
        let left = links.next().ok_or(())?.trim();
        let right = links.next().ok_or(())?.trim();
        let links = (&left[1..], &right[..right.len() - 1]);

        Ok(Node { name, links })
    }
}

fn parse_nodes<'a>(lines: impl Iterator<Item = &'a str>) -> Option<HashMap<&'a str, Node<'a>>> {
    lines
        .map(Node::try_from)
        .map(|node| {
            if let Ok(node) = node {
                Some((node.name, node))
            } else {
                None
            }
        })
        .collect::<Option<HashMap<_, _>>>()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn main() {
    let input = include_str!("input1.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> Option<u64> {
    let mut data = input.lines();
    let instructions = data.next()?;
    let nodes = parse_nodes(data.skip(1))?;

    let mut curr = nodes.get("AAA")?;
    let mut count = 0;

    for instruction in instructions.chars().cycle() {
        match instruction {
            'L' => curr = nodes.get(curr.links.0)?,
            'R' => curr = nodes.get(curr.links.1)?,
            _ => return None,
        }

        count += 1;

        if curr.name == "ZZZ" {
            break;
        }
    }

    Some(count)
}

fn part2(input: &str) -> Option<u64> {
    let mut data = input.lines();
    let instructions = data.next()?;
    let nodes = parse_nodes(data.skip(1))?;

    let mut curr = nodes
        .iter()
        .filter_map(|(&k, node)| if k.ends_with('A') { Some((node, 0)) } else { None })
        .collect::<Vec<_>>();
    let mut count = 0;

    for (step, instruction) in instructions.chars().cycle().enumerate() {
        for (node, min_steps) in curr.iter_mut() {
            *node = match instruction {
                'L' => nodes.get(node.links.0)?,
                'R' => nodes.get(node.links.1)?,
                _ => return None,
            };

            if node.name.ends_with('Z') {
                *min_steps = (step + 1) as u64;
                count += 1;
            }
        }

        if count == curr.len() {
            break;
        }
    }

    let lcm = curr.iter().fold(1, |acc, &(_, steps)| {
        lcm(acc, steps)
    });

    Some(lcm)
}
