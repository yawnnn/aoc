struct Input {
    data: String,
    width: usize,
    height: usize,
}

impl Input {
    fn from(input: &str) -> Self {
        let width = match input.find('\r') {
            Some(width) => width,
            None => input.find('\n').unwrap(),
        };
        let height = input.lines().count();
        let data = input.replace("\r\n", "").replace('\n', "");

        Input {
            data,
            width,
            height,
        }
    }

    #[inline]
    fn coords_1d_to_2d(&self, pos: usize) -> (usize, usize) {
        (pos % self.width, pos / self.width)
    }

    #[inline]
    fn coords_2d_to_1d(&self, (x, y): (usize, usize)) -> usize {
        x + (y * self.width)
    }

    fn get(&self, pos: (usize, usize)) -> char {
        self.data.as_bytes()[self.coords_2d_to_1d(pos)] as char
    }

    fn find_origin(&self) -> Option<(usize, usize)> {
        let pos_1d = self.data.find('S')?;
        Some(self.coords_1d_to_2d(pos_1d))
    }

    fn coords_if_ok(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        if x < self.width && y < self.height {
            Some((x, y))
        } else {
            None
        }
    }

    fn count_tiles_inside_path(&self, walls: &[Option<Pipe>]) -> u64 {
        let mut count = 0;
        let horizontal_pipe = Pipe::try_from('-').unwrap();

        for y in 0..self.height {
            let mut inside = false;

            for x in 0..self.width {
                let curr = walls[self.coords_2d_to_1d((x, y))];

                match curr {
                    Some(pipe) if pipe != horizontal_pipe => inside = !inside,
                    None if inside => {
                        println!("{x}, {y}, {}", self.coords_2d_to_1d((x, y)));
                        count += 1;
                    }
                    _ => (),
                }
            }
        }

        count
    }

    fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get((x, y)));
            }
            println!();
        }
    }

    fn draw_walls(&self, walls: &[Option<Pipe>]) {
        for y in 0..self.height {
            for x in 0..self.width {
                match walls[self.coords_2d_to_1d((x, y))] {
                    Some(_) => print!("X"),
                    None => print!("."),
                }
            }
            println!();
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn go(self, (x, y): (usize, usize), input: &Input) -> Option<(usize, usize)> {
        match self {
            Self::Up if y > 0 => Some((x, y - 1)),
            Self::Down => input.coords_if_ok((x, y + 1)),
            Self::Left if x > 0 => Some((x - 1, y)),
            Self::Right => input.coords_if_ok((x + 1, y)),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct Pipe(Direction, Direction);

impl Pipe {
    fn enter(self, from: Direction) -> Option<Direction> {
        match from {
            from if self.0 == from => Some(self.1),
            from if self.1 == from => Some(self.0),
            _ => None,
        }
    }

    fn from_neighbors(from1: Direction, from2: Direction) -> Pipe {
        if ['|', '-', 'L', 'F', 'J', '7']
            .into_iter()
            .filter_map(Pipe::try_from)
            .any(|p| p == Pipe(from1, from2))
        {
            Pipe(from1, from2)
        } else {
            Pipe(from2, from1)
        }
    }

    fn try_from(value: char) -> Option<Self> {
        use Direction::*;
        match value {
            '|' => Some(Pipe(Up, Down)),
            '-' => Some(Pipe(Left, Right)),
            'L' => Some(Pipe(Up, Right)),
            'F' => Some(Pipe(Down, Right)),
            'J' => Some(Pipe(Left, Up)),
            '7' => Some(Pipe(Left, Down)),
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
struct Tile {
    pos: (usize, usize),
    from: Direction,
    to: Direction,
    pipe: Pipe,
}

impl Tile {
    fn find_paths_from_origin(origin: (usize, usize), input: &Input) -> Vec<Tile> {
        use Direction::*;
        let make_dummy_pos = |dir| Tile {
            pos: origin,
            from: dir,
            to: dir,
            pipe: Pipe(dir, dir),
        };

        let paths = [Up, Down, Left, Right]
            .iter()
            .filter_map(|&dir| Tile::try_from(make_dummy_pos(dir), input))
            .collect::<Vec<_>>();

        assert_eq!(paths.len(), 2);

        paths
    }

    fn try_from(from: Tile, input: &Input) -> Option<Self> {
        let pos = from.to.go(from.pos, input)?;
        let pipe = Pipe::try_from(input.get(pos))?;
        let to = pipe.enter(from.to.get_opposite())?;

        Some(Tile {
            pos,
            from: from.to,
            to,
            pipe,
        })
    }
}

fn main() {
    let input = include_str!("input0.txt");

    // if it's not ascii the code isn't right
    // since i know it is, for the sake of simplicity, i'll just leave the check and not worry about it
    if !input.is_ascii() {
        panic!("input not in ascii");
    }

    let input = Input::from(input);

    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}

fn part1(input: &Input) -> Option<u64> {
    let origin = input.find_origin()?;
    let mut paths = Tile::find_paths_from_origin(origin, input);

    let mut steps = 1;
    loop {
        for path in paths.iter_mut() {
            *path = Tile::try_from(*path, input)?;
        }

        steps += 1;

        if paths[0].pos == paths[1].pos {
            break;
        }
    }

    Some(steps)
}

fn part2(input: &Input) -> Option<u64> {
    let origin = input.find_origin()?;
    let mut paths = Tile::find_paths_from_origin(origin, input);
    let mut walls = vec![None; input.width * input.height];

    walls[input.coords_2d_to_1d(origin)] = Some(Pipe::from_neighbors(paths[0].from, paths[1].from));
    for path in paths.iter() {
        walls[input.coords_2d_to_1d(path.pos)] = Some(path.pipe);
    }

    loop {
        for path in paths.iter_mut() {
            *path = Tile::try_from(*path, input)?;
            walls[input.coords_2d_to_1d(path.pos)] = Some(path.pipe);
        }

        if paths[0].pos == paths[1].pos {
            break;
        }
    }

    println!("w: {}, h: {}", input.width, input.height);
    println!();
    input.draw();
    println!();
    input.draw_walls(&walls);
    println!();
    Some(input.count_tiles_inside_path(&walls))
}
