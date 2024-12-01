# Advent of Code 2024 in Rust

My solutions for the [Advent of Code 2024](https://adventofcode.com/2024) in Rust.

As usual, there's very little or no error handling so the solutions blow up if the input files aren't exactly as expected and specified in each day's puzzle description.

Code is in `src/`, input files are in `data/`.

## Build
With [Cargo](https://doc.rust-lang.org/cargo/getting-started/index.html)


> `cargo build`


## Usage
To run, from the root dir:
> `./target/debug/aoc2024 $day [-t] [$file]`
>
> `cargo run -- $day [-t] [$file]`

This will run the 2 parts of the specified `$day`, using `$file` as input. If no file is specified `./data/input{$day}` is used as input. If no file is specified and `-t` is used, `./data/input{$day}Test` is used instead. 

## [Day 1](https://adventofcode.com/2024/day/1)
And we're off. The borrow checker put up some fight, but that's the price of learning. Somehow the solution doesn't look very idiomatic, i'll have to take a look at some others to get the hang of it.
