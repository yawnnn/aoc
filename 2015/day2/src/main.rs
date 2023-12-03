struct Shape3D {
    l: u32,
    w: u32,
    h: u32,
}

#[allow(dead_code)]
enum Rect3DSide {
    Front, Back,
    Up, Down,
    Left, Right
}

trait Rect3DSurface {
    fn surface_side(&self, side: Rect3DSide) -> u32;
    fn surface(&self) -> u32;
}

impl Rect3DSurface for Shape3D {
    fn surface_side(&self, side: Rect3DSide) -> u32 {
        use Rect3DSide::*;
        match side {
            Front | Back => self.l * self.h,
            Up | Down => self.l * self.w,
            Left | Right => self.w * self.h,
        }
    }
    
    fn surface(&self) -> u32 {
        use Rect3DSide::*;
        2 * (self.surface_side(Front) + self.surface_side(Up) + self.surface_side(Left))
    }
}

impl std::str::FromStr for Shape3D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r = Shape3D { l: 0, w: 0, h: 0 };
        let box_dims = [&mut r.l, &mut r.w, &mut r.h];

        let input_dims: Vec<_> = s.split('x').collect();
        if input_dims.len() == 3 {
            for (val, s) in std::iter::zip(box_dims, input_dims) {
                *val = s.parse().map_err(|_| ())?;
            }
            return Ok(r);
        }
        Err(())
    }
}

fn main() {
    let input = include_str!("input1.txt");
    println!("{}", part1(input).unwrap_or(0));
    println!("{}", part2(input).unwrap_or(0));
}

fn part1(s: &str) -> Option<u32> {
    // let boxes: Vec<_> = s
    //     .lines()
    //     .filter_map(|s| Shape3D::from_str(s).ok())
    //     .collect();
    let boxes = s
        .lines()
        .map(std::str::FromStr::from_str)
        .collect::<Result<Vec<Shape3D>, _>>();

    let boxes = boxes.ok()?;
    let mut total = 0;

    for r in boxes {
        let s1 = r.surface_side(Rect3DSide::Front);
        let s2 = r.surface_side(Rect3DSide::Up);
        let s3 = r.surface_side(Rect3DSide::Left);
        let min = std::cmp::min(std::cmp::min(s1, s2), s3);
        total += 2 * (s1 + s2 + s3) + min
    }

    Some(total)
}

fn part2(s: &str) -> Option<u32> {
    let boxes = s
        .lines()
        .map(std::str::FromStr::from_str)
        .collect::<Result<Vec<Shape3D>, _>>();

    let boxes = boxes.ok()?;
    let mut total = 0;

    for r in boxes {
        let mut dims = [ r.l, r.w, r.h ];
        dims.sort_unstable();
        let length = 2 * dims[0] + 2 * dims[1];
        //let bow = dims[0] * dims[1] * dims[2];
        let bow = dims.iter().product::<u32>();
        total += length + bow;
    }

    Some(total)
}
