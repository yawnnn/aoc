use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
#[repr(u8)]
enum Card {
    Digit(char),
    T,
    J(char),
    Q,
    K,
    A,
}

impl Card {
    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, rust guarantees a specific memory layout.
        // Most importatly for this, the discriminant is at the beginning
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    fn cmp_canonical_order(&self, other: &Card) -> Ordering {
        match self.discriminant().cmp(&other.discriminant()) {
            Ordering::Equal => match (self, other) {
                (Card::Digit(c1), Card::Digit(c2)) => c1.cmp(c2),
                _ => Ordering::Equal,
            },
            cmp => cmp,
        }
    }

    fn cmp_part1(&self, other: &Card) -> Ordering {
        self.cmp_canonical_order(other)
    }

    // in part2 J is considered the lowest
    fn cmp_part2(&self, other: &Card) -> Ordering {
        match (self, other) {
            (Card::J(_), Card::J(_)) => Ordering::Equal,
            (Card::J(_), _) => Ordering::Less,
            (_, Card::J(_)) => Ordering::Greater,
            _ => self.cmp_canonical_order(other),
        }
    }
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(c: char) -> Result<Card, Self::Error> {
        match c {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J(c)),
            'T' => Ok(Card::T),
            c if c.is_ascii_digit() => Ok(Card::Digit(c)),
            _ => Err(()),
        }
    }
}

impl From<Card> for char {
    fn from(value: Card) -> Self {
        match value {
            Card::Digit(char) => char,
            Card::T => 'T',
            Card::J(_) => 'J',
            Card::Q => 'Q',
            Card::K => 'K',
            Card::A => 'A',
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
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
                Ordering::Greater => __self,
                _ => __other,
            },
        }
    }

    fn try_from_cards(cards: &[Card]) -> Option<Hand> {
        use Hand::*;
        let mut current_best: Option<Hand> = None;
        let mut occurrences = HashMap::new();

        // in part2 J can assume another value, and i need to consider that one
        let real_cards = cards
            .iter()
            .map(|&c| match c {
                Card::J(fake_value) if fake_value != 'J' => Card::try_from(fake_value).unwrap(),
                _ => c,
            })
            .collect::<Vec<_>>();

        real_cards.iter().for_each(|c| {
            occurrences
                .entry(c)
                .or_insert(real_cards.iter().filter(|&c2| c2 == c).count());
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

        current_best
    }

    fn try_from_part1(cards: &[Card]) -> Option<Hand> {
        Self::try_from_cards(cards)
    }

    fn try_joker_values(start: usize, cards: &[Card]) -> Option<Hand> {
        let mut best = Self::try_from_cards(cards)?;

        if let Some(card) = cards.get(start) {
            if matches!(card, Card::J(_)) {
                let mut options = cards
                    .iter()
                    .filter(|c| !matches!(c, Card::J(_)))
                    .collect::<HashSet<_>>();
                let mut tmp = Vec::from(cards);

                // in case i have JJJJJ
                if options.is_empty() {
                    options.insert(&Card::A);
                }

                for option in options {
                    tmp[start] = Card::J((*option).into());

                    if let Some(other) = Self::try_joker_values(start + 1, &tmp) {
                        if best.cmp(&other) == Ordering::Less {
                            best = other;
                        }
                    }
                }
            } else if let Some(other) = Self::try_joker_values(start + 1, cards) {
                if best.cmp(&other) == Ordering::Less {
                    best = other;
                }
            }
        }

        Some(best)
    }

    fn try_from_part2(cards: &[Card]) -> Option<Hand> {
        Self::try_joker_values(0, cards)
    }
}

// it needs to ignore the data, and only use the discriminant
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.discriminant().cmp(&other.discriminant())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct HandData {
    cards: Vec<Card>,
    hand: Hand,
    bid: u32,
}

impl HandData {
    // alternatively could have made a newtype, but it wasn't really worth it imo
    fn cmp_cards<F>(&self, other: &Self, cmp: F) -> Ordering
    where
        F: Fn(&Card, &Card) -> Ordering,
    {
        self.cards
            .iter()
            .zip(&other.cards)
            .map(|(self_card, other_card)| cmp(self_card, other_card))
            .find(|&ordering| ordering != Ordering::Equal)
            .unwrap_or(Ordering::Equal)
    }

    fn cmp_cards_part1(&self, other: &Self) -> Ordering {
        self.cmp_cards(other, Card::cmp_part1)
    }

    fn cmp_cards_part2(&self, other: &Self) -> Ordering {
        self.cmp_cards(other, Card::cmp_part2)
    }

    fn cmp_part1(&self, other: &Self) -> Ordering {
        match self.hand.cmp(&other.hand) {
            Ordering::Equal => self.cmp_cards_part1(other),
            not_eq => not_eq,
        }
    }

    fn cmp_part2(&self, other: &Self) -> Ordering {
        match self.hand.cmp(&other.hand) {
            Ordering::Equal => self.cmp_cards_part2(other),
            not_eq => not_eq,
        }
    }

    fn try_from_str<F>(line: &str, hand_try_from: F) -> Option<HandData>
    where
        F: Fn(&[Card]) -> Option<Hand>,
    {
        let mut data = line.split(' ');
        let hand_s = data.next()?;

        let cards = hand_s
            .chars()
            .map(|c| c.try_into().ok())
            .collect::<Option<Vec<_>>>()?;
        let hand = hand_try_from(&cards[..])?;
        let bid = data.next()?.parse().ok()?;

        Some(HandData { cards, hand, bid })
    }

    fn try_from_part1(line: &str) -> Option<HandData> {
        Self::try_from_str(line, Hand::try_from_part1)
    }

    fn try_from_part2(line: &str) -> Option<HandData> {
        Self::try_from_str(line, Hand::try_from_part2)
    }
}

fn main() {
    let input = include_str!("input1.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(HandData::try_from_part1)
        .collect::<Option<Vec<_>>>()?;

    hands.sort_by(HandData::cmp_part1);

    Some(
        hands
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, hand)| acc + (hand.bid * (idx as u32 + 1))),
    )
}

fn part2(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(HandData::try_from_part2)
        .collect::<Option<Vec<_>>>()?;

    hands.sort_by(HandData::cmp_part2);

    Some(
        hands
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, hand)| acc + (hand.bid * (idx as u32 + 1))),
    )
}
