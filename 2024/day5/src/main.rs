use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> Option<(HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>)> {
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();

    let mut lines = input.lines();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (pred, succ) = line.split_once('|')?;
        let pred = pred.parse().ok()?;
        let succ = succ.parse().ok()?;

        rules.entry(pred).or_default().insert(succ);
    }

    let updates = lines
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().ok())
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>()?;

    Some((rules, updates))
}

fn is_valid_order(rules: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> bool {
    for (i, page) in update.iter().enumerate() {
        for rule in update.iter().skip(i + 1).filter_map(|p2| rules.get(p2)) {
            if rule.contains(page) {
                return false;
            }
        }
    }

    true
}

fn part1(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input)?;

    let total = updates
        .iter()
        .filter_map(|update| match is_valid_order(&rules, update) {
            true => update.get(update.len() / 2).copied(),
            _ => None,
        })
        .sum();

    Some(total)
}

fn handmade_sort(rules: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> Option<Vec<u32>> {
    let mut sorted = update.to_vec();

    for i in 0..sorted.len() {
        let mut page1 = *sorted.get(i)?;

        for j in 0..sorted.len() {
            let page2 = *sorted.get(j)?;

            let rule = match j.cmp(&i) {
                Ordering::Less => rules.get(&page1),
                Ordering::Greater => rules.get(&page2),
                _ => None,
            };

            if let Some(rule) = rule {
                if rule.contains(&page2) {
                    sorted.swap(i, j);
                    page1 = page2;
                }
            }
        }
    }

    Some(sorted)
}

#[allow(unused)]
/// Kahn's algorithm for DAGs (needs to count only the stuff in `update`, not the whole `graph`)
fn topological_sort(graph: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> Option<Vec<u32>> {
    let mut successors_map = HashMap::new();

    let mut edges = update.iter().map(|&p| (p, 0)).collect::<HashMap<_, _>>();

    for (&node, dependencies) in graph {
        for &dep in dependencies {
            if update.contains(&node) && update.contains(&dep) {
                successors_map
                    .entry(node)
                    .or_insert_with(Vec::new)
                    .push(dep);
                *edges.entry(dep).or_insert(0) += 1;
            }
        }
    }

    let mut queue = edges
        .iter()
        .filter(|(_, &count)| count == 0)
        .map(|(&node, _)| node)
        .collect::<VecDeque<_>>();

    let mut sorted = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted.push(node);

        if let Some(successors) = successors_map.get(&node) {
            for &succ in successors {
                let edge = edges.get_mut(&succ)?;
                *edge -= 1;

                if *edge == 0 {
                    queue.push_back(succ);
                }
            }
        }
    }

    Some(sorted)
}

fn part2(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input)?;

    let total = updates
        .iter()
        .filter_map(|update| {
            match is_valid_order(&rules, update) {
                false => {
                    // a normal sort won't work because `rules` are not transitive
                    // i initially took a "direct" approach with `handmade_sort`
                    // but a "scientific" one would be to see the rules as a DAG and use `topological_sort`
                    
                    //let sorted = topological_sort(&rules, update);
                    let sorted = handmade_sort(&rules, update);

                    sorted.and_then(|sorted| sorted.get(sorted.len() / 2).copied())
                }
                _ => None,
            }
        })
        .sum();

    Some(total)
}

fn main() {
    let input0 = include_str!("input0.txt");
    let input1 = include_str!("input1.txt");

    println!("{:?}", part1(input0));
    println!("{:?}", part1(input1));

    println!("{:?}", part2(input0));
    println!("{:?}", part2(input1));
}
