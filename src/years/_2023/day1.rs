#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.

// mod utils; // imports utils.rs (not needed as its done in main.rs. not even possible for unknown reason)
use crate::utils::*; // needs to use crate, refers to crate populated by main.rs

// Creates struct for Day#, implementing Day trait.
// Thus links passed functions to main.rs (as it has macros instantiating Day# structs for each Day#.rs file).
use proc_macro_aoc;
proc_macro_aoc::impl_day!(part_1, part_2);

use std::collections::HashMap;


// Implementations
fn part_1(input: &str) -> Result<isize, String> {
    let digits: Vec<String> = vec![
        0,1,2,3,4,5,6,7,8,9,
    ].iter().map(|n| n.to_string()).collect();
    
    let solution = input
        .lines()
        .map(|s| s.chars().filter(|c| digits.contains(&c.to_string())).collect::<String>())
        .map(|s| format!("{}{}", s.chars().nth(0).unwrap(), s.chars().last().unwrap()))
        .map(|s| s.parse::<isize>().unwrap())
        // .for_each(|s| println!("{}",s));
        .fold(0, |sum, x| sum + x);

    println!("sol: {}",solution);
    return Ok(solution);
    // return Ok(1);
}

fn part_2(input: &str) -> Result<isize, String> {
    let digits_map = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        // ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let digits = digits_map.keys().map(|s| s.to_string()).collect::<Vec<String>>();
    
    let solution = input
        .lines()
        .map(|s| s.to_lowercase())
        .map(|s| {
            let mut res: String = "".to_string();
            let mut working_str: String = s.to_string();
            while !working_str.is_empty() {
                let start = digits.clone().into_iter()
                    .filter(|s2| working_str.starts_with(s2)).next();
                if let Some(start) = start {
                    let digit = digits_map.get(start.to_string().as_str()).unwrap().to_string();
                    res += &digit;
                    // working_str = working_str[start.len()..].to_string(); // drop substring
                // } else {
                //     working_str = working_str[1..].to_string(); // drop 1
                }
                working_str = working_str[1..].to_string(); // drop 1
            }
            return res;
        })
        .map(|s| format!("{}{}", s.chars().next().unwrap(), s.chars().last().unwrap()))
        .map(|s| s.parse::<isize>().unwrap())
        .fold(0, |sum, x| sum + x);
    return Ok(solution);
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
        test_helper_1(r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#
        , 142);
    }

    fn test_helper_2(s : & str, v : isize)
    {
        assert_eq! (part_2(s).unwrap(), v) ;
    }

    #[test]
    fn test_2() {
        test_helper_2("", 0);
        test_helper_2(r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#
        , 281);
    }
}


// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }



// fn part_1(input: &str) -> Result<isize, String> {
//     let solution = input
//         .lines()
//         .map(|s| s.parse::<isize>().unwrap())
//         .fold(0, |sum, x| sum + x);
// 
//     return Ok(solution);
// }
// 
// fn part_2(input: &str) -> Result<isize, String> {
//     let solution = input
//         .lines()
//         .map(|s| s.parse::<isize>().unwrap())
//         .fold(0, |sum, x| sum + x);
// 
//     return Ok(solution);
// }
// 
// 
// fn test_helper_1(s : & str, v : isize) {
//     assert_eq! (part_1(s).unwrap(), v) ;
// }
// fn test_helper_2(s : & str, v : isize) {
//     assert_eq! (part_2(s).unwrap(), v) ;
// }
// 
// fn test_1() {
//     assert_eq!("", "");
//     test_helper_1("", 0);
// }
// fn test_2() {
//     assert_eq!("", "");
//     test_helper_1("", 0);
// }
// 
// proc_macro_lib::impl_day_2!(
// 
// part 1 part_1
// test 1 test_1
// 
// part 2 part_2
// test 2 test_2
// 
// );

// Surprisingly, compiler errors in the "pseudo code" of this macro is expressed properly by vscode.
// Example is changing the type of part 1's function, highlighting it's return value and the tests,
// giving the proper error about incorrect type.
//
// proc_macro_lib::impl_day!(
// 
// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }
// 
// part 1
// (input: &str) -> isize {
//     // println!("{}", input);
//     let solution = input.lines()
//         .map(|s| s.parse::<isize>().unwrap())
//         .fold(0, |sum, x| sum + x );
//     
//     return Ok(solution);
// }
// 
// part 2
// (input: &str) -> isize {
//     let solution = input.lines()
//         .map(|s| s.parse::<isize>().unwrap())
//         .fold(0, |sum, x| sum + x );
//     
//     return Ok(solution);
// }
// 
// test 1
// assert("" , "")
// ("" , 0)
// 
// test 2
// ("" , 0)
// ("" , 0)
// 
// );


// Expands to:

// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }
// 
// 
// #[derive(Debug)]
// pub struct Day01 {}
// 
// impl Day for Day01 {
// 
//     fn part_1(&self, input: &str) -> Result<String, ()> {
//         return Ok(format!("Part {:?}: {:?}", 1, self.part_impl_1(input)));
//     }
// 
//     fn part_2(&self, input: &str) -> Result<String, ()> {
//         return Ok(format!("Part {:?}: {:?}", 2, self.part_impl_2(input)));
//     }
// 
// }
// 
// impl Day01 {
// 
//     fn part_impl_1(&self, input: &str) -> Result<isize, String> {
//         let solution = input
//             .lines()
//             .map(|s| s.parse::<isize>().unwrap())
//             .fold(0, |sum, x| sum + x);
// 
//         return Ok(solution);
//     }
// 
//     fn part_impl_2(&self, input: &str) -> Result<isize, String> {
//         let solution = input
//             .lines()
//             .map(|s| s.parse::<isize>().unwrap())
//             .fold(0, |sum, x| sum + x);
// 
//         return Ok(solution);
//     }
// 
// }
// 
// #[cfg(test)]
// mod tests
// {
//     use super :: * ;
// 
//     fn test_helper_1(s : & str, v : isize) {
//         assert_eq! (Day01 {}.part_impl_1(s).unwrap(), v) ;
//     }
// 
//     #[test]
//     fn test_1() {
//         assert_eq!("", "");
//         test_helper_1("", 0);
//     }
// 
//     fn test_helper_2(s : & str, v : isize)
//     {
//         assert_eq! (Day01 {}.part_impl_2(s).unwrap(), v) ;
//     }
// 
//     #[test]
//     fn test_2() {
//         test_helper_2("", 0);
//         test_helper_2("", 0);
//     }
// }




// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }

// #[derive(Debug)]
// pub struct Day01 {}

// impl Day for Day01
// {
//     fn part_1(& self, input : & str) -> Result<String, ()>
//     {
//         return Ok(format!("Part {:?}: {:?}", 1usize, self.part_impl_1(input)));
//     }
//     fn part_2(& self, input : & str) -> Result<String, ()>
//     {
//         return Ok(format!("Part {:?}: {:?}", 2usize, self.part_impl_2(input)));
//     }
// }

// impl Day01
// {
//     fn part_impl_1(& self, input : & str) -> Result<isize, String>
//     {
//         {
//             let solution =
//             input.lines().map(| s | s.parse :: < isize >
//             ().unwrap()).fold(0, | sum, x | sum + x) ; return Ok(solution) ;
//         }
//     } fn part_impl_2(& self, input : & str) -> Result < isize, String >
//     {
//         {
//             let solution =
//             input.lines().map(| s | s.parse :: < isize >
//             ().unwrap()).fold(0, | sum, x | sum + x) ; return Ok(solution) ;
//         }
//     }
// }
// #[cfg(test)]
// mod tests
// {
//     use super :: * ;
//     fn test_helper_1(s : & str, v : isize)
//     {
//         assert_eq! (Day01 {}.part_impl_1(s).unwrap(), v) ;
//     }

//     #[test]
//     fn test_1()
//     {
//         assert_eq! ("", "") ; test_helper_1("", 0) ;
//     }

//     fn test_helper_2(s : & str, v : isize)
//     {
//         assert_eq! (Day01 {}.part_impl_2(s).unwrap(), v) ;
//     }

//     #[test]
//     fn test_2()
//     {
//         test_helper_2("", 0) ; test_helper_2("", 0) ;
//     }
// }



// Using macro_rules!

// crate::day!{
//     Day01
//     
//     part1
//     |input: &str| -> isize {
//         let solution = input.lines()
//             .map(|s| s.parse::<isize>().unwrap())
//             .fold(0, |sum, x| sum + x );
//         
//         return Ok(solution);
//     }
//     
//     part2
//     |input: &str| -> isize {
//         let solution = input.lines()
//             .map(|s| s.parse::<isize>().unwrap())
//             .fold(0, |sum, x| sum + x );
//         
//         return Ok(solution);
//     }
//     
//     test1
//     ("" , "", false)
//     ("" , 0)
//     
//     test2
//     ("" , 0)
//     ("" , 0)
//     
// }
