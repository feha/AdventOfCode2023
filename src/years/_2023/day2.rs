#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.

use std::{collections::HashMap, convert::identity};

// mod utils; // imports utils.rs (not needed as its done in main.rs. not even possible for unknown reason)
use crate::utils::*; // needs to use crate, refers to crate populated by main.rs

// Creates struct for Day#, implementing Day trait.
// Thus links passed functions to main.rs (as it has macros instantiating Day# structs for each Day#.rs file).
use proc_macro_aoc;
proc_macro_aoc::impl_day!(part_1, part_2);


// Implementations
fn part_1(input: &str) -> Result<isize, String> {
    let criteria = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let solution = input
        .lines()
        .map(|s| {
            let mut foo = s.split(':');
            let id = foo.next().unwrap().split(' ').last().unwrap().parse::<usize>().unwrap();
            let rhs = foo.next().unwrap();
            let shows: Vec<bool> = rhs.split(';')
                .map(|show| {
                    let show: HashMap<&str, usize> = show.split(',')
                        .map(|entry| {
                            let mut entry = entry.trim().split(' ');
                            let n = entry.next().unwrap().parse::<usize>().unwrap();
                            let color = entry.next().unwrap();
                            return (color, n);
                        })
                        .collect();
                    let impossible = show.iter()
                        .any(|(k, v)| {
                            // let k = *k;
                            criteria.get(k).unwrap() < v
                        });
                    impossible
                }).collect();
            let impossible = shows.into_iter().any(identity);
            (id, impossible)
        })
        .filter(|(_, impossible)| !impossible)
        .map(|(id, _)| id)
        .fold(0, |sum, x| sum + x);

    return Ok(solution as isize);
}

fn part_2(input: &str) -> Result<isize, String> {
    let solution: usize = input
        .lines()
        .map(|s| {
            let mut foo = s.split(':');
            let _id = foo.next().unwrap().split(' ').last().unwrap().parse::<usize>().unwrap();
            let rhs = foo.next().unwrap();
            let shows: Vec<HashMap<&str, usize>> = rhs.split(';')
                .map(|show| {
                    let show: HashMap<&str, usize> = show.split(',')
                        .map(|entry| {
                            let mut entry = entry.trim().split(' ');
                            let n = entry.next().unwrap().parse::<usize>().unwrap();
                            let color = entry.next().unwrap();
                            return (color, n);
                        })
                        .collect();
                    show
                }).collect();
            let mut shows2: HashMap<&str, Vec<usize>> = HashMap::from([
                ("red", vec![]),
                ("green", vec![]),
                ("blue", vec![]),
            ]);
            shows.iter().for_each(|m| m.iter().for_each(|(k, v)| {
                let arr = shows2.get_mut(k).unwrap();
                arr.push(*v);
            }));
            let power: usize = shows2.iter()
                .map(|(_k, arr)| *(*arr).iter().max().unwrap())
                .product();
            power
        })
        .sum();

    return Ok(solution as isize);
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
        test_helper_1(r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#, 8);
    }

    fn test_helper_2(s : & str, v : isize)
    {
        assert_eq! (part_2(s).unwrap(), v) ;
    }

    #[test]
    fn test_2() {
        test_helper_2(r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#, 2286);
    }
}


// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }