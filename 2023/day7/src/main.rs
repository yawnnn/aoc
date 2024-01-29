#[derive(Debug, Eq, PartialEq)]
#[repr(u8)]
enum Hand {
    HighCard(char),
    OnePair(char),
    TwoPair(char, char),
    ThreeOfAKind(char),
    FullHouse(char, char),
    FourOfAKind(char),
    FiveOfAKind(char),
}

impl Hand {
    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.discriminant().cmp(&other.discriminant())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Card {
    Digit(char),
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Eq, PartialEq)]
struct HandData {
    hand: Vec<Card>,
    hand_type: Hand,
    bid: u32,
}

impl Ord for HandData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hand_type_ord = self.hand_type.cmp(&other.hand_type);

        match hand_type_ord {
            std::cmp::Ordering::Equal => self.hand.cmp(&other.hand),
            other => other,
        }
    }
}

impl PartialOrd for HandData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("input1.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn parse_hand(s: &str) -> Option<Hand> {
    use Hand::*;
    let mut best = HighCard(s.chars().next()?);

    for c in s.chars() {
        let count = s.chars().filter(|&c2| c2 == c).count();

        best = match count {
            5 => FiveOfAKind(c),
            4 => FourOfAKind(c),
            3 => match best {
                OnePair(c2) if c2 != c => FullHouse(c, c2),
                _ => ThreeOfAKind(c),
            },
            2 => match best {
                ThreeOfAKind(c2) if c2 != c => FullHouse(c2, c),
                ThreeOfAKind(_) => best,
                OnePair(c2) if c2 != c => TwoPair(c, c2),
                _ => OnePair(c),
            },
            _ => match best {
                HighCard(c2) if c.cmp(&c2) == std::cmp::Ordering::Greater => HighCard(c),
                _ => best,
            },
        };

        match best {
            FiveOfAKind(_) | FourOfAKind(_) | FullHouse(_, _) | TwoPair(_, _) => break,
            _ => (),
        }
    }

    Some(best)
}

fn map_card(c: char) -> Option<Card> {
    match c {
        'A' => Some(Card::A),
        'K' => Some(Card::K),
        'Q' => Some(Card::Q),
        'J' => Some(Card::J),
        'T' => Some(Card::T),
        c if c.is_ascii_digit() => Some(Card::Digit(c)),
        _ => None,
    }
}

fn parse_input_line(line: &str) -> Option<HandData> {
    let mut data = line.split(' ');
    let hand_s = data.next()?;
    let hand_type = parse_hand(hand_s)?;
    let bid = data.next()?.parse().ok()?;
    let hand = hand_s.chars().map(map_card).collect::<Option<Vec<_>>>()?;

    Some(HandData {
        hand,
        hand_type,
        bid,
    })
}

fn part1(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(parse_input_line)
        .collect::<Option<Vec<_>>>()?;

    hands.sort();

    Some(
        hands
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, hand)| acc + (hand.bid * (idx as u32 + 1))),
    )
}

fn part2(input: &str) -> Option<()> {
    todo!()
}
