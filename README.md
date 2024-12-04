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

## [Day 2](https://adventofcode.com/2024/day/2)
Part one is easy: just check that the sequence is increasing or decreasing, respecting the bounds.
Part two was surprisingly twisted. The straightforward and inefficient  way to solve it is to iteratively remove each value from a sequence and check if it is valid, but we can do much better, by identifying the index of the first sequence break and just remove the element at that index (and the successor), and check if the sequence is valid.

## [Day 3](https://adventofcode.com/2024/day/3)
Straightforward enough, given that the work is mostly done via regular expressions.

## [Day 4](https://adventofcode.com/2024/day/4)
Transform the input into a 2d matrix of chars and for each position check if the search string is rooted there. On part one check for the string "XMAS" on each of the 8 possible directions, on part two, starting from the middle "A" check the corners to see if they contain the necessary "S" and "M".
The twist on part two was quite clever.

