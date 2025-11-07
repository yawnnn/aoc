use std::{cmp, iter, str};

struct Cuboid {
    l: u32,
    w: u32,
    h: u32,
}

#[allow(dead_code)]
enum CuboidSide {
    Front,
    Back,
    Up,
    Down,
    Left,
    Right,
}

impl Cuboid {
    fn side_surface(&self, side: CuboidSide) -> u32 {
        use CuboidSide as S;
        match side {
            S::Front | S::Back => self.l * self.h,
            S::Up | S::Down => self.l * self.w,
            S::Left | S::Right => self.w * self.h,
        }
    }
}

impl str::FromStr for Cuboid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = Cuboid { l: 0, w: 0, h: 0 };
        let dims = [&mut c.l, &mut c.w, &mut c.h];

        let str_dims: Vec<_> = s.split('x').collect();
        if str_dims.len() != dims.len() {
            return Err(());
        }

        for (dim, s) in iter::zip(dims, str_dims) {
            *dim = s.parse().map_err(|_| ())?;
        }
        
        Ok(c)
    }
}

fn main() {
    let input = include_str!("input1.txt");
    println!("{}", part1(input).unwrap_or(0));
    println!("{}", part2(input).unwrap_or(0));
}

fn part1(s: &str) -> Option<u32> {
    let cuboids = s
        .lines()
        .map(str::FromStr::from_str)
        .collect::<Result<Vec<Cuboid>, _>>()
        .ok()?;
    let mut total = 0;

    for c in cuboids {
        let s1 = c.side_surface(CuboidSide::Front);
        let s2 = c.side_surface(CuboidSide::Up);
        let s3 = c.side_surface(CuboidSide::Left);
        let min = cmp::min(cmp::min(s1, s2), s3);
        total += 2 * (s1 + s2 + s3) + min
    }

    Some(total)
}

fn part2(s: &str) -> Option<u32> {
    let cuboids = s
        .lines()
        .map(std::str::FromStr::from_str)
        .collect::<Result<Vec<Cuboid>, _>>()
        .ok()?;
    let mut total = 0;

    for r in cuboids {
        let mut dims = [r.l, r.w, r.h];
        dims.sort_unstable();
        let length = 2 * dims[0] + 2 * dims[1];
        let bow = dims.iter().product::<u32>();
        total += length + bow;
    }

    Some(total)
}
