#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.

use std::iter::repeat;
use regex::Regex;

// mod utils; // imports utils.rs (not needed as its done in main.rs. not even possible for unknown reason)
use crate::utils::*; // needs to use crate, refers to crate populated by main.rs

use either::Either::{Left, Right};
// Creates struct for Day#, implementing Day trait.
// Thus links passed functions to main.rs (as it has macros instantiating Day# structs for each Day#.rs file).
use proc_macro_aoc;
proc_macro_aoc::impl_day!(part_1, part_2);


// Implementations
fn part_1(input: &str) -> Result<isize, String> {
    // let pad_outer: Vec<Option<Vec<_>>> = vec![None];
    // let pad_inner: Vec<Either<usize, Option<char>>> = vec![Right(None)];
//     let solution = pad_lines.into_iter()
//     .chain(
//         input.lines()
//     )
//     .chain(pad_lines)
//         .map(|s|
//             pad_inner.clone().into_iter()
//                 .chain(
//                     s.split(r"[^\d]")
//                         .flat_map(|s2| {
//                             if let Ok(n) = s2.parse::<usize>() {
//                                 // let index: usize = i;
//                                 // lookup.insert(index, n);
//                                 // i+=1;
//                                 // repeat(Left(index))
//                                 //     .take(s2.len())
//                                 //     .collect::<Vec<_>>()
//                                 //     .into_iter()
//                                 repeat(Left(n))
//                                     .take(s2.len())
//                                     .collect::<Vec<_>>()
//                                     .into_iter()
//                             } else {
//                                 s2.chars()
//                                     .map(|c| Right(Some(c)))
//                                     .collect::<Vec<_>>()
//                                     .into_iter()
//                             }
//                         })
//                         .collect::<Vec<_>>()
//                         .into_iter()
//             )
//             .chain(
//                 pad_inner.clone().into_iter()
//             )
//             .collect::<Vec<_>>()
//             .windows(3)
//             .map(|window| window.into_iter().collect::<Vec<_>>())
//             .collect::<Vec<_>>()
//         )
//         .collect::<Vec<_>>()
//         .into_iter()
//         .map(Some)
//     )
//     .collect::<Vec<_>>()
//     .into_iter()
//     .chain(pad_outer.into_iter())
//     .collect::<Vec<_>>()
//     .windows(3)
//     .map(|window| window.into_iter().collect::<Vec<_>>())
//     .collect::<Vec<_>>()
//     .into_iter()
//     .fold(HashMap::new(),|acc, window_vertical| {
//         let line_above = *window_vertical[0];
//         let line_center = window_vertical[1].unwrap();
//         let line_below = *window_vertical[2];
// 
//         let empty_line: Vec<Option<Vec<_>>> = repeat(None).take(line_center.len()).collect();
//         let line_above = line_above.map_or_else(
//                 || empty_line, 
//                 |windows_hor| windows_hor.into_iter().map(Some).collect()
//             );
//         let line_below = line_below.map_or_else(
//                 || empty_line, 
//                 |windows_hor| windows_hor.into_iter().map(Some).collect()
//             );
// 
//         let zipped = line_above.into_iter()
//             .zip(line_below)
//             .zip(line_center);
// 
//         zipped.into_iter().for_each(|((w_hor_above, w_hor_below), w_hor)| {
//             // let top = w_hor_above
//             //     .map(|windows| windows.into_iter()
//             //         .map(|window|
//             //             window.into_iter()
//             //                 .filter(|cell| cell.is_left() || cell.right().is_some())
//             //                 .map(|cell| cell.map_right(|opt| opt.unwrap()))
//             //                 .collect::<Vec<_>>()
//             //         )
//             //         .collect::<Vec<_>>()
//             //     );
//             // let center = w_hor.into_iter()
//             //     .map(|window|
//             //         window.into_iter()
//             //             .filter(|cell| cell.is_left() || cell.right().is_some())
//             //             .map(|cell| cell.map_right(|opt| opt.unwrap()))
//             //             .collect::<Vec<_>>()
//             //     )
//             //     .collect::<Vec<_>>();
//             // let bot = w_hor_below
//             //     .map(|windows| windows.into_iter()
//             //         .map(|window|
//             //             window.into_iter()
//             //                 .filter(|cell| cell.is_left() || cell.right().is_some())
//             //                 .map(|cell| cell.map_right(|opt| opt.unwrap()))
//             //                 .collect::<Vec<_>>()
//             //         )
//             //         .collect::<Vec<_>>()
//             //     );
//             
//             if let Left(n) = *w_hor.remove(1) {
//                 let is_part = vec![w_hor_above, Some(w_hor), w_hor_below].into_iter()
//                     // .filter(|opt| opt.is_some())
//                     // .map(|opt| opt.unwrap())
//                     .flatten()
//                     .flatten()
//                     .fold(false, |acc, cell| if cell.is_left() { acc } else {acc || cell.right().unwrap() != Some('.')} );
//                     // .collect::<Vec<_>>();
// 
//                 let entry = acc.entry(n);
//                 entry.or_default();
//                 entry.and_modify(|v| *v = *v || is_part);
//             }
//         });
//         
// 
//         acc
//     })
//     .into_iter()
//     .fold(0, |acc, (k, b)| acc + if b { k } else { 0 } );
//     // .fold(0, |sum, x| sum + x);

    let pad_lines = ".".repeat(input.lines().count());
    let pad_line = ".".to_owned();
    let padded_input = pad_lines.clone() + "\n" + input + "\n" + &pad_lines;
    let padded_lines = padded_input.lines()
        .map(|line| pad_line.clone() + line + &pad_line);

    let pattern = Regex::new(r"(\D+|\d+)").unwrap();

    let mut part_lookup: Vec<usize> = Vec::new();
    let graph = padded_lines.clone()
        .map(|line| {
            pattern.captures_iter(&line).flat_map(|capture| {
                    let s = capture.get(0).unwrap().as_str();
                    if let Ok(part_n) = s.parse::<usize>() {
                        let index: usize = part_lookup.len();
                        part_lookup.push(part_n);
                        repeat(Left::<usize, char>(index))
                            .take(s.len())
                            .collect::<Vec<_>>()
                    } else {
                        s.chars()
                            .map(|c| Right::<usize, char>(c))
                            .collect::<Vec<_>>()
                    }
                })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let graph_row_windows = graph.into_iter()
        .map(|row| 
            row
                .windows(3)
                .map(|window| window.into_iter().map(|w| *w).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    let graph_windows = graph_row_windows
        .windows(3)
        .map(|window| window.into_iter().map(|w| w.to_owned()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let lookup = graph_windows
        .into_iter()
        .fold(part_lookup.iter().map(|_| false).collect::<Vec<bool>>(), |mut acc, window_vertical| {
            let line_above = window_vertical[0].clone();
            let line_center = window_vertical[1].clone();
            let line_below = window_vertical[2].clone();

            let zipped = line_above.into_iter()
                .zip(line_below)
                .zip(line_center);

            zipped.into_iter().for_each(|((w_hor_above, w_hor_below), mut w_hor)| {
                if let Left(part_id) = w_hor.remove(1) {
                    let neighbours = vec![w_hor_above, w_hor, w_hor_below]
                        .into_iter()
                        .flatten();
                    let is_part = neighbours
                        .fold(false, |acc, cell| if cell.is_left() { acc } else {acc || cell.right().unwrap() != '.'} );

                    acc[part_id] = acc[part_id] || is_part;
                }
            });
            

            acc
        }).into_iter().enumerate().collect::<Vec<_>>();

    let solution = lookup
        .into_iter()
        .fold(0, |acc, (part_id, is_part)| acc + if is_part { part_lookup[part_id] } else { 0 } );
        // .fold(0, |sum, x| sum + x);

    return Ok(solution as isize);
}

fn part_2(input: &str) -> Result<isize, String> {

    let pad_lines = ".".repeat(input.lines().count());
    let pad_line = ".".to_owned();
    let padded_input = pad_lines.clone() + "\n" + input + "\n" + &pad_lines;
    let padded_lines = padded_input.lines()
        .map(|line| pad_line.clone() + line + &pad_line);

    let pattern = Regex::new(r"(\D+|\d+)").unwrap();

    let mut part_lookup: Vec<usize> = Vec::new();
    let graph = padded_lines.clone()
        .map(|line| {
            pattern.captures_iter(&line).flat_map(|capture| {
                    let s = capture.get(0).unwrap().as_str();
                    if let Ok(part_n) = s.parse::<usize>() {
                        let index: usize = part_lookup.len();
                        part_lookup.push(part_n);
                        repeat(Left::<usize, char>(index))
                            .take(s.len())
                            .collect::<Vec<_>>()
                    } else {
                        s.chars()
                            .map(|c| Right::<usize, char>(c))
                            .collect::<Vec<_>>()
                    }
                })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let graph_row_windows = graph.clone().into_iter()
        .map(|row| 
            row
                .windows(3)
                .map(|window| window.into_iter().map(|w| *w).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    let graph_windows = graph_row_windows
        .windows(3)
        .map(|window| window.into_iter().map(|w| w.to_owned()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let lookup = graph_windows
        .into_iter()
        .fold(part_lookup.iter().map(|_| false).collect::<Vec<bool>>(), |mut acc, window_vertical| {
            let line_above = window_vertical[0].clone();
            let line_center = window_vertical[1].clone();
            let line_below = window_vertical[2].clone();

            let zipped = line_above.into_iter()
                .zip(line_below)
                .zip(line_center);

            zipped.into_iter().for_each(|((w_hor_above, w_hor_below), mut w_hor)| {
                if let Left(part_id) = w_hor.remove(1) {
                    let neighbours = vec![w_hor_above, w_hor, w_hor_below]
                        .into_iter()
                        .flatten();
                    let is_part = neighbours
                        .fold(false, |acc, cell| if cell.is_left() { acc } else {acc || cell.right().unwrap() != '.'} );
    
                    acc[part_id] = acc[part_id] || is_part;
                }
            });
            

            acc
        }).into_iter().enumerate().collect::<Vec<_>>();
    
    let graph_filtered = graph.clone().into_iter()
        .map(|row| row.into_iter()
            .map(|cell| 
                if let Left(part_id) = cell {
                    if lookup[part_id].1 {
                        cell
                    } else {
                        Right('.') // filter out non-part numbers
                    }
                } else {
                    cell
                }
            )
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    let graph_row_windows = graph_filtered.clone().into_iter()
        .map(|row| 
            row
                .windows(3)
                .map(|window| window.into_iter().map(|w| *w).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    let graph_windows = graph_row_windows
        .windows(3)
        .map(|window| window.into_iter().map(|w| w.to_owned()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let gears: Vec<(usize, (usize, usize))> = graph_windows.clone()
        .into_iter()
        .fold(Vec::new(), |mut acc, window_vertical| {
            let line_above = window_vertical[0].clone();
            let line_center = window_vertical[1].clone();
            let line_below = window_vertical[2].clone();

            let zipped = line_above.into_iter()
                .zip(line_below)
                .zip(line_center);

            zipped.into_iter().for_each(|((w_hor_above, w_hor_below), mut w_hor)| {
                if let Right(c) = w_hor.remove(1) {
                    if c == '*' {
                        let neighbours = vec![w_hor_above.clone(), w_hor.clone(), w_hor_below.clone()]
                            .into_iter()
                            .flatten();

                        let mut pair = vec![];
                        neighbours
                            .for_each(|cell| {
                                if let Left(part_id) = cell {
                                    if !pair.contains(&part_id) {
                                        pair.push(part_id);
                                    }
                                }
                            });

                        if pair.len() == 2{
                            acc.push((pair[0], pair[1]));
                        }
                    }
                }
            });
            
            acc
        }).into_iter().enumerate().collect::<Vec<_>>();

    // graph.iter().for_each(|row|
    //     {
    //         let line = row.iter().map(|cell| format!("{}", cell)).join(" ");
    //         println!("   {:?}", line)
    //     });
    // println!("----");
    // graph_filtered.iter().for_each(|row|
    //     {
    //         let line = row.iter().map(|cell| format!("{}", cell)).join(" ");
    //         println!("   {:?}", line)
    //     });
    // println!("{:?}", part_lookup);
    // println!("{:?}", lookup);
    // println!("{:?}", gears);

    let solution: usize = gears
        .into_iter()
        .map(|(_i, (part_id1, part_id2))| part_lookup[part_id1] * part_lookup[part_id2])
        .sum();
        // .fold(0, |sum, x| sum + x);

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
        test_helper_1(r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#, 4361);
    }

    fn test_helper_2(s : & str, v : isize)
    {
        assert_eq! (part_2(s).unwrap(), v) ;
    }

    #[test]
    fn test_2() {
        test_helper_2(r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#, 467835);
    }
}


// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }