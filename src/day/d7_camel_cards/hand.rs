
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Ace,
    King, 
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Invalid card: {}", c),
        }
    }
}

type UnsortedHand = [Card; 5];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind(UnsortedHand),
    FourOfAKind(UnsortedHand),
    FullHouse(UnsortedHand),
    ThreeOfAKind(UnsortedHand),
    TwoPair(UnsortedHand),
    OnePair(UnsortedHand),
    HighCard(UnsortedHand),
}

impl HandType {

    fn parse(cards: &[char; 5]) -> Self {

        let unsorted_hand = cards.map(Card::from);        
        let mut sorted_hand = unsorted_hand;
        sorted_hand.sort();

        match sorted_hand {
            [a5, b5, c5, d5, e5] if a5 == b5 && b5 == c5 && c5 == d5 && d5 == e5 => Self::FiveOfAKind(unsorted_hand),

            [a4, b4, c4, d4, _] if a4 == b4 && b4 == c4 && c4 == d4 => Self::FourOfAKind(unsorted_hand),
            [_, b4, c4, d4, e4] if b4 == c4 && c4 == d4 && d4 == e4 => Self::FourOfAKind(unsorted_hand),

            [a3, b3, c3, d2, e2] if a3 == b3 && b3 == c3 && d2 == e2 => Self::FullHouse(unsorted_hand),
            [a2, b2, c3, d3, e3] if a2 == b2 && c3 == d3 && d3 == e3 => Self::FullHouse(unsorted_hand),

            [a3, b3, c3, _, _] if a3 == b3 && b3 == c3 => Self::ThreeOfAKind(unsorted_hand), 
            [_, b3, c3, d3, _] if b3 == c3 && c3 == d3 => Self::ThreeOfAKind(unsorted_hand),
            [_, _, c3, d3, e3] if c3 == d3 && d3 == e3 => Self::ThreeOfAKind(unsorted_hand),

            [a1, b1, c2, d2, _] if a1 == b1 && c2 == d2 => Self::TwoPair(unsorted_hand),
            [a1, b1, _, d2, e2] if a1 == b1 && d2 == e2 => Self::TwoPair(unsorted_hand),
            [_, b1, c1, d2, e2] if b1 == c1 && d2 == e2 => Self::TwoPair(unsorted_hand),

            [a1, b1, _, _, _] if a1 == b1 => Self::OnePair(unsorted_hand),
            [_, b1, c1, _, _] if b1 == c1 => Self::OnePair(unsorted_hand),
            [_, _, c1, d1, _] if c1 == d1 => Self::OnePair(unsorted_hand),
            [_, _, _, d1, e1] if d1 == e1 => Self::OnePair(unsorted_hand),

            [_, _, _, _, _] => Self::HighCard(unsorted_hand),
        }

    }

    
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    cards: HandType,
    pub bet: u64,
}

impl Hand {

    pub fn parse(input: &str) -> Vec<Self> {

       input.lines()
        .map(|line| {
            let (cards, bet) = line.split_once(' ').unwrap();
            let bet = bet.parse().unwrap();
            let chars = cards.chars().collect::<Vec<_>>();
            let hand = HandType::parse(chars.as_slice().try_into().unwrap());
            Self {
                cards: hand,
                bet,
            }
        })
        .collect()

    }

}

