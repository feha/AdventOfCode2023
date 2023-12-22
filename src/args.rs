
use clap::{
    // Args,
    Parser,
    // Command,
    // Subcommand,
    // Args,
    // ValueEnum, Arg, ArgGroup,
};




#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct CLIAll {
    /// Optionally run all days. Mutually exclusive with providing specific days.
    #[clap(short, long)]
    pub all: bool,

    /// Optionally supply specific days to run. Defaults to latest (highest number). Mutually exclusive with --all.
    pub days: Vec<usize>,

    /// Optionally supply specific years to run. Defaults to latest (highest number).
    #[clap(short, long)]
    pub year: Option<usize>,
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct CLILatest {
    /// Optionally supply specific years to run. Defaults to latest (highest number).
    #[clap(short, long)]
    pub year: Option<usize>,
}




// #[derive(Debug, Parser)]
// #[command(author, version, about)]
// pub struct CLI {
//     #[command(subcommand)]
//     command: Option<Commands>,
// 
//     #[clap(short, long, parse(from_occurrences))]
//     year: u32,
// }
// 
// #[derive(Debug, Subcommand)]
// enum Commands {
//     /// does testing things
//     Create(CommandCreate),
//     Run(CommandRun),
// }
// 
// #[derive(Debug, Args)]
// pub struct CommandCreate {
//     all: bool, // not all and no day => latest
//     #[arg(value_parser = parse_entry)]
//     day: Vec<Entry>, // should be plural and allow ranges
// }
// 
// #[derive(Debug, Args)]
// pub struct CommandRun {
//     all: bool, // not all and no day => latest
//     #[arg(value_parser = parse_entry)]
//     days: Vec<Entry>, // should be plural and allow ranges
// }
// 
// // #[derive(Args)]
// // #[group(required = false, multiple = false)]
// #[derive(Debug, Args)]
// struct MutexArgs {
//     #[arg(exclusive(true), long = "latest")]
//     latest: bool, // true if nothing was given at all
// 
//     #[arg(exclusive(true), short, long)]
//     all: bool, // not all and no day => latest. -a is also accepted
// 
//     #[arg(exclusive(true), parse(try_from_str = parse_entry))]
//     days: Option<Vec<Entry>>, // should be plural and allow ranges
//     
// }
// 
// // #[derive(Debug, Parser)]
// // #[command(author, version, about)]
// // pub struct BuildArgs {
// //     /// Path to archive or directory with archives to combine. Multiple can be given.
// //     #[arg(
// //         short, long, value_delimiter=',',
// //         // default_values_t = [],
// //     )]
// //     pub days: Vec<Entry>,
// // }
// 




// // #[derive(Debug, Clone)]
// pub enum Entry {
//     Singular(u32),
//     Interval(u32, u32),
// }
// // 
// // fn parse_entry(s: &str) -> Result<Option<Vec<Entry>>, String> {
// //     return Ok(Some(vec![]));
// // }
// 
// 
// pub fn cli() -> Command {
//     Command::new("")
//         .about("abouttext")
//         .subcommand(
//             cli_helper(
//                 Command::new("create")
//                     .about("Create the files for targeted day(s)")
//                 )
//             )
//         .subcommand(
//             cli_helper(
//                 Command::new("run")
//                     .about("Run targeted day(s)")
//                 )
//             )
// }
// 
// fn cli_helper(cmd: Command) -> Command {
//     cmd.subcommand(
//         Command::new("all")
//             .about("day subcommand")
//             // Add subcommand-specific arguments here
//     )
//     .subcommand(
//         Command::new("latest")
//             .about("latest subcommand")
//             // Add subcommand-specific arguments here
//     )
//     .arg(
//         Arg::new("days")
//             .help("A positional argument")
//             // Add positional argument-specific configuration here
//     )
//     // .group(
//     //     ArgGroup::new("inputs")
//     //         .args(&["all", "latest", "days"])
//     //         .required(true)
//     // )
//     .arg(
//             Arg::new("year")
//                 .help("Specify which year, defaults to AOC_YEAR in .cargo/config.toml")
//         )
// }




// pub fn cli() -> Command {
//     Command::new("")
//         .about("abouttext")
//         .subcommand(
//             Command::new("create")
//                 .about("Create the files for targeted day(s)")
//                 .arg(
//                     Arg::new("mutex")
//                         .help("helptext")
//                 )
//             )
//         .subcommand(
//             Command::new("run")
//                 .about("Run targeted day(s)")
//                 .arg(
//                     Arg::new("mutex")
//                         .help("helptext")
//                 )
//                 .group(
//                     ArgGroup::new("inputs")
//                         .args(&["subcommand1", "subcommand2", "positional_arg"])
//                         .required(true)
//                 )
//             )
//         .arg(
//                 Arg::new("year")
//                     .help("Specify which year, defaults to AOC_YEAR in .cargo/config.toml")
//             )
// }
// 
// fn cli_helper(cmd: Command) -> Command {
//     cmd.subcommand(
//         Command::new("all")
//             .about("day subcommand")
//             // Add subcommand-specific arguments here
//     )
//     .subcommand(
//         Command::new("latest")
//             .about("latest subcommand")
//             // Add subcommand-specific arguments here
//     )
//     .arg(
//         Arg::new("days")
//             .help("A positional argument")
//             .required(true)
//             // Add positional argument-specific configuration here
//     )
//     .group(
//         ArgGroup::new("inputs")
//             .args(&["all", "latest", "days"])
//             .required(true)
//     )
// }