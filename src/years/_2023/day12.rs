#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.

use std::{iter::repeat, collections::HashMap};

// mod utils; // imports utils.rs (not needed as its done in main.rs. not even possible for unknown reason)
use crate::utils::*; // needs to use crate, refers to crate populated by main.rs

use itertools::Itertools;
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
            let mut foo = line.split(' ');
            let gears = foo.next().unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Gear::Operational,
                    '#' => Gear::Damaged,
                    '?' => Gear::Unknown,
                    _ => panic!("Unexpected gear symbol!"),
                })
                .collect::<Vec<_>>();

            let groups = foo.next().unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            valid_permutations(gears, groups)
        })
        .sum::<u128>();

    return Ok(solution as isize);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Gear {
    Operational,
    Damaged,
    Unknown,
}
impl std::fmt::Display for Gear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Gear::Operational => write!(f, "."),
            Gear::Damaged => write!(f, "#"),
            Gear::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum StateNum<T> where T: num::Num {
    Unlocked(T),
    Opened(T),
    Locked(T),
}

fn to_pretty_permutation_string(gears: &Vec<Gear>) -> String {
    gears.iter().map(|gear| match gear {
        Gear::Operational => '.',
        Gear::Damaged => '#',
        Gear::Unknown => '?',
    })
    .collect::<String>()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Group {
    id: usize,
    size: usize,
}


fn valid_permutations(gears: Vec<Gear>, groups: Vec<usize>) -> u128 {
    let subgears: Vec<Vec<_>> = gears
        .split(|gear| *gear == Gear::Operational)
        .map(|gears| gears.to_vec())
        .filter(|gears| !gears.is_empty())
        .collect();

    let groups: Vec<Group> = groups.iter()
        .enumerate()
        .map(|(i, n)| Group {id: i, size: *n})
        .collect();
    let groups_len = groups.len()+1;
    println!("gears: {:?}", gears);
    println!("subgears: {:?}", subgears);
    println!("groups: {:?}", groups);

    let groups_superset = (1..groups_len).
        flat_map(|n| 
            groups
                .windows(n)
                .map(|window| window.into_iter().cloned().collect())
        )
        // .chain(vec![vec![]])
        .collect::<Vec<Vec<_>>>();

    let subgears_valid_groups = subgears.iter()
        .map(|gears| {
            groups_superset.iter()
                .cloned()
                .map(|subgroups| {
                    let foo: Vec<usize> = subgroups.iter()
                        .map(|group| group.size)
                        .collect();
                    let subgroups = subgroups.iter()
                        .map(|group| group.id)
                        .collect();
                    let n = valid_permutations2(gears.clone(), foo.clone());
                    println!("{:?} - {:?} - {}", gears.clone(), foo.clone(), n);
                    (gears.clone(), subgroups, n)
                })
                .filter(|(_, _, n)| *n > 0)
                .collect::<Vec<(_,_,_)>>()
        })
        .collect::<Vec<_>>();
    println!("subgears_valid_groups: {:?}", subgears_valid_groups);

    subgears_valid_groups.into_iter()
        .fold(HashMap::from([(vec![], 1)]), |acc: HashMap<Vec<usize>, u128>, valid_groupss: Vec<(Vec<Gear>, Vec<usize>, u128)>| {
            println!("acc: {:?}", acc);
            let new_acc = acc.iter()
                .flat_map(|(seen_groups, n)| {
                    let filtered1 = valid_groupss.iter()
                        .filter(|(_, valid_groups, _)|
                            valid_groups.is_empty() || valid_groups.iter().any(|group|
                                    !seen_groups.contains(group)
                                )
                        );
                    // println!("filtered1: {:?}", filtered1.clone().collect::<Vec<_>>());
                    let filtered2 = filtered1
                        .filter(|(_, unseen_groups, _)| {
                            let last_seen = seen_groups.iter().max();
                            let first_unseen = unseen_groups.iter().min();
                            if first_unseen.is_none() {
                                return true;
                            }
                            let first_unseen = *first_unseen.unwrap();
                            if last_seen.is_none() {
                                first_unseen == 0
                            } else {
                                last_seen.unwrap() + 1 == first_unseen
                            }
                        });
                    // println!("filtered2: {:?}", filtered2.clone().collect::<Vec<_>>());
                    if filtered2.clone().collect::<Vec<_>>().is_empty() {
                        return vec![(seen_groups.clone(), *n)];
                    }
                    filtered2
                        .map(move |(gears, unseen_groups, n2)| {
                            let n2 = n2 + if gears.iter().all(|gear| *gear == Gear::Unknown) {
                                1
                            } else {
                                0
                            };
                            let mut new_seen_groups = vec![
                                    seen_groups.clone(),
                                    unseen_groups.clone()
                                ].concat();
                            new_seen_groups.sort();
                            (new_seen_groups, n * n2)
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<HashMap<_,_>>();
            println!("new_acc: {:?}", new_acc);
            
            new_acc
        })
        .iter()
        .filter(|(seen, _)| groups.iter().all(|group| seen.contains(&group.id)))
        .map(|(_,n)| n)
        .sum()
}
fn valid_permutations2(gears: Vec<Gear>, groups: Vec<usize>) -> u128 {
    let mut gears_iter = gears.iter();
    let working_area: Vec<Gear> = gears_iter.by_ref()
        // .peeking_take_while(|gear| **gear != Gear::Operational)
        .cloned()
        .collect();


    if groups.is_empty() {
        if gears.iter().all(|gear| *gear == Gear::Unknown) {
            return 1;
        } else {
            return 0;
        }
    } else {
        if gears.is_empty() {
            return 0;
        }
    }
    
    let group_size = *groups.first().unwrap();
    if gears.len() < group_size {
        return 0;
    }

    // let gears_tail: Vec<Gear> = gears_iter
    //     .cloned()
    //     .collect();
    let groups_tail: Vec<usize> = groups.into_iter().skip(1).collect();

    let permutations: u128 = working_area
        .windows(group_size)
        .map(|window| window.into_iter().cloned().collect::<Vec<_>>())
        .enumerate()
        .map(|(i, window)| {
            let pre = working_area.iter()
                .take(i)
                .cloned().collect::<Vec<_>>();
            let window = window.to_vec();
            let tail = working_area.iter()
                .skip(i + group_size)
                .cloned().collect::<Vec<_>>();
            // let tail = vec![tail, gears_tail.clone()].concat();
            // if let Some(lookahead) = lookahead {
            //     tail = vec![tail, vec![lookahead]].concat();
            // }
            (pre, window, tail)
        })
        .filter_map(|(pre, _window, tail)|
            if pre.iter().any(|gear| *gear == Gear::Damaged) {None} else {Some(tail)}
        )
        .map(|tail| {
            if !groups_tail.is_empty() {
                if let Some(lookahead) = tail.first() {
                    if *lookahead == Gear::Operational || *lookahead == Gear::Unknown {
                        let tail: Vec<Gear> = tail.iter().cloned().skip(1).collect();
                        let n = valid_permutations2(tail, groups_tail.clone());
                        return n;
                    } else {
                        return 0;
                    }
                }
                return 0;
            } else {
                if let Some(lookahead) = tail.first() {
                    if *lookahead == Gear::Operational || *lookahead == Gear::Unknown {
                        return 1;
                    } else {
                        return 0;
                    }
                }
                return 1;
            }
        })
        .sum();

    return permutations;
}
// fn valid_permutations(gears: Vec<Gear>, groups: Vec<usize>) -> u128 {
//     let mut gears_iter = gears.iter();
//     let operational_gears = gears_iter.by_ref()
//         .peeking_take_while(|gear | **gear == Gear::Operational)
//         .count();
//     let working_area: Vec<Gear> = gears_iter.by_ref()
//         // .peeking_take_while(|gear| **gear != Gear::Operational)
//         .cloned()
//         .collect();
// 
//     // never happens due to how this recurses
//     if groups.is_empty() {
//         if gears.is_empty() || operational_gears > 0 {
//             return 1;
//         } else {
//             return 0;
//         }
//     }
//     
//     let group_size = *groups.first().unwrap();
//     if working_area.len() < group_size {
//         return 0;
//     }
// 
//     // let gears_tail: Vec<Gear> = gears_iter
//     //     .cloned()
//     //     .collect();
//     let groups_tail: Vec<usize> = groups.into_iter().skip(1).collect();
// 
//     let permutations: u128 = working_area
//         .windows(group_size)
//         .map(|window| window.into_iter().cloned().collect::<Vec<_>>())
//         .enumerate()
//         .map(|(i, window)| {
//             let pre = working_area.iter()
//                 .take(i)
//                 .cloned().collect::<Vec<_>>();
//             let window = window.to_vec();
//             let tail = working_area.iter()
//                 .skip(i + group_size)
//                 .cloned().collect::<Vec<_>>();
//             // let tail = vec![tail, gears_tail.clone()].concat();
//             // if let Some(lookahead) = lookahead {
//             //     tail = vec![tail, vec![lookahead]].concat();
//             // }
//             (pre, window, tail)
//         })
//         .filter_map(|(pre, window, tail)|
//             if pre.iter().any(|gear| *gear == Gear::Damaged) {None} else {Some((window, tail))}
//         )
//         .filter_map(|(window, tail)|
//             if window.iter().any(|gear| *gear == Gear::Operational) {None} else {Some(tail)}
//         )
//         .map(|tail| {
//             if !groups_tail.is_empty() {
//                 if let Some(lookahead) = tail.first() {
//                     if *lookahead == Gear::Operational || *lookahead == Gear::Unknown {
//                         let tail: Vec<Gear> = tail.iter().cloned().skip(1).collect();
//                         let n = valid_permutations(tail, groups_tail.clone());
//                         return n;
//                     } else {
//                         return 0;
//                     }
//                 }
//                 return 0;
//             } else {
//                 if let Some(lookahead) = tail.first() {
//                     if *lookahead == Gear::Operational || *lookahead == Gear::Unknown {
//                         return 1;
//                     } else {
//                         return 0;
//                     }
//                 }
//                 return 1;
//             }
//         })
//         .sum();
// 
//     return permutations;
// }

// fn valid_permutations(gears: Vec<Gear>, groups: Vec<isize>) -> usize {
//     let total_broken = groups.iter().sum::<isize>();
//     let known_broken = gears.iter()
//         .filter(|gear| **gear == Gear::Damaged)
//         .count() as isize;
// 
//     let groups: Vec<StateNum<_>> = groups.into_iter()
//         .map(|n| StateNum::Locked(n))
//         .collect();
//     let groups = match groups.as_slice() {
//         [StateNum::Locked(0), ..] =>
//             panic!("Found a group of size 0!"),
//         [StateNum::Locked(n), rest @ ..] =>
//             vec![vec![StateNum::Unlocked(*n)], rest.to_vec()].concat(),
//         _ =>
//             groups,
//     };
// 
//     let permutations = gears
//         .into_iter()
//         .fold(vec![(Vec::new(), groups, known_broken)], |acc, gear| {
//             acc.into_iter()
//                 .flat_map(|(permutation, groups, mut future_known_broken)| {
//                     let past_broken = permutation
//                         .iter()
//                         .filter(|gear| **gear == Gear::Damaged)
//                         .count() as isize;
//                     if total_broken - past_broken - future_known_broken < 0 {
//                         return vec![];
//                     }
// 
//                     let gear_branches = match gear {
//                         Gear::Operational => vec![
//                             gear,
//                         ],
//                         Gear::Damaged => {
//                             future_known_broken = future_known_broken - 1;
//                             vec![
//                                 gear,
//                             ]
//                         },
//                         Gear::Unknown => vec![
//                             Gear::Operational,
//                             Gear::Damaged,
//                         ],
//                     };
// 
//                     gear_branches.into_iter().filter_map(move |gear| {
//                         let permutation = permutation.clone();
//                         let groups = groups.clone();
//                         // let groups2 = groups.clone();
//                         let next_groups = match gear {
//                             Gear::Operational => 
//                                 match groups.as_slice() {
//                                     [StateNum::Unlocked(0), ..]
//                                     | [StateNum::Locked(0), ..] =>
//                                         panic!("Found a group of size 0!"),
//                                     [StateNum::Unlocked(n), rest @ ..]
//                                     | [StateNum::Locked(n), rest @ ..] =>
//                                         Ok(vec![vec![StateNum::Unlocked(*n)], rest.to_vec()].concat()),
//                                     [StateNum::Opened(_), ..] =>
//                                         Err("bad permutation: operational gear when opened"),
//                                     _ =>
//                                         Ok(groups),
//                                 },
//                             Gear::Damaged =>
//                                 match groups.as_slice() {
//                                     // [Open(0), rest @ ..] => todo!("bad permutation"),
//                                     // [Open(0)] => todo!("bad permutation"),
//                                     // [Open(0), Closed(n), rest @ ...] => todo!("bad permutation"),
//                                     [StateNum::Unlocked(0), ..]
//                                     | [StateNum::Opened(0), ..]
//                                     | [StateNum::Locked(0), ..] =>
//                                         panic!("Found a group of size 0!"),
//                                     [StateNum::Unlocked(1)]
//                                     | [StateNum::Opened(1)] =>
//                                         Ok(vec![]),
//                                     [StateNum::Unlocked(1), StateNum::Locked(n), rest @ ..]
//                                     | [StateNum::Opened(1), StateNum::Locked(n), rest @ ..] =>
//                                         Ok(vec![vec![StateNum::Locked(*n)], rest.to_vec()].concat()),
//                                     [StateNum::Unlocked(n), rest @ ..]
//                                     | [StateNum::Opened(n), rest @ ..] =>
//                                         Ok(vec![vec![StateNum::Opened(*n-1)], rest.to_vec()].concat()),
//                                     [StateNum::Locked(_), ..] =>
//                                         Err("bad permutation: damaged gear when locked"),
//                                     [] =>
//                                         Err("bad permutation: damaged gear when there are no groups left!"),
//                                     // _ =>
//                                     //     panic!("Unexpected group!"),
//                                 },
//                             Gear::Unknown =>
//                                 panic!("Encountered an attempt to branch into 'Gear::Unknown'!"),
//                         };
// 
//                         // ignore bad permutations, keep Ok() ones.
//                         if let Ok(next_groups) = next_groups {
//                             Some((
//                                 permutation.iter().cloned()
//                                     .chain(std::iter::once(gear))
//                                     .collect::<Vec<_>>(),
//                                     next_groups,
//                                     future_known_broken
//                             ))
//                         } else {
//                             None
//                         }
//                     })
//                     .collect::<Vec<_>>()
//                 })
//                 .collect::<Vec<_>>()
//             }
//         )
//         .into_iter()
//         .filter_map(|(perm, groups, _)| if groups.is_empty() {Some(perm)} else {None})
//         .collect::<Vec<_>>();
// 
//     // permutations.iter().for_each(|perm| { println!("{}", to_pretty_permutation_string(perm)); });
// 
//     permutations.len()
// }
// fn valid_permutations(gears: Vec<Gear>, groups: Vec<isize>) -> usize {
//     let broken = groups.iter().sum::<isize>() as usize;
//     let known_broken = gears.iter()
//         .filter(|gear| **gear == Gear::Damaged)
//         .count();
//     let unknown_broken = broken - known_broken;
// 
//     let groups_pattern = groups.iter()
//         .map(|n| format!("#{{{}}}", n))
//         .collect::<Vec<_>>()
//         .join("\\.+");
//     let pattern = format!(r"\.*{}\.*", groups_pattern);
//     let re = Regex::new(&pattern).unwrap();
//     
//     let permutations = gears
//         .into_iter()
//         .fold(vec![(Vec::new(), 0)], |acc, gear|
//             match gear {
//                 Gear::Operational | Gear::Damaged => {
//                     acc.into_iter()
//                         .map(|(permutation, n)| 
//                                 (permutation.iter().cloned()
//                                         .chain(std::iter::once(gear))
//                                         .collect::<Vec<_>>(), n)
//                             )
//                         .collect::<Vec<_>>()
//                 },
//                 Gear::Unknown => {
//                     acc.into_iter()
//                         .flat_map(|(permutation, n)| 
//                             if unknown_broken == n { // optimization
//                                 vec![
//                                     (
//                                         permutation.iter().cloned()
//                                             .chain(std::iter::once(Gear::Operational))
//                                             .collect::<Vec<_>>(),
//                                         n
//                                     ),
//                                 ]
//                             } else {
//                                 vec![
//                                     (
//                                         permutation.iter().cloned()
//                                             .chain(std::iter::once(Gear::Operational))
//                                             .collect::<Vec<_>>(),
//                                         n
//                                     ),
//                                     (
//                                         permutation.iter().cloned()
//                                             .chain(std::iter::once(Gear::Damaged))
//                                             .collect::<Vec<_>>(),
//                                         n+1
//                                     ),
//                                 ]
//                             })
//                         .collect::<Vec<_>>()
//                 },
//             }
//         )
//         .into_iter()
//         .map(|(perm, _)| perm)
//         .collect::<Vec<_>>();
// 
//     
//     let valids = permutations.iter()
//         .filter(|perm| {
//             let line = perm.iter()
//                 .cloned()
//                 .map(|gear| gear.to_string())
//                 .collect::<String>();
//             let valid = re.is_match(&line);
//             valid
//         });
// 
//     valids.count()
// }

fn part_2(input: &str) -> Result<isize, String> {
    let repetitions = 5;

    let solution = input
        .lines()
        .map(|s| s.trim())
        .map(|line| {
            let mut foo = line.split(' ');
            let gears = foo.next().unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Gear::Operational,
                    '#' => Gear::Damaged,
                    '?' => Gear::Unknown,
                    _ => panic!("Unexpected gear symbol!"),
                })
                .collect::<Vec<_>>();
            let gears_with_trailing = repeat(gears)
                .take(repetitions)
                .flat_map(|v|
                    v.into_iter().chain(std::iter::once(Gear::Unknown))
                )
                .collect::<Vec<_>>();
            let gears = gears_with_trailing.iter().cloned().take(gears_with_trailing.len()-1)
                .collect::<Vec<_>>();

            let groups = foo.next().unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let groups = repeat(groups)
                .take(repetitions)
                .flatten()
                .collect::<Vec<_>>();

                valid_permutations(gears, groups)
        })
        .sum::<u128>();

    return Ok(solution as isize);
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
        test_helper_1(r#"???.### 1,1,3"#, 1);
        test_helper_1(r#".??..??...?##. 1,1,3"#, 4);
        test_helper_1(r#"?#?#?#?#?#?#?#? 1,3,1,6"#, 1);
        test_helper_1(r#"????.#...#... 4,1,1"#, 1);
        test_helper_1(r#"????.######..#####. 1,6,5"#, 4);
        test_helper_1(r#"?###???????? 3,2,1"#, 10);
        test_helper_1(r#"???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1"#, 21);
    }

    fn test_helper_2(s : & str, v : isize)
    {
        assert_eq!(part_2(s).unwrap(), v, "input: {}", s);
    }

    #[test]
    fn test_2() {
        test_helper_2(r#"???.### 1,1,3"#, 1);
        test_helper_2(r#".??..??...?##. 1,1,3"#, 16384);
        test_helper_2(r#"?#?#?#?#?#?#?#? 1,3,1,6"#, 1);
        test_helper_2(r#"????.#...#... 4,1,1"#, 16);
        test_helper_2(r#"????.######..#####. 1,6,5"#, 2500);
        test_helper_2(r#"?###???????? 3,2,1"#, 506250);
        test_helper_2(r#"???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1"#, 525152);
    }
}


// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }