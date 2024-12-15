use std::cmp::{max, min};

fn parse_input(input: &str) -> Option<Vec<&[u8]>> {
    assert!(input.is_ascii());

    Some(input.lines().map(|line| line.as_bytes()).collect())
}

fn count_occourrences(haystack: &[u8], forwards: &[u8], backwards: &[u8]) -> u32 {
    let len = forwards.len();

    haystack
        .windows(len)
        .filter(|&window| window == forwards || window == backwards)
        .count() as u32
}

fn search_pt1(matrix: &[&[u8]]) -> Option<u32> {
    let len = matrix.len();

    assert!(len == matrix.first()?.len());

    let mut haystack = Vec::with_capacity(len);
    let forwards = "XMAS".as_bytes();
    let backwards = "SAMX".as_bytes();

    let mut total = 0;

    for row in matrix.iter() {
        haystack.clear();
        (0..len).for_each(|y| haystack.push(row[y]));

        total += count_occourrences(&haystack, forwards, backwards);
    }

    for y in 0..len {
        haystack.clear();
        (0..len).for_each(|x| haystack.push(matrix[x][y]));

        total += count_occourrences(&haystack, forwards, backwards);
    }

    for q in -(len as isize) + 1..len as isize {
        haystack.clear();
        (0..len).for_each(|x| {
            let y = q + x as isize;

            if (0..len as isize).contains(&y) {
                haystack.push(matrix[x][y as usize]);
            }
        });

        total += count_occourrences(&haystack, forwards, backwards);
    }

    for q in 0..(len * 2 - 1) {
        haystack.clear();
        (0..len).for_each(|x| {
            let y = q as isize - x as isize;

            if (0..len as isize).contains(&y) {
                haystack.push(matrix[x][y as usize]);
            }
        });

        total += count_occourrences(&haystack, forwards, backwards);
    }

    Some(total)
}

fn matches(haystack1: &[u8], haystack2: &[u8], forwards: &[u8], backwards: &[u8]) -> bool {
    (haystack1 == forwards || haystack1 == backwards)
        && (haystack2 == forwards || haystack2 == backwards)
}

fn search_pt2(matrix: &[&[u8]]) -> Option<u32> {
    let len = matrix.len();

    assert!(len == matrix.first()?.len());

    let mut haystack1 = Vec::with_capacity(len);
    let mut haystack2 = Vec::with_capacity(len);
    let mut points = Vec::with_capacity(len);
    let forwards = "MAS".as_bytes();
    let backwards = "SAM".as_bytes();

    let mut total = 0;

    for q1 in -(len as isize) + 1..len as isize {
        haystack1.clear();
        points.clear();

        (0..len).for_each(|x| {
            let y = q1 + x as isize;

            if (0..len as isize).contains(&y) {
                points.push((haystack1.len(), x as isize, y));
                haystack1.push(matrix[x][y as usize]);
            }
        });

        for (pos, px, py) in &points {
            if !(1..haystack1.len() - 1).contains(pos) {
                continue;
            }

            for q2 in 0..(len * 2 - 1) {
                let rx = max(0, px - 1)..min(len as isize, px + 2);
                let ry = max(0, py - 1)..min(len as isize, py + 2);

                haystack2.clear();
                rx.for_each(|x| {
                    let y = q2 as isize - x;

                    if ry.contains(&y) {
                        haystack2.push(matrix[x as usize][y as usize]);
                    }
                });

                if haystack2.len() >= 3 {
                    let h1 = &haystack1[*pos - 1..pos + 2];

                    total += matches(h1, &haystack2, forwards, backwards) as u32;
                }
            }
        }
    }

    Some(total)
}

fn part1(input: &str) -> Option<u32> {
    let matrix = parse_input(input)?;

    search_pt1(&matrix)
}

fn part2(input: &str) -> Option<u32> {
    let matrix = parse_input(input)?;

    search_pt2(&matrix)
}

fn main() {
    let input0 = include_str!("input0.txt");
    let input1 = include_str!("input1.txt");

    println!("{:?}", part1(input0));
    println!("{:?}", part1(input1));

    println!("{:?}", part2(input0));
    println!("{:?}", part2(input1));
}
