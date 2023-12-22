# AoC Rust Template #

A Rust project for AoC 2023, using a template framework to make adding new days easier.

Uses proc_macro's to remove the need for user to rename anything when starting a new day, with the exception of the filename when copying the prior day / day template-file.
The macros are kept in the `proc_macro_aoc` library.

## Results: ##
```lua
Day      Part1       Part2 
 12   01:40:48           -
 11   00:53:15    00:30:30
 10   01:59:59    05:25:05
  9   00:44:43    00:02:12
  8   00:37:39    00:34:32
  7   01:28:24    01:08:37
  6   00:18:56    00:03:24
  5   00:49:43    00:49:57
  4   00:14:01    00:34:15
  3   04:03:45    00:52:43
  2   00:29:46    00:15:31
  1   00:21:44    01:14:01
```

## Usage: ##

You can build, run & test with cargo in the terminal. It uses aliases to make some stuff less verboose:
* `cargo clean` / `cargo build` / `cargo run` / `cargo expand`
  * The standard cargo commands
* `cargo test --bin default day##`
  * Standard cargo command, used to test specific files
* `cargo test --bin latest day##`
  * Standard cargo command, used to test specific files, only works if said file is the latest day
* `cargo latest`
  * An alias to run the binary that only compiles latest day
* `cargo solve [\d+|--all]` / `cargo day [\d+|--all]`
  * Alias for running the binary that compiles all days. Takes the number corresponding to which day as an argument-
* `cargo all`
  * Alias for `cargo solve -a` (the "--all" argument). Runs all days.