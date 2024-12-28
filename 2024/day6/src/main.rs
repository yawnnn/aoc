const EMPTY_BLOCK: char = '.';
const START_BLOCK: char = '^';
const WALL_BLOCK: char = '#';

fn coords_1to2(pos: usize, size: (isize, isize)) -> (isize, isize) {
    (
        (pos / size.0 as usize) as isize,
        (pos % size.0 as usize) as isize,
    )
}

fn coords_2to1(pos: (isize, isize), size: (isize, isize)) -> usize {
    pos.1 as usize + pos.0 as usize * size.0 as usize
}

fn add(v1: (isize, isize), v2: (isize, isize)) -> (isize, isize) {
    (v1.0 + v2.0, v1.1 + v2.1)
}

fn rotate(v: (isize, isize)) -> (isize, isize) {
    const ROTATION: [(isize, isize); 2] = [(0, -1), (1, 0)];
    (
        v.0 * ROTATION[0].0 + v.1 * ROTATION[1].0,
        v.0 * ROTATION[0].1 + v.1 * ROTATION[1].1,
    )
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> Option<(Vec<char>, (isize, isize), (isize, isize))> {
    let w = input.lines().count();
    let h = input.lines().next()?.len();
    let size = (w as isize, h as isize);

    let grid = input.lines().flat_map(|l| l.chars()).collect::<Vec<_>>();

    let start = grid
        .iter()
        .position(|&c| c == START_BLOCK)
        .map(|p| coords_1to2(p, size))?;

    Some((grid, size, start))
}

fn is_inside(pos: (isize, isize), size: (isize, isize)) -> bool {
    (0..size.0).contains(&pos.0) && (0..size.1).contains(&pos.1)
}

fn step(
    grid: &[char],
    size: (isize, isize),
    pos: (isize, isize),
    dir: (isize, isize),
) -> ((isize, isize), (isize, isize)) {
    let mut next_pos = add(pos, dir);
    let mut next_dir = dir;
    let mut max_iter = 0..4;

    while max_iter.next().is_some()
        && is_inside(next_pos, size)
        && grid[coords_2to1(next_pos, size)] == WALL_BLOCK
    {
        next_dir = rotate(next_dir);
        next_pos = add(pos, next_dir);
    }

    (next_pos, next_dir)
}

fn traverse(
    grid: &[char],
    size: (isize, isize),
    start_pos: (isize, isize),
) -> (bool, Vec<(isize, isize)>) {
    let mut pos = start_pos;
    let mut dir = (-1, 0);
    let mut visited = vec![(0, 0); grid.len()];
    let mut is_loop = false;

    while is_inside(pos, size) {
        if visited[coords_2to1(pos, size)] == dir {
            is_loop = true;
            break;
        }
        visited[coords_2to1(pos, size)] = dir;

        (pos, dir) = step(grid, size, pos, dir);
    }

    (is_loop, visited)
}

fn part1(input: &str) -> Option<u32> {
    let (grid, size, start) = parse_input(input)?;

    let (_, visited) = traverse(&grid, size, start);
    let count = visited.iter().filter(|&d| *d != (0, 0)).count() as u32;

    Some(count)
}

fn part2(input: &str) -> Option<u32> {
    let (mut grid, size, start) = parse_input(input)?;

    let (_, visited) = traverse(&grid, size, start);

    let start_1d = coords_2to1(start, size);

    let total = visited
        .into_iter()
        .enumerate()
        .filter(|&(p, d)| p != start_1d && d != (0, 0))
        .fold(0, |total, (pos_1d, _)| {
            grid[pos_1d] = WALL_BLOCK;
            let (is_loop, _) = traverse(&grid, size, start);
            grid[pos_1d] = EMPTY_BLOCK;

            total + is_loop as u32
        });

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
