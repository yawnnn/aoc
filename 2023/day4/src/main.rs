use std::collections::HashSet;

#[derive(Debug)]
struct Scratchcard {
    wins: u32,
    owned: u32,
}

fn main() {
    let input = include_str!("input1.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> Option<u32> {
    let mut total = 0;

    for line in input.lines() {
        let data = line.split(':').nth(1)?;
        let mut number_groups = [None, None];

        for (nums_s, nums) in data.split('|').zip(number_groups.iter_mut()) {
            *nums = Some(
                nums_s
                    .split(' ')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect::<HashSet<u32>>(),
            );
        }

        let winning = number_groups[0].take()?;
        let found = number_groups[1].take()?;
        let wins = winning.intersection(&found).count() as u32;

        if wins > 0 {
            total += 2_u32.pow(wins - 1);
        }
    }

    Some(total)
}

fn part2(input: &str) -> Option<u32> {
    let mut total = 0;
    let mut scrathcards = Vec::new();

    for line in input.lines() {
        let data = line.split(':').nth(1)?;
        let mut number_groups = [None, None];

        for (nums_s, nums) in data.split('|').zip(number_groups.iter_mut()) {
            *nums = Some(
                nums_s
                    .split(' ')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect::<HashSet<u32>>(),
            );
        }

        let winning = number_groups[0].take()?;
        let found = number_groups[1].take()?;
        let wins = winning.intersection(&found).count() as u32;

        scrathcards.push(Scratchcard { wins, owned: 1 });
    }

    for i in 0..scrathcards.len() {
        // i could use match/if let with .get(),
        // but really, if the for loop is wrong the program SHOULD panic and let me know
        let wins = scrathcards[i].wins;
        let owned = scrathcards[i].owned;

        if wins > 0 {
            scrathcards[i + 1..=i + wins as usize]
                .iter_mut()
                .for_each(|card| card.owned += owned);
        }

        total += owned;
    }

    Some(total)
}
