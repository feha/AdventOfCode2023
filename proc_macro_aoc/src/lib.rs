
#![feature(proc_macro_internals)]
#![feature(proc_macro_span)]
#![allow(unused_macros)]

use std::env;
use std::collections::HashSet;

use proc_macro::Span;
use proc_macro2::{TokenStream, Ident};

use syn;
use syn::parse::{Parse, ParseStream};

use quote::quote;

use glob::glob;
use regex::Regex;



// use std::env;
// 
// pub fn your_proc_macro() {
//     // Access the environment variable
//     if let Ok(value) = env::var("YOUR_ENV_VARIABLE") {
//         // Use the value in your proc-macro logic
//         println!("Environment variable value: {}", value);
//     } else {
//         // Handle the case when the environment variable is not set
//         eprintln!("Error: YOUR_ENV_VARIABLE is not set");
//     }
// }

macro_rules! year_dir_pattern {
    () => {
        "./src/years/_*"
    }
}
macro_rules! day_dir_pattern {
    () => {
        "./src/years/_{}/day*.rs"
    }
}
// "./src/years/_{}/day*.rs"
// "e:/workspaces/AdventOfCode/AdventOfCode2023/src/years/_{}/day*.rs"

const ENV_VAR_DAYS: &str = "AOC_SOLVE_DAYS";

const ENV_VAR_YEAR: &str = "AOC_YEAR";

const FIRST_YEAR: u32 = 2023;
// const FIRST_YEAR: u32 = 2015;

#[proc_macro]
pub fn days_list_env_var(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(quote! {String::from("#DAYS_LIST_ENV_VAR")})
}

fn parse_days() -> Option<HashSet<u32>>{
    if let Ok(value) = env::var(ENV_VAR_DAYS) {
        return Some(
            value
                .split(',')
                .map(|s|
                    s.parse()
                    .expect(
                        &format!("Invalid integer in {}", ENV_VAR_DAYS)
                    )
                ).collect()
        );

    }
    return None;
}
fn parse_year() -> Option<u32>{
    if let Ok(value) = env::var(ENV_VAR_YEAR) {
        return Some(
            value
                .parse()
                .expect(
                    &format!("Invalid integer in {}", ENV_VAR_YEAR)
                )
        );

    }
    return None;
}



// #[proc_macro]
// pub fn module_days(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let mut stream = TokenStream::new();
// 
//     let year = parse_year().unwrap_or(FIRST_YEAR);
//     
//     let year_dir = format!("_{}", year).parse::<TokenStream>().unwrap();
//     stream.extend(quote!{
//         pub mod #year_dir;
//     });
// 
//     return proc_macro::TokenStream::from(stream);
// }
// #[proc_macro]
// pub fn module_days2(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let mut stream = TokenStream::new();
// 
//     let year = parse_year().unwrap_or(FIRST_YEAR);
//     let days = parse_days();
// 
//     let re = Regex::new(r".+(\d+)").unwrap();
//     // let years_dirs = glob(format!(year_dir_pattern!(), year).as_str())
//     //     .expect("Failed to read pattern");
//     let files = glob(format!(day_dir_pattern!(), year).as_str())
//         .expect("Failed to read pattern");
// 
//     let mut block  = TokenStream::new();
//     for entry in files {
//         if let Ok(path) = entry {
//             let prefix = path.file_stem().unwrap().to_str().unwrap();
//             let caps = re.captures(prefix);
//             if let Some(caps) = caps {
//                 let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
// 
//                 // if days.is_none() || days.unwrap().contains(&n) {
//                 if days.as_ref().map_or(true, |hs: &HashSet<u32>| hs.contains(&n)) {
//                     let day = &format!("{}", prefix);
//                     // let day_padded = &format!("day{:0>2}", n);
// 
//                     // stream.extend(quote!{
//                     //     mod solver::years::_#year::#day;
//                     // });
//                     // stream.extend(format!("mod solver::years::_{}::{};", year, day).parse::<TokenStream>().unwrap());
//                     if n < 10 {
//                         block.extend(format!("pub mod {};", day).parse::<TokenStream>().unwrap());
//                     }
//                 }
//             }
//         }
//     }
//     stream.extend(quote!{
//         #block
//     });
// 
//     return proc_macro::TokenStream::from(stream);
// }
#[proc_macro]
pub fn module_days(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut stream = TokenStream::new();

    let year = parse_year().unwrap_or(FIRST_YEAR);
    let days = parse_days();

    let re = Regex::new(r".+?(\d+)").unwrap();
    // let years_dirs = glob(format!(year_dir_pattern!(), year).as_str())
    //     .expect("Failed to read pattern");
    let files = glob(format!(day_dir_pattern!(), year).as_str())
        .expect("Failed to read pattern");

    let mut block  = TokenStream::new();
    for entry in files {
        if let Ok(path) = entry {
            let prefix = path.file_stem().unwrap().to_str().unwrap();
            let caps = re.captures(prefix);
            if let Some(caps) = caps {
                let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();

                // if days.is_none() || days.unwrap().contains(&n) {
                if days.as_ref().map_or(true, |hs: &HashSet<u32>| hs.contains(&n)) {
                    let day = &format!("{}", prefix);
                    // let day_padded = &format!("day{:0>2}", n);

                    // stream.extend(quote!{
                    //     mod solver::years::_#year::#day;
                    // });
                    // stream.extend(format!("mod solver::years::_{}::{};", year, day).parse::<TokenStream>().unwrap());
                    
                    // if n < 10 {
                        block.extend(format!("pub mod {};", day).parse::<TokenStream>().unwrap());
                    // }
                }
            }
        }
    }
    
    let year_dir = format!("_{}", year).parse::<TokenStream>().unwrap();
    stream.extend(quote!{
        pub mod #year_dir {
            #block
        }
    });

    return proc_macro::TokenStream::from(stream);
}


#[proc_macro]
pub fn import_days(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut stream = TokenStream::new();
    stream.extend(format!("mod years;").parse::<TokenStream>().unwrap());

    let year = parse_year().unwrap_or(FIRST_YEAR);
    let days = parse_days();

    let re = Regex::new(r".+?(\d+)").unwrap();
    let files = glob(format!(day_dir_pattern!(), year).as_str())
        .expect("Failed to read pattern");
    for entry in files {
        if let Ok(path) = entry {
            let prefix = path.file_stem().unwrap().to_str().unwrap();
            let caps = re.captures(prefix);
            if let Some(caps) = caps {
                let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();

                // if days.is_none() || days.unwrap().contains(&n) {
                if days.as_ref().map_or(true, |hs: &HashSet<u32>| hs.contains(&n)) {
                    let day = &format!("{}", prefix);
                    let day_padded = &format!("day{:0>2}", n);

                    // stream.extend(quote!{
                    //     mod solver::years::_#year::#day;
                    // });
                    // stream.extend(format!("mod solver::years::_{}::{};", year, day).parse::<TokenStream>().unwrap());
                    // if n < 10 {
                        stream.extend(format!("use years::_{}::{} as {};", year, day, day_padded).parse::<TokenStream>().unwrap());
                    // }
                }
            }
        }
    }

    return proc_macro::TokenStream::from(stream);
}

