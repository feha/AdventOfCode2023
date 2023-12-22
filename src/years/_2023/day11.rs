#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.

use std::{convert::identity, cmp::{min, max}};

// mod utils; // imports utils.rs (not needed as its done in main.rs. not even possible for unknown reason)
use crate::utils::*; // needs to use crate, refers to crate populated by main.rs

use itertools::Itertools;
// Creates struct for Day#, implementing Day trait.
// Thus links passed functions to main.rs (as it has macros instantiating Day# structs for each Day#.rs file).
use proc_macro_aoc;
proc_macro_aoc::impl_day!(part_1, part_2);


// Implementations
fn part_1(input: &str) -> Result<isize, String> {
    let horizontally_expanded = input
        .lines()
        .map(|s| s.trim()
            .chars()
            .collect::<Vec<_>>())
        .flat_map(|row| 
            if row.clone().into_iter().all_equal() {
                vec![row.clone(), row]
            } else {
                vec![row]
            }
        )
        .collect::<Vec<_>>();
    let expanded_transposed = transpose(horizontally_expanded)
        .into_iter()
        .flat_map(|col|
            if col.clone().into_iter().all_equal() {
                return vec![col.clone(), col];
            } else {
                return vec![col];
        })
        .collect::<Vec<_>>();
    let expanded = transpose(expanded_transposed);

    let stars = expanded.into_iter()
        .enumerate()
        .flat_map(|(y, row)| row.into_iter()
            .enumerate()
            .map(move |(x, c)|
                if c == '#' {
                    Some((x,y))
                } else {
                    None
                }
            )
            .filter_map(identity)
        );
    
    // let shortest = stars.clone().map(|star|
    //     stars.clone().fold(None, |acc, star2|
    //             if star == star2 {
    //                 acc
    //             } else {
    //                 let dist = (star.0 as isize - star2.0 as isize).abs()
    //                     + (star.1 as isize - star2.1 as isize).abs();
    //                 if acc.is_none() || dist < acc.unwrap() {
    //                     Some(dist)
    //                 } else {
    //                     acc
    //                 }
    //             }
    //         ).unwrap()
    // );
    let distances = stars.clone().enumerate().flat_map(|(i, star)|
            stars.clone().skip(i).filter_map(move |star2|
                    if star == star2 {
                        None
                    } else {
                        let dist = (star.0 as isize - star2.0 as isize).abs()
                            + (star.1 as isize - star2.1 as isize).abs();
                        Some(dist)
                    }
                )
        );

    let solution = distances
        .sum::<isize>();

    return Ok(solution);
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn part_2(input: &str) -> Result<isize, String> {
    return part2_impl(input, 10usize.pow(6));
}

fn part2_impl(input: &str, n: usize) -> Result<isize, String> {

    let input = input
        .lines()
        .map(|s| s.trim()
            .chars()
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
        

    let empty_rows = input.clone()
        .into_iter()
        .enumerate()
        .filter(|(_, s)| s
            .into_iter()
            .all_equal()
        )
        .map(|(y, _)| y)
        .collect::<Vec<_>>();

    let empty_cols = transpose(input.clone())
        .iter()
        .enumerate()
        .filter(|(_, s)| s
            .into_iter()
            .all_equal()
        )
        .map(|(x, _)| x)
        .collect::<Vec<_>>();

    let stars = input.into_iter()
        .enumerate()
        .flat_map(|(y, row)| row.into_iter()
            .enumerate()
            .map(move |(x, c)|
                if c == '#' {
                    Some((x,y))
                } else {
                    None
                }
            )
            .filter_map(identity)
        )
        .collect::<Vec<_>>();
    
    let distances = stars.clone()
        .into_iter()
        .enumerate()
        .flat_map(|(i, star)|
            stars.clone().into_iter().skip(i).filter_map(|star2| {
                    if star != star2 {
                        let dist = (star.0 as isize - star2.0 as isize).abs()
                            + (star.1 as isize - star2.1 as isize).abs();
                        let empty_rows = empty_rows.clone()
                            .into_iter()
                            .filter(|y|
                                min(star.1, star2.1) < *y && *y < max(star.1, star2.1)
                            )
                            .count();
                        let empty_cols = empty_cols.clone()
                            .into_iter()
                            .filter(|x|
                                min(star.0, star2.0) < *x && *x < max(star.0, star2.0)
                            )
                            .count();
                        let dist = dist as usize + (empty_rows + empty_cols) * (n-1);
                        return Some(dist as isize);
                    }
                    return None;
                }).collect::<Vec<_>>()
        );

    let solution = distances
        .sum::<isize>();

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
        test_helper_1(r#"...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."#, 374);
    }

    fn test_helper_2(s : & str, v : isize, n: usize)
    {
        assert_eq!(part2_impl(s, n).unwrap(), v, "input: {}", s);
    }

    #[test]
    fn test_2() {
        test_helper_2(r#"...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."#, 374, 2);
        test_helper_2(r#"...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."#, 1030, 10);
        test_helper_2(r#"...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."#, 8410, 100);
    }
}


// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }