#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.

use std::{collections::HashMap, cmp::Ordering};

// mod utils; // imports utils.rs (not needed as its done in main.rs. not even possible for unknown reason)
use crate::utils::*; // needs to use crate, refers to crate populated by main.rs

use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
// Creates struct for Day#, implementing Day trait.
// Thus links passed functions to main.rs (as it has macros instantiating Day# structs for each Day#.rs file).
use proc_macro_aoc;
proc_macro_aoc::impl_day!(part_1, part_2);


// Implementations
fn part_1(input: &str) -> Result<isize, String> {
    let solution = input
        .lines()
        .map(|s| s.trim())
        .map(|line| {
            let foo = line.split(' ').collect::<Vec<_>>();

            let hand = foo[0]
                .chars()
                .map(|c|
                    Card {
                        value: match c {
                            'A' => 14,
                            'K' => 13,
                            'Q' => 12,
                            'J' => 11,
                            'T' => 10,
                            _ => c.to_string().parse::<usize>().unwrap(),
                        },
                    }
                ).collect::<Vec<_>>();

            let hand_type = hand.clone().into_iter()
                .fold(HashMap::<Card, usize>::new(), |mut acc, card| {
                    let entry = acc.entry(card).or_insert(0);
                    *entry += 1;
                    acc
                })
                .into_iter()
                .sorted_by(|a, b| Ord::cmp(&b.1, &a.1)) // "b, a" to sort descending order
                .fold_while(Hand::Empty, |acc, (card, n)|
                    match n {
                        5 => Done(Hand::Five(card)),
                        4 => Done(Hand::Four(card)),
                        3 => match acc {
                            Hand::Pair(card2) => Done(Hand::House(card, card2)),
                            _ => Continue(Hand::Three(card)),
                        },
                        2 => match acc {
                            Hand::Three(card2) => Done(Hand::House(card2, card)),
                            Hand::Pair(card2) => Done(Hand::PairPair(card, card2)),
                            _ => Continue(Hand::Pair(card)),
                        },
                        1 => match acc.clone() {
                            Hand::Three(_) => Done(acc),
                            Hand::Pair(_) => Continue(acc),
                            Hand::High(card2) => {
                                if card < card2 { Continue(acc) } else { Continue(Hand::High(card)) }
                            },
                            _ => Continue(Hand::High(card)),
                        },
                        _ => panic!("Found more cards of type than hand should be able to contain!"),
                    }).into_inner();

            let bid = foo[1].parse::<usize>().unwrap();

            (hand, hand_type, bid)
        })
        .sorted_by(|(hand1, hand_type1, _), (hand2, hand_type2, _)| {
            let hand_ranks = vec![hand_type1, hand_type2].into_iter()
                .map(|hand_type| match hand_type {
                        Hand::Five(_) => 7,
                        Hand::Four(_) => 6,
                        Hand::House(_, _) => 5,
                        Hand::Three(_) => 4,
                        Hand::PairPair(_, _) => 3,
                        Hand::Pair(_) => 2,
                        Hand::High(_) => 1,
                        _ => panic!("Unexpected hand!"),
                    }
                )
                .collect::<Vec<_>>();
            let cmp = Ord::cmp(&hand_ranks[0], &hand_ranks[1]);
            if Ordering::Equal == cmp {
                return hand1.into_iter().zip(hand2)
                    .fold(Ordering::Equal, |acc, (card1, card2)| {
                        if Ordering::Equal == acc {
                            if card1 > card2 {
                                return Ordering::Greater;
                            } else if card1 < card2 {
                                return Ordering::Less;
                            }
                        }
                        return acc;
                    });
            }
            cmp
        })
        .enumerate()
        .map(|(i, (_, _, bid))| bid * (i + 1))
        .sum::<usize>();

    return Ok(solution as isize);
}

fn part_2(input: &str) -> Result<isize, String> {
    let bets = input
        .lines()
        .map(|s| s.trim())
        .map(|line| {
            let foo = line.split(' ').collect::<Vec<_>>();
            
            let hand = foo[0]
                .chars()
                .map(|c|
                    Card {
                        value: match c {
                            'A' => 14,
                            'K' => 13,
                            'Q' => 12,
                            'J' => 1,
                            'T' => 10,
                            _ => c.to_string().parse::<usize>().unwrap(),
                        },
                    }
                ).collect::<Vec<_>>();

            let hand_type = hand.clone().into_iter()
                .fold(HashMap::<Card, (usize, usize)>::new(), |mut acc, card| {
                    if card.value == 1 { // joker
                        (1..15).into_iter()
                            .map(|card| Card { value: card })
                            .for_each(|card| {
                                let entry = acc.entry(card).or_insert((0, 0));
                                (*entry).1 += 1;
                            });
                    } else {
                        let entry = acc.entry(card).or_insert((0, 0));
                        (*entry).0 += 1;
                    }
                    acc
                })
                .into_iter()
                .sorted_by(|a, b| {
                    // sorting by n+jokers so most valuable joker-combination appears first
                    // "cmp(rhs, lhs)" to sort descending order using the n:usize,
                    // resolving ties with card-value.
                    let lhs = a.1;
                    let rhs = b.1;
                    let lhs = lhs.0 + lhs.1; // n + jokers
                    let rhs = rhs.0 + rhs.1;
                    let cmp = Ord::cmp(&rhs, &lhs);
                    if Ordering::Equal == cmp { // this secondary sorting should be unnecessary
                        return Ord::cmp(&b.0, &a.0);
                    }
                    cmp
                }) 
                .fold_while((Hand::Empty, 0), |(hand, jokers_seen), (card, (n, jokers))| {
                    let count = n+jokers-jokers_seen;
                    let jokers_seen = jokers; // toggle away jokers
                    match count {
                        5 => Done((Hand::Five(card), jokers_seen)),
                        4 => Done((Hand::Four(card), jokers_seen)),
                        3 => match hand {
                            Hand::Pair(card2) => Done((Hand::House(card, card2), jokers_seen)),
                            _ => Continue((Hand::Three(card), jokers_seen)),
                        },
                        2 => match hand {
                            Hand::Three(card2) => Done((Hand::House(card2, card), jokers_seen)),
                            Hand::Pair(card2) => Done((Hand::PairPair(card, card2), jokers_seen)),
                            _ => Continue((Hand::Pair(card), jokers_seen)),
                        },
                        1 => match hand.clone() {
                            Hand::Three(_) => Done((hand, jokers_seen)),
                            Hand::Pair(_) => Continue((hand, jokers_seen)),
                            Hand::High(card2) => {
                                if card < card2 { Continue((hand, jokers_seen)) } else { Continue((Hand::High(card), jokers_seen)) }
                            },
                            _ => Continue((Hand::High(card), jokers_seen)),
                        },
                        0 => Continue((hand, jokers_seen)),
                        _ => panic!("Found more cards of type than hand should be able to contain!"),
                    }
                }).into_inner().0;

            let bid = foo[1].parse::<usize>().unwrap();

            (hand, hand_type, bid)
        })
        .sorted_by(|(hand1, hand_type1, _), (hand2, hand_type2, _)| {
            let hand_ranks = vec![hand_type1, hand_type2].into_iter()
                .map(|hand_type| match hand_type {
                        Hand::Five(_) => 7,
                        Hand::Four(_) => 6,
                        Hand::House(_, _) => 5,
                        Hand::Three(_) => 4,
                        Hand::PairPair(_, _) => 3,
                        Hand::Pair(_) => 2,
                        Hand::High(_) => 1,
                        _ => panic!("Unexpected hand!"),
                    }
                )
                .collect::<Vec<_>>();

            let cmp = Ord::cmp(&hand_ranks[0], &hand_ranks[1]);
            if Ordering::Equal == cmp {
                return hand1.into_iter().zip(hand2)
                    .fold(Ordering::Equal, |acc, (card1, card2)| {
                        if Ordering::Equal == acc {
                            if card1 > card2 {
                                return Ordering::Greater;
                            } else if card1 < card2 {
                                return Ordering::Less;
                            }
                        }
                        return acc;
                    });
            }
            cmp
        })
        .enumerate()
        .collect::<Vec<_>>();

    let solution = bets.into_iter()
        .map(|(i, (_, _, bid))| bid * (i + 1))
        .sum::<usize>();

    return Ok(solution as isize);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Card {
    value: usize,
}
#[derive(Debug, Clone)]
enum Hand {
    Five(Card),
    Four(Card),
    House(Card, Card),
    Three(Card),
    PairPair(Card, Card),
    Pair(Card),
    High(Card),
    Empty,
}


// Tests
#[cfg(test)]
mod tests
{
    use super :: * ;

    fn test_helper_1(s : & str, v : isize) {
        assert_eq! (part_1(s).unwrap(), v) ;
    }

    #[test]
    fn test_1() {
        // assert_eq!("", "");
        test_helper_1(r#"32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"#, 6440);
    }

    fn test_helper_2(s : & str, v : isize)
    {
        assert_eq!(part_2(s).unwrap(), v, "input: {}", s);
    }

    #[test]
    fn test_2() {
        test_helper_2(r#"32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"#, 5905);
        test_helper_2(r#"JJJJJ 1
        QQQQ2 10
        JKKK2 100"#, 123);
        test_helper_2(r#"KKKKK 1
        KKKKJ 10
        KKKJJ 100"#, 123);
        test_helper_2(r#"65433 1
        22456 10
        J2348 100"#, 123);
        test_helper_2(r#"72772 10
        8Q278 100
        QQJQQ 1"#, 123);
    }
}


// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }