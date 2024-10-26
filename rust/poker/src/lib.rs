use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
enum CardValue {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
enum CardSuit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone, Copy, Debug)]
struct Card {
    value: CardValue,
    suit: CardSuit,
}

impl Card {
    fn new(s: &str) -> Option<Self> {
        if s.is_empty() {
            return None;
        }
        let (value, suit) = s.split_at(s.len() - 1);
        let suit = match suit {
            "c" | "C" => Some(CardSuit::Clubs),
            "d" | "D" => Some(CardSuit::Diamonds),
            "h" | "H" => Some(CardSuit::Hearts),
            "s" | "S" => Some(CardSuit::Spades),
            _ => None,
        }?;
        let value = match value {
            "a" | "A" | "1" => Some(CardValue::Ace),
            "2" => Some(CardValue::Two),
            "3" => Some(CardValue::Three),
            "4" => Some(CardValue::Four),
            "5" => Some(CardValue::Five),
            "6" => Some(CardValue::Six),
            "7" => Some(CardValue::Seven),
            "8" => Some(CardValue::Eight),
            "9" => Some(CardValue::Nine),
            "10" => Some(CardValue::Ten),
            "j" | "J" => Some(CardValue::Jack),
            "q" | "Q" => Some(CardValue::Queen),
            "k" | "K" => Some(CardValue::King),
            _ => None,
        }?;
        Some(Card { value, suit })
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap() == Ordering::Equal
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.partial_cmp(&other.value).unwrap()
    }
}

impl Eq for Card {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
enum Hand {
    HighCard(Card, Card, Card, Card, Card),
    OnePair((Card, Card), Card, Card, Card),
    TwoPair((Card, Card), (Card, Card), Card),
    ThreeOfAKind((Card, Card, Card), Card, Card),
    Straight(Card, Card, Card, Card, Card),
    Flush(Card, Card, Card, Card, Card),
    FullHouse((Card, Card, Card), (Card, Card)),
    FourOfAKind((Card, Card, Card, Card), Card),
    StraightFlush(Card, Card, Card, Card, Card),
}

fn is_flush(cards: &[Card]) -> bool {
    let mut cards = cards.iter();
    let suit = cards.next().unwrap().suit;
    cards.all(|card| card.suit == suit)
}

fn is_straight(cards: &Vec<Card>) -> bool {
    let cards = cards.as_slice();
    match cards {
        [Card {
            value: CardValue::Ace,
            ..
        }, Card {
            value: CardValue::Five,
            ..
        }, Card {
            value: CardValue::Four,
            ..
        }, Card {
            value: CardValue::Three,
            ..
        }, Card {
            value: CardValue::Two,
            ..
        }] => true,
        [Card { value: value0, .. }, ..] => {
            let max_value = *value0 as i8;
            cards
                .iter()
                .enumerate()
                .all(|(i, card)| (card.value as i8) == max_value - i as i8)
        }
        _ => unreachable!(),
    }
}

impl Hand {
    fn new(slice: &str) -> Option<Hand> {
        let mut cards = slice
            .split_ascii_whitespace()
            .map(Card::new)
            .collect::<Option<Vec<Card>>>()?;
        if cards.len() != 5 {
            return None;
        }
        // Highest card is first
        cards.sort_by(|a, b| b.cmp(a));
        // Straight flush
        let flush = is_flush(&cards);
        let straight = is_straight(&cards);
        if flush && straight {
            if cards[0].value == CardValue::Ace && cards[1].value == CardValue::Five {
                return Some(Hand::StraightFlush(
                    cards[1], cards[2], cards[3], cards[4], cards[0],
                ));
            }
            return Some(Hand::StraightFlush(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            ));
        }
        // Four-of-a-kind
        for (i, window) in cards.windows(4).enumerate() {
            if window.first() == window.last() {
                match (i, cards.as_slice()) {
                    (0, [_, _, _, _, remainder]) | (1, [remainder, _, _, _, _]) => {
                        return Some(Hand::FourOfAKind(
                            (window[0], window[1], window[2], window[3]),
                            *remainder,
                        ))
                    }
                    _ => (),
                }
            }
        }
        // Full house
        for (i, window) in cards.windows(3).enumerate() {
            if window.first() == window.last() {
                match (i, cards.as_slice()) {
                    (0, [_, _, _, remainder_0, remainder_1])
                    | (2, [remainder_0, remainder_1, _, _, _])
                        if remainder_0 == remainder_1 =>
                    {
                        return Some(Hand::FullHouse(
                            (window[0], window[1], window[2]),
                            (*remainder_0, *remainder_1),
                        ));
                    }
                    _ => (),
                }
            }
        }
        // Flush
        if flush {
            return Some(Hand::Flush(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            ));
        }
        // Straight
        if straight {
            if cards[0].value == CardValue::Ace && cards[1].value == CardValue::Five {
                return Some(Hand::Straight(
                    cards[1], cards[2], cards[3], cards[4], cards[0],
                ));
            }
            return Some(Hand::Straight(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            ));
        }
        // Three-of-a-kind
        for (i, window) in cards.windows(3).enumerate() {
            if window.first() == window.last() {
                match (i, cards.as_slice()) {
                    (0, [_, _, _, remainder_0, remainder_1])
                    | (1, [remainder_0, _, _, _, remainder_1])
                    | (2, [remainder_0, remainder_1, _, _, _]) => {
                        return Some(Hand::ThreeOfAKind(
                            (window[0], window[1], window[2]),
                            *remainder_0,
                            *remainder_1,
                        ));
                    }
                    _ => (),
                }
            }
        }
        // Two pair
        for (i, window) in cards.windows(2).enumerate() {
            if window.first() == window.last() {
                match (i, cards.as_slice()) {
                    (0, [_, _, remainder_0, remainder_1, remainder_2])
                        if remainder_0 == remainder_1 =>
                    {
                        return Some(Hand::TwoPair(
                            (window[0], window[1]),
                            (*remainder_0, *remainder_1),
                            *remainder_2,
                        ))
                    }
                    (0, [_, _, remainder_0, remainder_1, remainder_2])
                    | (1, [remainder_0, _, _, remainder_1, remainder_2])
                        if remainder_1 == remainder_2 =>
                    {
                        return Some(Hand::TwoPair(
                            (window[0], window[1]),
                            (*remainder_1, *remainder_2),
                            *remainder_0,
                        ))
                    }
                    (2, [remainder_0, remainder_1, _, _, _])
                    | (3, [remainder_0, remainder_1, _, _, _])
                    | (3, [_, remainder_0, remainder_1, _, _])
                        if remainder_0 == remainder_1 =>
                    {
                        unreachable!()
                    }
                    _ => (),
                }
            }
        }
        // One pair
        for (i, window) in cards.windows(2).enumerate() {
            if window.first() == window.last() {
                match (i, cards.as_slice()) {
                    (0, [_, _, remainder_0, remainder_1, remainder_2])
                    | (1, [remainder_0, _, _, remainder_1, remainder_2])
                    | (2, [remainder_0, remainder_1, _, _, remainder_2])
                    | (3, [remainder_0, remainder_1, remainder_2, _, _]) => {
                        return Some(Hand::OnePair(
                            (window[0], window[1]),
                            *remainder_0,
                            *remainder_1,
                            *remainder_2,
                        ));
                    }
                    _ => (),
                }
            }
        }
        // High card
        Some(Hand::HighCard(
            cards[0], cards[1], cards[2], cards[3], cards[4],
        ))
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.is_empty() {
        return vec![];
    }
    if hands.len() == 1 {
        return vec![hands[0]];
    }
    let parsed_hands: Vec<Hand> = hands.iter().map(|hand| Hand::new(hand).unwrap()).collect();
    let winning_hand = parsed_hands.iter().max().unwrap();
    hands
        .iter()
        .zip(parsed_hands.iter().map(|hand| hand == winning_hand))
        .filter(|(_, is_winning_hand)| *is_winning_hand)
        .map(|(&hand, _)| hand)
        .collect()
}
