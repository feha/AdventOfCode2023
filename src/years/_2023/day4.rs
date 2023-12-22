#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.

// mod utils; // imports utils.rs (not needed as its done in main.rs. not even possible for unknown reason)
use crate::utils::*; // needs to use crate, refers to crate populated by main.rs

// Creates struct for Day#, implementing Day trait.
// Thus links passed functions to main.rs (as it has macros instantiating Day# structs for each Day#.rs file).
use proc_macro_aoc;
proc_macro_aoc::impl_day!(part_1, part_2);


// Implementations
fn part_1(input: &str) -> Result<isize, String> {
    let solution: isize = input
        .lines()
        .map(|s| s.trim())
        .map(|s| {
            let mut foo = s.split(':');
            let id = foo.next().unwrap()
                .split(' ')
                .last().unwrap()
                .parse::<usize>().unwrap();
            let rhs = foo.next().unwrap();
            let mut bar = rhs.split('|');
            let winning = bar.next().unwrap()
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let candidates = bar.next().unwrap()
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|c| c.parse::<usize>().unwrap());

            (id, winning, candidates)
        })
        .map(|(id, winning, candidates)| {
            let wins = candidates.filter(|n| winning.contains(&n))
                .count();

            (id, wins)
        })
        .map(|(_id, n)| if n <= 0 { 0 } else { 1 << (n-1) })
        .sum();

    return Ok(solution);
}

fn part_2(input: &str) -> Result<isize, String> {
    let cards = input
        .lines()
        .map(|s| s.trim())
        .map(|s| {
            let mut foo = s.split(':');
            let _id = foo.next().unwrap()
                .split(' ')
                .last().unwrap()
                .parse::<usize>().unwrap();
            let rhs = foo.next().unwrap();
            let mut bar = rhs.split('|');
            let winning = bar.next().unwrap()
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let candidates = bar.next().unwrap()
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|c| c.parse::<usize>().unwrap());

            (winning, candidates)
        })
        .map(|(winning, candidates)| {
            let wins = candidates.filter(|n| winning.contains(&n))
                .count();

            wins
        })
        .collect::<Vec<_>>();

    let mut deck: Vec<usize> = cards.clone().into_iter().map(|wins| wins).collect();

    // reverse to enable dynamic programming (no need to recurse if last is calculated first)
    let binding = cards.clone();
    let solution = binding.iter().enumerate().rev()
        // flat_map(|i, (id, wins)| cards.drop(id+1)).take(wins).sum();
        .map(|(id, _wins)| {
            let wins = deck[id];
            let spawns = 1 + deck.clone().iter().skip(id+1).take(wins).sum::<usize>();
            deck[id] = spawns;
            spawns
        });
    // println!("{:?}", solution.clone().collect::<Vec<_>>());
        // .sum();

    return Ok(solution.sum::<usize>() as isize);
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
        test_helper_1(r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#, 13);
    }

    fn test_helper_2(s : & str, v : isize)
    {
        assert_eq! (part_2(s).unwrap(), v) ;
    }

    #[test]
    fn test_2() {
        test_helper_2(r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#, 30);
    }
}


// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }