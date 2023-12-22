#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.

use std::collections::HashMap;

// mod utils; // imports utils.rs (not needed as its done in main.rs. not even possible for unknown reason)
use crate::utils::*; // needs to use crate, refers to crate populated by main.rs

use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
// Creates struct for Day#, implementing Day trait.
// Thus links passed functions to main.rs (as it has macros instantiating Day# structs for each Day#.rs file).
use proc_macro_aoc;
use regex::Regex;
proc_macro_aoc::impl_day!(part_1, part_2);


// Implementations
fn part_1(input: &str) -> Result<isize, String> {
    let mut lines = input
        .lines()
        .map(|s| s.trim());

    let instr = lines.next().unwrap().chars().cycle();

    let re = Regex::new(r"(.+) = \((.+), (.+)\)").unwrap();

    let graph = lines.skip(1)
        .map(|line| {
            let captures = re.captures(line).unwrap()
                .iter()
                .collect::<Vec<_>>();
            let captures = &captures.into_iter()
                .map(|opt| opt.map(|m| m.as_str()))
                .collect::<Vec<_>>()[..];
            match captures {
                [_, Some(node), Some(left), Some(right)] => {
                    (*node, (*left, *right))
                }
                _ => panic!("Unexpected number of captures! {:?}", captures),
            }
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    let steps = instr.clone().fold_while(vec!["AAA"], |mut acc, c| {
        let node = acc.last().unwrap();
        let edges = graph[node];
        let node = match c {
            'L' => edges.0,
            'R' => edges.1,
            _ => panic!("Inalid instruction {}", c),
        };
        acc.push(node);

        if node == "ZZZ" {
            Done(acc)
        } else {
            Continue(acc)
        }
    }).into_inner();

    let solution = steps.len() - 1;

    return Ok(solution as isize);
}

fn part_2(input: &str) -> Result<isize, String> {
    let mut lines = input
        .lines()
        .map(|s| s.trim());

    let instr = lines.next().unwrap().chars().cycle();

    let re = Regex::new(r"(.+) = \((.+), (.+)\)").unwrap();

    let graph = lines.skip(1)
        .map(|line| {
            let captures = re.captures(line).unwrap()
                .iter()
                .collect::<Vec<_>>();
            let captures = &captures.into_iter()
                .map(|opt| opt.map(|m| m.as_str()))
                .collect::<Vec<_>>()[..];
            match captures {
                [_, Some(node), Some(left), Some(right)] => {
                    (*node, (*left, *right))
                }
                _ => panic!("Unexpected number of captures! {:?}", captures),
            }
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    let _memoizing: HashMap<&str, usize> = HashMap::new();

    let start_nodes = graph.keys()
        .map(|s| *s)
        .filter(|node| node.chars().last().unwrap() == 'A');
    
    let steps = start_nodes
        .map(|start|
            instr.clone()
                .fold_while(vec![start], |mut acc, c| {
                    let node = acc.last().unwrap();
                    let edges = graph[node];
                    let node = match c {
                        'L' => edges.0,
                        'R' => edges.1,
                        _ => panic!("Inalid instruction {}", c),
                    };
                    acc.push(node);
            
                    if node.chars().last().unwrap() == 'Z' {
                        Done(acc)
                    } else {
                        Continue(acc)
                    }
            }).into_inner()
            .len() - 1
        )
        .collect::<Vec<_>>();
    
    let solution = lcm(&steps);

    return Ok(solution as isize);
}

// fn lcm(ns: &Vec<usize>) -> usize {
//     let max_n = ns.clone().into_iter().max().unwrap();
//     (1..)
//         .skip_while(|n| {
//             let target = n * max_n;
//             !ns.clone().into_iter().all(|n| target % n == 0)
//         })
//         .next().unwrap() * max_n
// }
fn lcm(ns: &Vec<usize>) -> usize {
    ns.iter().fold(1, |acc, n| num::integer::lcm(acc, *n))
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
        test_helper_1(r#"RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)"#, 2);
        test_helper_1(r#"LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"#, 6);
    }

    fn test_helper_2(s : & str, v : isize)
    {
        assert_eq!(part_2(s).unwrap(), v, "input: {}", s);
    }

    #[test]
    fn test_2() {
        test_helper_2(r#"LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)"#, 6);
    }
}


// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }