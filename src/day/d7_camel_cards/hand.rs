
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

type Five = Card;
type Four = Card;
type Three = Card;
type Pair = Card;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind(Five),
    FourOfAKind(Four, Card),
    FullHouse(Three, Pair),
    ThreeOfAKind(Three, Card, Card),
    TwoPair(Pair, Pair, Card),
    OnePair(Pair, Card, Card, Card),
    HighCard(Card, Card, Card, Card, Card),
}

impl HandType {

    fn parse(cards: &[char; 5]) -> Self {

        let mut cards = cards.map(Card::from);        
        cards.sort();

        match cards {
            [a5, b5, c5, d5, e5] if a5 == b5 && b5 == c5 && c5 == d5 && d5 == e5 => Self::FiveOfAKind(a5),
            [a4, b4, c4, d4, e1] if a4 == b4 && b4 == c4 && c4 == d4 => Self::FourOfAKind(a4, e1),
            [a1, b4, c4, d4, e4] if b4 == c4 && c4 == d4 && d4 == e4 => Self::FourOfAKind(b4, a1),
            [a3, b3, c3, d2, e2] if a3 == b3 && b3 == c3 && d2 == e2 => Self::FullHouse(a3, d2),
            [a2, b2, c3, d3, e3] if a2 == b2 && c3 == d3 && d3 == e3 => Self::FullHouse(c3, a2),
            [a3, b3, c3, d1, e1] if a3 == b3 && b3 == c3 => Self::ThreeOfAKind(a3, d1, e1), 
            [a1, b3, c3, d3, e1] if b3 == c3 && c3 == d3 => Self::ThreeOfAKind(b3, a1, e1),
            [a1, b1, c3, d3, e3] if c3 == d3 && d3 == e3 => Self::ThreeOfAKind(c3, a1, b1),

            [a1, b1, c2, d2, e] if a1 == b1 && c2 == d2 => Self::TwoPair(a1, c2, e),
            [a1, b1, c, d2, e2] if a1 == b1 && d2 == e2 => Self::TwoPair(a1, d2, c),
            [a, b1, c1, d2, e2] if b1 == c1 && d2 == e2 => Self::TwoPair(b1, d2, a),

            [a1, b1, c, d, e] if a1 == b1 => Self::OnePair(a1, c, d, e),
            [a, b1, c1, d, e] if b1 == c1 => Self::OnePair(b1, a, d, e),
            [a, b, c1, d1, e] if c1 == d1 => Self::OnePair(c1, a, b, e),
            [a, b, c, d1, e1] if d1 == e1 => Self::OnePair(d1, a, b, c),

            [a, b, c, d, e] => Self::HighCard(a, b, c, d, e),
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

