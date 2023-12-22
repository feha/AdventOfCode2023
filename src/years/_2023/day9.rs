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
    let solution = input
        .lines()
        .map(|s| s.trim())
        .map(|line| {
            let row = line.split(' ')
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            let pyramid = diff_recurse(&row);
            if let Ok(arr) = pyramid {
                return arr.iter()
                    .rev()
                    .fold(0, |acc, diffs| {
                        let last = diffs.last().unwrap();
                        let extrapolated = last + acc;
                        extrapolated
                    });
            }
            panic!("diff_recurse errored: {:?}", pyramid);
        })
        .sum();

    return Ok(solution);
}

fn diff_recurse(ns: &Vec<isize>) -> Result<Vec<Vec<isize>>,()> {
    let ns = ns.clone();
    let windows = ns.windows(2);
    if windows.len() == 0 {
        return Err(());
    }
    let diffs = windows
        .map(|wind| wind[1] - wind[0])
        // .map(|wind| wind.as_slice())
        // .map(|[a, b]| b - a)
        .collect::<Vec<_>>();

    if diffs.iter().all(|diff| *diff == 0) {
        return Ok(vec![ns, diffs]);
    }
    return diff_recurse(&diffs)
        .map(|pyramid| vec![vec![ns], pyramid].concat());
}

fn part_2(input: &str) -> Result<isize, String> {
    let solution = input
        .lines()
        .map(|s| s.trim())
        .map(|line| {
            let row = line.split(' ')
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            let pyramid = diff_recurse(&row);
            if let Ok(arr) = pyramid {
                return arr.iter()
                    .rev()
                    .fold(0, |acc, diffs| {
                        let first = diffs.first().unwrap();
                        let extrapolated = first - acc;
                        extrapolated
                    });
            }
            panic!("diff_recurse errored: {:?}", pyramid);
        })
        .sum();

    return Ok(solution);
}


// Tests
#[cfg(test)]
mod tests
{
    use super :: * ;

    fn test_helper_1(s : & str, v : isize) {
        assert_eq!(part_1(s).unwrap(), v, "input: {}", s);
    }

    #[test]
    fn test_1() {
        // assert_eq!("", "");
        test_helper_1(r#"0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"#, 114);
    }

    fn test_helper_2(s : & str, v : isize)
    {
        assert_eq!(part_2(s).unwrap(), v, "input: {}", s);
    }

    #[test]
    fn test_2() {
        test_helper_2(r#"0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"#, 2);
    }
}


// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }