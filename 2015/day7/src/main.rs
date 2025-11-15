use std::collections::HashMap;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Gate {
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOT,
    ASSIGN,
}

impl Gate {
    fn exec(self, in1: u16, in2: u16) -> u16 {
        match self {
            Gate::AND => in1 & in2,
            Gate::OR => in1 | in2,
            Gate::LSHIFT => in1 << in2,
            Gate::RSHIFT => in1 >> in2,
            Gate::NOT => !in2,
            Gate::ASSIGN => in1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Scheme<'a> {
    gate: Gate,
    in1: &'a str,
    in2: &'a str,
}

fn parse_input<'a>(input: &'a str) -> HashMap<&'a str, Scheme<'a>> {
    let table = [
        ("AND", Gate::AND),
        ("OR", Gate::OR),
        ("LSHIFT", Gate::LSHIFT),
        ("RSHIFT", Gate::RSHIFT),
        ("NOT", Gate::NOT),
    ];

    input
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(" -> ").unwrap();

            let wire = table
                .into_iter()
                .find_map(|t| {
                    let (in1, in2) = lhs.split_once(t.0)?;

                    Some(Scheme {
                        gate: t.1,
                        in1: in1.trim(),
                        in2: in2.trim(),
                    })
                })
                .unwrap_or(Scheme {
                    gate: Gate::ASSIGN,
                    in1: lhs,
                    in2: "",
                });

            (rhs, wire)
        })
        .collect::<_>()
}

fn walk_scheme<'a>(
    schemes: &HashMap<&'a str, Scheme<'a>>,
    chain: &mut Vec<&'a str>,
    p: &Scheme<'a>,
) {
    walk_wire(schemes, chain, p.in1);
    walk_wire(schemes, chain, p.in2);
}

fn walk_wire<'a>(schemes: &HashMap<&'a str, Scheme<'a>>, chain: &mut Vec<&'a str>, wire: &'a str) {
    if !wire.is_empty() && wire.parse::<u16>().is_err() && !chain.contains(&wire) {
        walk_scheme(schemes, chain, schemes.get(wire).unwrap());
        chain.push(wire);
    }
}

fn calc(schemes: &HashMap<&str, Scheme>, chain: &[&str], target: &str) -> Option<u16> {
    let mut values: HashMap<&str, u16> = HashMap::new();

    for elem in chain {
        let scheme = schemes.get(elem).unwrap();
        let val1 = values
            .get(scheme.in1)
            .copied()
            .unwrap_or_else(|| scheme.in1.parse().unwrap_or_default());
        let val2 = values
            .get(scheme.in2)
            .copied()
            .unwrap_or_else(|| scheme.in2.parse().unwrap_or_default());
        let val = scheme.gate.exec(val1, val2);

        values.insert(elem, val);
    }

    values.get(target).copied()
}

fn part1(input: &str, target: &str) -> Option<u16> {
    let schemes = parse_input(input);

    let mut chain = vec![];
    walk_wire(&schemes, &mut chain, target);
    
    calc(&schemes, &chain, target)
}

fn part2(input: &str, target: &str) -> Option<u16> {
    let mut schemes = parse_input(input);

    let mut chain = vec![];
    walk_wire(&schemes, &mut chain, target);

    let val = calc(&schemes, &chain, target).unwrap();

    let sval = val.to_string();
    *schemes.get_mut("b").unwrap() = Scheme {
        gate: Gate::ASSIGN,
        in1: &sval,
        in2: "",
    };

    calc(&schemes, &chain, target)
}

fn main() {
    //let input0 = include_str!("input0.txt");
    let input1 = include_str!("input1.txt");

    //println!("{:?}", part1(input0, "d"));
    println!("{:?}", part1(input1, "a"));

    //println!("{:?}", part2(input0, "d"));
    println!("{:?}", part2(input1, "a"));
}
