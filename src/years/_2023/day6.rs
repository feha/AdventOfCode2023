#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.

// mod utils; // imports utils.rs (not needed as its done in main.rs. not even possible for unknown reason)
use crate::utils::*; // needs to use crate, refers to crate populated by main.rs

use itertools::Itertools;
// Creates struct for Day#, implementing Day trait.
// Thus links passed functions to main.rs (as it has macros instantiating Day# structs for each Day#.rs file).
use proc_macro_aoc;
proc_macro_aoc::impl_day!(part_1, part_2);


// Implementations
fn part_1(input: &str) -> Result<isize, String> {
    let rows = input.split('\n')
        .map(|s| s.trim())
        .collect::<Vec<_>>();

    let times = rows[0].split(':').collect::<Vec<_>>()[1]
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|c| c.parse::<usize>().unwrap());
    let records = rows[1].split(':').collect::<Vec<_>>()[1]
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|c| c.parse::<usize>().unwrap());

    let races = times.zip(records);

    let wins: Vec<usize> = races
        .map(|(time, record)| 
            (0..time)
                .map(|n| n * (time - n)) // n*time - n^2
                .filter(|dist| *dist > record)
                .count()
        )
        .collect();

    let solution: usize = wins.iter().product();

    return Ok(solution as isize);
}

fn part_2(input: &str) -> Result<isize, String> {
    let rows = input.split('\n')
        .map(|s| s.trim())
        .collect::<Vec<_>>();

    let time = rows[0].split(':').collect::<Vec<_>>()[1]
        .split(' ').join("")
        .parse::<usize>().unwrap();
    let record = rows[1].split(':').collect::<Vec<_>>()[1]
        .split(' ').join("")
        .parse::<usize>().unwrap();

    let wins = (0..time)
        .map(|n| n * (time - n))
        .filter(|dist| *dist > record)
        .count();

    let solution: usize = wins;

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
        test_helper_1(r#"Time:      7  15   30
        Distance:  9  40  200"#, 288);
    }

    fn test_helper_2(s : & str, v : isize)
    {
        assert_eq! (part_2(s).unwrap(), v) ;
    }

    #[test]
    fn test_2() {
        test_helper_2(r#"Time:      7  15   30
        Distance:  9  40  200"#, 71503);
    }
}


// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }