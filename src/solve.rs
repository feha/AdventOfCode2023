#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.
// #[macro_use] // Allows usage of macros.

// use proc_macro_aoc;
use crate::utils::*;
use crate::args::{CLIAll, CLILatest};

// use std::env;
use clap::Parser;

pub fn run(days: Vec<(usize, &dyn Day)>, latest: bool) -> Result<(), ()> {
    // In terms of feature parity, CLIAll + CLILatest = CLIAll. No need for a third type.
    let mut args = CLIAll {
        all: false,
        days: vec![],
        year: None,
    };
    if latest {
        // args.all = false;
        args.all = true; // latest only imports + instances the latest day anyway, so can just as well run all.
        args.days = vec![]; // empty defaults to "latest"
        args.year = CLILatest::parse().year;
    } else {
        args = CLIAll::parse();
    }

    let all = args.all;
    let whitelist_days = args.days;
    let year = args.year.unwrap_or(get_year());
    
    for (i, (day, day_instance)) in days.iter().enumerate().rev() {
        let latest = whitelist_days.is_empty() && i == 0;
        let whitelisted = whitelist_days.contains(&day);
        if !all && !(latest || whitelisted) {
            continue; // Skip days not asked for
        }
        println!("= Solving Day: {:?} =", day);
        
        let input = get_input(year, *day);
        let input = input.trim();
        println!("= {} =", day);
        println!("{}", day_instance.part_1(input)?);
        println!("{}", day_instance.part_2(input)?);
        
        if !all {
            break; // only run specified day
        } 
    }

    // for (day, day_instance) in days.iter().rev() {
    //     let target = arg0.clone();
    //     if !all && target.is_some() && target.unwrap() != (day).to_string() {
    //         continue; // Skip days not asked for
    //     }
    //     
    //     let input = get_input(year, *day);
    //     let input = input.trim();
    //     println!("= {} =", day);
    //     println!("{}", day_instance.part_1(input)?);
    //     println!("{}", day_instance.part_2(input)?);
    //     
    //     if !all {
    //         break; // only run specified day
    //     } 
    // }
    
    return Ok(());
    //Err(1);
}


// fn add(a : i32, b : i32) -> i32 {
//   return a + b;
// }

// #[cfg(test)] // Only compiled with 'cargo'test' ('cargo bench' can't find it)
// #[test] // This function is a unit-test.
// fn hello_test() {
    //   assert_eq!(main(), Ok(()));
// }

// #[test]
// fn hello_test_add() {
    //   assert_eq!(add(1,2),3);
// }

// #[test]
// #[ignore] // ignore this test; doesn't run it, but still list (as ignored)
// fn hello_test_add_false() {
    //   assert_ne!(add(1,2),3);
// }