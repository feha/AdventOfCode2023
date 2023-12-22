#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.
// #[macro_use] // Allows usage of macros.

mod utils; // imports utils.rs (needed as its not in .toml)
mod args;
mod solve;

// use proc_macro_aoc;
proc_macro_aoc::import_latest_day!();
use utils::*;

fn main() -> Result<(), ()> {
    let days = proc_macro_aoc::instantiate_latest_day!();
    
    return solve::run(days, true);
    
    // return Ok(());
    //Err(1);
}