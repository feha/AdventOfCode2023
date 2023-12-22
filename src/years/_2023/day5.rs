#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.

// mod utils; // imports utils.rs (not needed as its done in main.rs. not even possible for unknown reason)
use crate::utils::*; // needs to use crate, refers to crate populated by main.rs

use itertools::{Itertools, iterate};
use itertools::FoldWhile::{Continue, Done};
// Creates struct for Day#, implementing Day trait.
// Thus links passed functions to main.rs (as it has macros instantiating Day# structs for each Day#.rs file).
use proc_macro_aoc;
proc_macro_aoc::impl_day!(part_1, part_2);


// Implementations
fn part_1(input: &str) -> Result<isize, String> {
    let binding = input
        .lines()
        .map(|s| s.trim().to_owned())
        .collect::<Vec<_>>()
        .join("\n");
    let mut foo = binding
        .split("\n\n");
    
    let seeds = foo.next().unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let seed2soil = parse_map(foo.next().unwrap());
    let soil2fert = parse_map(foo.next().unwrap());
    let fert2water = parse_map(foo.next().unwrap());
    let water2light = parse_map(foo.next().unwrap());
    let light2temp = parse_map(foo.next().unwrap());
    let temp2humid = parse_map(foo.next().unwrap());
    let humid2loc = parse_map(foo.next().unwrap());

    let solution = seeds.into_iter()
        .map(|seed| index_map(&seed2soil, seed))
        .map(|soil| index_map(&soil2fert, soil))
        .map(|fert| index_map(&fert2water, fert))
        .map(|water| index_map(&water2light, water))
        .map(|light| index_map(&light2temp, light))
        .map(|temp| index_map(&temp2humid, temp))
        .map(|humid| index_map(&humid2loc, humid))
        .min().unwrap();

    return Ok(solution as isize);
}

fn part_2(input: &str) -> Result<isize, String> {
    let binding = input
        .lines()
        .map(|s| s.trim().to_owned())
        .collect::<Vec<_>>()
        .join("\n");
    let mut foo = binding
        .split("\n\n");

    let seeds = foo.next().unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| 
            match chunk {
                &[start, len] => (start, start + len - 1),
                _ => panic!("Unexpected chunk size"),
            }
        )
        // .map(|&[start, len]| (start, start + len - 1))
        .collect::<Vec<_>>();

    let soil2seed = parse_map_rev(foo.next().unwrap());
    let fert2soil = parse_map_rev(foo.next().unwrap());
    let water2fert = parse_map_rev(foo.next().unwrap());
    let light2water = parse_map_rev(foo.next().unwrap());
    let temp2light = parse_map_rev(foo.next().unwrap());
    let humid2temp = parse_map_rev(foo.next().unwrap());
    let loc2humid = parse_map_rev(foo.next().unwrap());

    // let locs = loc2humid.into_iter()
    //     .map(|Row { src, dest, len }| dest)
    //     .sorted();
    // let min_dest = locs.min();
    // let locs = chain![
    //     0..min_dest, // in case there is a seed that results in a lower loc than explicitly stated in map
    //     locs // values that exist explicitly in map
    // ];
    

    let maps = vec![
        soil2seed,
        fert2soil,
        water2fert,
        light2water,
        temp2light,
        humid2temp,
        loc2humid,
    ].into_iter().rev().collect::<Vec<_>>();
    // let loc2seed = |loc: usize| {
    //     maps.into_iter().fold(loc, |acc, map| index_map(&map, acc))
    // };
    fn map_pipeline(loc: usize, maps: &Vec<Vec<Row>>) -> usize {
        maps.into_iter().fold(loc, |acc, map| 
            {
                index_map(&map, acc)
            })
    }

    let solution = iterate(0, |&n| n+1)
        .fold_while(0, |_acc, loc| {
            // let seed = (&loc2seed)(loc);
            let seed = map_pipeline(loc, &maps);
            // if seed % 100000 == 0 { println!("loc -> seed: {} -> {}", loc, seed) }
            if seeds.iter().any(|(start, end)| *start <= seed && seed <= *end) {
                Done(loc)
            } else {
                Continue(0)
            }
        }).into_inner();

    return Ok(solution as isize);
}

#[derive(Debug)]
struct Row {
    src: usize,
    dest: usize,
    len: usize,
}
// fn parse_map(input: &str) -> HashMap<usize, usize> {
//     input.split('\n')
//         .skip(1)
//         .flat_map(|line| {
//             let args = line
//                 .split(' ')
//                 .map(|s| s.parse::<usize>().unwrap())
//                 .collect::<Vec<_>>();
//             let range = Row {
//                 src: args[1],
//                 dest: args[0],
//                 len: args[2],
//             };
//             let expanded_range = vec![(range.src, range.dest)]
//                 .repeat(range.len)
//                 .into_iter()
//                 .enumerate()
//                 .map(|(i, (src, dest))| ((src + i, dest+i)));
// 
//             expanded_range
//         })
//         .collect()
// }
// fn index_map(map: &HashMap<usize, usize>, key: usize) -> usize {
//     *map.get(&key).unwrap_or(&key)
// }
fn parse_map(input: &str) -> Vec<Row> {
    input.split('\n')
        .skip(1)
        .map(|line| {
            let args = line
                .split(' ')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            Row {
                src: args[1],
                dest: args[0],
                len: args[2],
            }
        })
        .collect()
}
fn index_map(map: &Vec<Row>, key: usize) -> usize {
    let in_range = map.into_iter()
        .skip_while(|Row {src, dest: _, len}| key < *src || (*src+*len-1) < key)
        .next();
    if let Some(Row {src, dest, len: _}) = in_range {
        let offset = key - *src;
        *dest + offset
    } else {
        key
    }
    // *map.get(&key).unwrap_or(&key)
}

fn parse_map_rev(input: &str) -> Vec<Row> {
    input.split('\n')
        .skip(1)
        .map(|line| {
            let args = line
                .split(' ')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            Row {
                src: args[0],
                dest: args[1],
                len: args[2],
            }
        })
        .collect()
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
        test_helper_1(r#"seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4"#, 35);
    }

    fn test_helper_2(s : & str, v : isize)
    {
        assert_eq! (part_2(s).unwrap(), v) ;
    }

    #[test]
    fn test_2() {
        test_helper_2(r#"seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4"#, 46);
    }
}


// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }