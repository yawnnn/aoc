use std::ops::Range;

fn main() {
    let input = include_str!("input1.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn parse_seeds(s: &str) -> Option<Vec<u64>> {
    let line = s.lines().next()?;
    let data = line.split(':').nth(1)?;

    Some(
        data.split(' ')
            .filter_map(|line| line.trim().parse().ok())
            .collect(),
    )
}

fn parse_seeds_part2(s: &str) -> Option<Vec<Range<u64>>> {
    let line = s.lines().next()?;
    let data = line.split(':').nth(1)?;

    let mut parsed = data.split(' ').filter_map(|line| line.trim().parse().ok());

    let mut seeds = Vec::new();
    // could use the currently nightly #![feature(iter_array_chunks)]
    while let Some(src) = parsed.next() {
        seeds.push(src..src + parsed.next()?);
    }

    Some(seeds)
}

fn parse_range(s: &str) -> Option<(Range<u64>, Range<u64>)> {
    let mut parsed = s.split(' ').filter_map(|s| s.trim().parse().ok());

    let dst = parsed.next()?;
    let src = parsed.next()?;
    let len = parsed.next()?;

    Some((src..src + len, dst..dst + len))
}

fn parse_mapping(s: &str) -> Vec<(Range<u64>, Range<u64>)> {
    s.lines().skip(1).filter_map(parse_range).collect()
}

fn map_seed(seed: u64, mapping: &Vec<(Range<u64>, Range<u64>)>) -> u64 {
    for (src, dst) in mapping {
        // it seems to me src.position() uses iterators and therefore just loops through every number
        // so i guess calculating it myself is better
        if src.contains(&seed) {
            return dst.start + (seed - src.start);
        }
    }

    seed
}

fn part1(input: &str) -> Option<u64> {
    let no_carriage_ret = input.replace("\r\n", "\n");
    let mut groups = no_carriage_ret.split("\n\n");

    let seeds = parse_seeds(groups.next()?)?;
    let mappings = groups.map(parse_mapping).collect::<Vec<_>>();

    let mut min = u64::MAX;

    for mut seed in seeds {
        for mapping in &mappings {
            seed = map_seed(seed, mapping);
        }

        if seed < min {
            min = seed;
        }
    }

    Some(min)
}

fn part2(input: &str) -> Option<u64> {
    let no_carriage_ret = input.replace("\r\n", "\n");
    let mut groups = no_carriage_ret.split("\n\n");

    let mut seed_ranges = parse_seeds_part2(groups.next()?)?;
    let mappings = groups.map(parse_mapping).collect::<Vec<_>>();

    let mut min = u64::MAX;

    for mapping in &mappings {
        let mut mapped = Vec::new();

        while let Some(mut seed_range) = seed_ranges.pop() {
            let mut found = false;

            for (src, dst) in mapping {
                if seed_range.start >= src.end || seed_range.end <= src.start {
                    continue;
                }

                found = true;

                if seed_range.start < src.start {
                    seed_ranges.push(seed_range.start..src.start);
                    seed_range.start = src.start;
                }
                if seed_range.end > src.end {
                    seed_ranges.push(src.end..seed_range.end);
                    seed_range.end = src.end;
                }

                // (dst.start - src.start) could undeflow
                let start = seed_range.start + dst.start - src.start;
                let end = seed_range.end + dst.start - src.start;
                mapped.push(start..end);
            }

            if !found {
                mapped.push(seed_range);
            }
        }

        seed_ranges = mapped;
    }

    for seed_range in seed_ranges {
        if seed_range.start < min {
            min = seed_range.start;
        }
    }

    Some(min)
}
