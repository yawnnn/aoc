struct Rect {
    dims: Vec<u32>,
}

impl std::str::FromStr for Rect {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rect: Vec<u32> = s
            .split('x')
            .enumerate()
            .take_while(|(p, s)| *p < 3)
            .map(|(p, s)| { s.parse::<u32>().map_err(|_| ()).unwrap() })
            .collect();

        if rect.len() == 3 {
            Ok(Rect{ dims: rect })
        } else {
            Err(())
        }

        // let parts: Vec<&str> = s.split('x').collect();
        // let mut dims: [u32; 3];

        // if parts.len() == 3 {
        //     let a: Vec<u32> = parts.iter().map(|&s| s.parse().map_err(|_| ())?).collect();
        //     return Ok(Rect{ dims: a });
        // }

        // Err(())
    }
}

fn main() {
    let input = include_str!("input1.txt");
    println!("{}", part1(input).unwrap_or(0));
}

fn part1(s: &str) -> Option<i32> {
    use std::str::FromStr;
    let mut dims: Vec<Rect> = vec![];

    let dims: Vec<Rect> = s.lines().map(|s| Rect::from_str(s).ok().unwrap()).collect();
}
