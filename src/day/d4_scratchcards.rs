use std::collections::HashSet;

use crate::prelude::*;

#[test]
fn test_scratchcards() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    assert_eq!(13, cal_card_points(input).unwrap());
}


pub fn cal_card_points(input: &str) -> Result<u32> {
    Ok(input.lines().map(cal_points).sum::<u32>())
}

fn cal_points(card: &str) -> u32 {
    let (_, card) = card.split_once(": ").unwrap();
    let (winners, input) = card.split_once(" | ").unwrap();
    let winners = winners.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<HashSet<_>>();
    let input = input.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<HashSet<_>>();

    let count = winners.intersection(&input).count() as u32;
    if count == 0 {
        return 0;
    }
    2_u32.pow(count-1)
}







