pub mod hand;

use crate::prelude::*;
use hand::Hand;

#[allow(dead_code)]
const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[test]
fn test_winning_points() {
    assert_eq!(6440, cal_winning_points(INPUT, false).unwrap());
}

#[test]
fn test_order() {
    //winnner is 33332 because 3 > 2
    let input = "33332 1
2AAAA 2";
    assert_eq!(2+2, cal_winning_points(input, false).unwrap());
}

#[test]
fn test_winning_points_joker() {
    assert_eq!(5905, cal_winning_points(INPUT, true).unwrap());
}

type TotalWinnings = u64;

pub fn cal_winning_points(input: &str, use_joker: bool) -> Result<TotalWinnings> {
    let mut games = Hand::parse(input, use_joker);
    games.sort();

    // println!("{:#?}", games);

    //test if a hand is duplicated. nod needed but used for debugging
    for (index, value) in games.iter().enumerate() {
        if let Some(next_value) = games.get(index + 1) {
            if value == next_value {
                panic!("Duplicate hand: {:#?}", value);
            }
        }
    }

    let winnings = games.into_iter()
    .rev()
    .enumerate()
    .map(|(i, hand)| {
        let place = i + 1;
        hand.bet * place as u64
    })
    .sum();

    Ok(winnings)
}

