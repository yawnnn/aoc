use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
enum Card {
    Digit(char),
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(c: char) -> Result<Card, Self::Error> {
        match c {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            c if c.is_ascii_digit() => Ok(Card::Digit(c)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u8)]
enum Hand {
    HighCard(Card),
    OnePair(Card),
    TwoPair(Card, Card),
    ThreeOfAKind(Card),
    FullHouse(Card, Card),
    FourOfAKind(Card),
    FiveOfAKind(Card),
}

impl Hand {
    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, rust guarantees a specific memory layout.
        // Most importatly for this, the discriminant is at the beginning
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    fn join(&self, other: &Hand) -> Hand {
        use Hand::*;
        let __self = *self;
        let __other = *other;

        match __self {
            FiveOfAKind(_) | FourOfAKind(_) | FullHouse(_, _) | TwoPair(_, _) => __self,
            ThreeOfAKind(c1) => match __other {
                OnePair(c2) => FullHouse(c1, c2),
                _ => __self,
            },
            OnePair(c1) => match __other {
                OnePair(c2) => TwoPair(c1, c2),
                ThreeOfAKind(c2) => FullHouse(c2, c1),
                _ => __self,
            },
            HighCard(_) => match self.cmp(other) {
                std::cmp::Ordering::Greater => __self,
                _ => __other,
            }
        }
    }
}

impl TryFrom<&[Card]> for Hand 
{
    type Error = ();

    fn try_from(cards: &[Card]) -> Result<Hand, Self::Error> {
        use Hand::*;
        let mut current_best: Option<Hand> = None;
        let mut occurrences = HashMap::new();

        cards.iter().for_each(|c| {
            occurrences.entry(c).or_insert(cards.iter().filter(|&c2| c2 == c).count());
        });

        for (&c, count) in occurrences {
            let next = match count {
                5 => FiveOfAKind(c),
                4 => FourOfAKind(c),
                3 => ThreeOfAKind(c),
                2 => OnePair(c),
                _ => HighCard(c),
            };

            let new_best = match current_best {
                Some(best_hand) => best_hand.join(&next),
                None => next,
            };

            current_best = Some(new_best);
        }
    
        current_best.ok_or(())
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

#[derive(Debug, Eq, PartialEq)]
struct HandData {
    cards: Vec<Card>,
    hand: Hand,
    bid: u32,
}

impl Ord for HandData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hand_ord = self.hand.cmp(&other.hand);

        match hand_ord {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
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

fn parse_input_line(line: &str) -> Option<HandData> {
    let mut data = line.split(' ');
    let hand_s = data.next()?;

    let cards = hand_s.chars().map(|c| c.try_into().ok()).collect::<Option<Vec<_>>>()?;
    let hand = Hand::try_from(&cards[..]).ok()?;
    let bid = data.next()?.parse().ok()?;

    Some(HandData {
        cards,
        hand,
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
