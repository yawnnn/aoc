struct Schematic<'a> {
    data: &'a str,
    rows: usize,
    cols: usize,
    carriage_return: bool,
}

impl<'a> std::ops::Deref for Schematic<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a> Schematic<'a> {
    fn new(input: &'a str) -> Self {
        if !input.is_ascii() {
            panic!("Input format not in ascii");
        }
    
        Schematic {
            data: input,
            rows: input.lines().count(),
            cols: input.lines().next().unwrap().len(),
            carriage_return: input.find('\r').is_some(),
        }
    }

    fn get(&self, col: usize, row: usize) -> Option<char> {
        let x = col.clamp(0, self.cols - 1);
        let y = row.clamp(0, self.rows - 1);
        let cols = self.cols + if self.carriage_return { 2 } else { 1 };
        self.as_bytes().get(x + y * cols).map(|b| *b as char)
    }

    fn get_part_number(&self, col: usize, row: usize) -> Option<u32> {
        let c = self.get(col, row)?;
        let mut ok = false;

        if c.is_ascii_digit() {
            let line = &self.lines().nth(row)?[col..];
            let num_end = line
                .char_indices()
                .find_map(|(p, c)| if !c.is_ascii_digit() { Some(p) } else { None })
                .unwrap_or(line.len());
            let num_str = &line[..num_end];
            let num_len = num_str.len() as i32;

            for dx in -1..=num_len {
                for dy in -1..=1 {
                    let x = (col as i32 + dx) as usize;
                    let y = (row as i32 + dy) as usize;
                    let neighbor = self.get(x, y)?;

                    if neighbor != '.' && !neighbor.is_alphanumeric() {
                        ok = true;
                        break;
                    }
                }

                if ok { break; }
            }

            if ok {
                return num_str.parse::<u32>().ok();
            }
        }
        None
    }

    fn get_gear_ratio(&self, col: usize, row: usize) -> Option<u32> {
        let c = self.get(col, row)?;

        if c == '*' {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let x = (col as i32 + dx) as usize;
                    let y = (row as i32 + dy) as usize;
                    let neighbor = self.get(x, y)?;

                    if neighbor.is_ascii_digit() {
                        let line = self.lines().nth(row)?;
                        let mut start = x;


                        let (p, _) = if x > col {
                            line[x..].chars()
                            .take_while(|c| c.is_ascii_digit())
                            .enumerate()
                            .last()?
                        } else {
                            line[..=x].chars().rev()
                            .take_while(|c| c.is_ascii_digit())
                            .enumerate()
                            .last()?
                        };
                    }
                }
            }
        }
        None
    }
}

fn main() {
    let input = include_str!("input1.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> Option<u32> {
    let schematic = Schematic::new(input);

    let mut total = 0;
    let mut last_c = '\0';

    for (row, line) in schematic.lines().enumerate() {
        for (col, c) in line.char_indices() {
            if !last_c.is_ascii_digit() {
                if let Some(num) = schematic.get_part_number(col, row) {
                    total += num;
                }
            }

            last_c = c;
        }
    }
    Some(total)
}

fn part2(input: &str) -> Option<()> {
    todo!()
}
