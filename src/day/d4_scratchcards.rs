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

#[test]
fn test_scratchcards2() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    assert_eq!(30, count_cards(input).unwrap());
}

pub fn cal_card_points(input: &str) -> Result<u32> {
    Ok(input.lines().map(cal_points).sum::<u32>())
}

fn cal_hits(card: &str) -> u32 {

    let (_, card) = card.split_once(": ").unwrap();
    let (winners, input) = card.split_once(" | ").unwrap();
    let winners = winners.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<HashSet<_>>();
    let input = input.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<HashSet<_>>();

    winners.intersection(&input).count() as u32
}

fn cal_points(card: &str) -> u32 {
    let hits = cal_hits(card);
    if hits == 0 {
        return 0;
    }
    2_u32.pow(hits-1)
}

type Count = usize;
type Points = usize;
#[derive(Debug, Clone, Copy)]
struct Card(Count, Points);


pub fn count_cards(input: &str) -> Result<u32> {
    let mut cards = input.lines().map(cal_hits).map(|points| Card(1, points as usize)).collect::<Vec<_>>();
    cards.clone().into_iter().enumerate().for_each(|(game, Card(_, points))| {
        for i in 1..=points {
            cards[game+i].0 += cards[game].0;
        }
    });

    // println!("{:#?}", cards);

    let count = cards.iter().fold(0, |sum, Card(counts, _)| {
        sum + counts
    }) as u32;

    Ok(count)

}

#[test]
fn test_range() {
    for i in (0..5).rev() {
        println!("{}", i);
    }
}