// Same as import_days, except it only imports the file with highest n (aka 'latest' day)
#[proc_macro]
pub fn import_latest_day(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut stream = TokenStream::new();

    let year = parse_year().unwrap_or(FIRST_YEAR);

    let re = Regex::new(r".+?(\d+)").unwrap();
    let paths = glob(format!(day_dir_pattern!(), year).as_str())
            .expect("Failed to read pattern");
            // .collect::<Vec<_>>(); // Result<Vec<_>, _>
    
    let mut days = paths.filter_map(|res| {
            match res {
                Ok(path) => Some(path),
                Err(e) => {
                    println!("{:?}", e);
                    None
                },
            }
        })
        .filter_map(|path| {
            let prefix = path.file_stem().unwrap().to_str().unwrap().to_owned();
            let caps = re.captures(&prefix);
            if let Some(caps) = caps {
                let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
                return Some((n, prefix));
            }
            return None;
        })
        .collect::<Vec<_>>();
    days.sort_by(|(a, _), (b, _)| Ord::cmp(b, a));

    if days.is_empty() {
        return proc_macro::TokenStream::default();
    }
    // paths.
    if let Some((n, prefix)) = days.get(0) {
        let day = &format!("{}", prefix);
        let day_padded = &format!("day{:0>2}", n);

        // stream.extend(format!("mod {};", day).parse::<TokenStream>().unwrap());
        stream.extend(format!("mod years;").parse::<TokenStream>().unwrap());
        // if n < 10 {
            stream.extend(format!("use years::_{}::{} as {};", year, day, day_padded).parse::<TokenStream>().unwrap());
            // stream.extend(format!("use {} as {};", day, day_padded).parse::<TokenStream>().unwrap());
        // }
    }

    return proc_macro::TokenStream::from(stream);
}


#[proc_macro]
pub fn instantiate_days(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut stream = TokenStream::new();

    let year = parse_year().unwrap_or(FIRST_YEAR);
    let days = parse_days();

    let re = Regex::new(r".+?(\d+)").unwrap();

    let mut block  = TokenStream::new();
    let files = glob(format!(day_dir_pattern!(), year).as_str())
        .expect("Failed to read pattern");
    for entry in files {
        match entry {
            Ok(path) => {
                let prefix = path.file_stem().unwrap().to_str().unwrap();
                let caps = re.captures(prefix);
                if let Some(caps) = caps {
                    let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
                    if days.as_ref().map_or(true, |hs| hs.contains(&n)) {
                        let day_padded = &format!("day{:0>2}", n);
                        let day_padded_upper = &format!("Day{:0>2}", n);
                        let instance = &format!("({}, &{}::{} {{}})", n, day_padded, day_padded_upper)
                            .parse::<TokenStream>().unwrap();
                        // let instance = &format!("&{}::{} {{}}", day_padded, day_padded_upper).parse::<TokenStream>().unwrap();
                        block.extend(quote!{
                            v.push( #instance );
                        });
                    }
                }
                
            },
            Err(e) => println!("{:?}", e),
        }
    }
    stream.extend(quote!{
        {
            let mut v: Vec<(usize, &dyn Day)> = Vec::new();
            #block
            v
        }
    });

    return proc_macro::TokenStream::from(stream);
}

// Same as import_days, except only the file with highest n (aka 'latest' day)
#[proc_macro]
pub fn instantiate_latest_day(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut stream = TokenStream::new();

    let year = parse_year().unwrap_or(FIRST_YEAR);

    let re = Regex::new(r".+?(\d+)").unwrap();

    let mut block  = TokenStream::new();
    let paths = glob(format!(day_dir_pattern!(), year).as_str())
            .expect("Failed to read pattern");
            // .collect(); // : Result<Vec<_>, _> 
    
    let mut days = paths.filter_map(|res| {
            match res {
                Ok(path) => Some(path),
                Err(e) => {
                    println!("{:?}", e);
                    None
                },
            }
        })
        .filter_map(|path| {
            let prefix = path.file_stem().unwrap().to_str().unwrap().to_owned();
            let caps = re.captures(&prefix);
            if let Some(caps) = caps {
                let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
                return Some(n);
            }
            return None;
        })
        .collect::<Vec<_>>();
    days.sort_by(|a, b| Ord::cmp(b, a));
    
    if days.is_empty() {
        return proc_macro::TokenStream::default();
    }
    // paths.
    if let Some(n) = days.get(0) {
        let day_padded = &format!("day{:0>2}", n);
        let day_padded_upper = &format!("Day{:0>2}", n);
        let instance = &format!("({}, &{}::{} {{}})", n, day_padded, day_padded_upper).parse::<TokenStream>().unwrap();
        block.extend(quote!{
            v.push( #instance );
        });
    }

    // if let Ok(mut paths) = paths {
    //     paths.reverse();
    //     if paths.is_empty() {
    //         return proc_macro::TokenStream::default();
    //     }
    //     if let Some(path) = paths.get(0) {
    //         let prefix = path.file_stem().unwrap().to_str().unwrap();
    //         let caps = re.captures(prefix);
    //         if let Some(caps) = caps {
    //             let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
    //             let day_padded = &format!("day{:0>2}", n);
    //             let day_padded_upper = &format!("Day{:0>2}", n);
    //             let instance = &format!("({}, &{}::{} {{}})", n, day_padded, day_padded_upper).parse::<TokenStream>().unwrap();
    //             block.extend(quote!{
    //                 v.push( #instance );
    //             });
    //         }
    //     }
    // }

    stream.extend(quote!{
        {
            let mut v: Vec<(usize, &dyn Day)> = Vec::new();
            #block
            v
        }
    });

    return proc_macro::TokenStream::from(stream);
}


#[derive(Debug, Default)]
struct DayParser {
    parts: Vec<Ident>,
}
impl Parse for DayParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut day_parser = DayParser::default();

        while !input.is_empty() {
            let fn_ident = input.parse::<Ident>()?;
            // Optional, Ok vs Err doesn't matter. Just consume if it exists.
            input.parse::<syn::token::Comma>().ok();
            day_parser.parts.push(fn_ident);
        }

        return Ok(day_parser);
    }
}
// A macro designed to look like a regular function call rather than a TokenStream,
// aiming to replace the smallest possible amount of code for the use it's designed for.
// It only expands to a `struct Day#` implementing `Day`,
// effectively linking the passed functions to main.rs
// as main.rs has macros importing & instantiating Day# structs for each Day#.rs file.
//
// Example:
// fn part1(_input: &str) -> Result<isize, String> {
//     return Ok(0);
// }
// ...
// impl_day!( part1, ... );
#[proc_macro]
pub fn impl_day(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut stream = TokenStream::new();

    let span = Span::call_site();
    let binding = span.source_file().path();
    let file = binding.to_str().unwrap();
    if file == "" {
        let str = format!("Tried to implement Day for a non-existent call_site: span = \"{:?}\" , path = \"{:?}\"", span, binding);
        println!("{}", str);
        return proc_macro::TokenStream::from(stream);
    }

    let re = Regex::new(r".*day(\d+).rs").unwrap();
    let caps = re.captures(file);
    if let Some(caps) = caps {
        let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let day_padded_upper = format!("Day{:0>2}", n).parse::<TokenStream>().unwrap();

        let day_parser = syn::parse_macro_input!(input as DayParser);

        let mut trait_parts = TokenStream::new();
        for (k, fn_ident) in day_parser.parts.into_iter().enumerate() {
            let k = k+1;
            let trait_part_ident = format!("part_{}", k).parse::<TokenStream>().unwrap();
            // let trait_part_ident = proc_macro::Ident::new(format!("part_{}", k).as_str(), span);
            trait_parts.extend(quote!{
                fn #trait_part_ident(&self, input: &str) -> Result<String, ()> {
                    return Ok(format!("Part {}: {:?}", #k, #fn_ident(input)));
                }
            });
        }

        stream.extend(quote!{
            #[derive(Debug)]
            pub struct #day_padded_upper {}

            impl Day for #day_padded_upper {
                #trait_parts
            }
        });

    } else {
        // don't generate anything
        let str = format!("Tried to implement Day for a file with malformed name: file = \"{}\" , re = \"{:?}\"", file, re);
        println!("{}", str);
        // compile_error!(str); // can't figure out how to use these
    }

    return proc_macro::TokenStream::from(stream);
}
