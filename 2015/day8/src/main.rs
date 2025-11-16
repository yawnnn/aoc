#[derive(Debug)]
enum State {
    Str,
    Escape,
    Hex1,
    Hex2(u8),
}

fn hex_val(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => 10 + (c - b'a'),
        b'A'..=b'F' => 10 + (c - b'A'),
        _ => unreachable!(),
    }
}

fn parse_input1(input: &str) -> Vec<(&str, Vec<u8>)> {
    input
        .lines()
        .map(|line| {
            let s = &line[1..line.len() - 1];
            let mut s2 = Vec::with_capacity(s.len());

            let mut state = State::Str;

            for b in s.bytes() {
                state = match state {
                    State::Str => match b {
                        b'\\' => State::Escape,
                        _ => {
                            s2.push(b);

                            State::Str
                        }
                    },
                    State::Escape => match b {
                        b'x' => State::Hex1,
                        _ => {
                            s2.push(b);

                            State::Str
                        }
                    },
                    State::Hex1 => match b {
                        b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => {
                            let high = hex_val(b);

                            State::Hex2(high)
                        }
                        _ => panic!("{state:?}"),
                    },

                    State::Hex2(high) => match b {
                        b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => {
                            let low = hex_val(b);
                            s2.push((high << 4) | low);

                            State::Str
                        }
                        _ => panic!("{state:?}"),
                    },
                }
            }

            (line, s2)
        })
        .collect()
}

fn parse_input2(input: &str) -> Vec<(&str, String)> {
    input
        .lines()
        .map(|line| {
            //let s2 = format!("{line:?}"); --- easy mode
            let mut s2 = String::with_capacity(line.len()+2);
            s2.push('"');

            for c in line.chars() {
                match c {
                    '\\' | '"' => s2.push('\\'),
                    _ => ()
                }
                s2.push(c);
            }

            s2.push('"');

            (line, s2)
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let strs = parse_input1(input);

    let sums = strs
        .iter()
        .map(|(s1, s2)| (s1.len(), s2.len()))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    sums.0 - sums.1
}

fn part2(input: &str) -> usize {
    let strs = parse_input2(input);

    let sums = strs
        .iter()
        .map(|(s1, s2)| (s1.len(), s2.len()))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    sums.1 - sums.0
}

fn main() {
    let input0 = include_str!("input0.txt");
    let input1 = include_str!("input1.txt");

    println!("{:?}", part1(input0));
    println!("{:?}", part1(input1));

    println!("{:?}", part2(input0));
    println!("{:?}", part2(input1));
}
