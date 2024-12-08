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

**Input parsing:** Returns the two lists separately as vectors of integers

**Part one solution:** Sort the lists, take the element wise absolute difference and sum it.

**Part two solution:** Build a histogram of the values on the second list, multiply the values on the first list by the value in the histogram.


## [Day 2](https://adventofcode.com/2024/day/2)

**Input parsing:** Vector of vector of integers

**Part one solution:** Check that each sequence is increasing or decreasing, respecting the bounds, by taking the difference between adjacent elements.

**Part two solution:** Surprisingly twisted. The straightforward and inefficient  way to solve it is to iteratively remove each value from a sequence and check if it is valid, but we can do much better, by identifying the index of the first sequence break and just remove the element at that index (and the successor), and check if the sequence is valid.

## [Day 3](https://adventofcode.com/2024/day/3)
Straightforward enough, given that the work is mostly done via regular expressions.

**Part one solution:** With a regular expression capture the relevant values between `mul()`.

**Part two solution:** For this we need an global regular expression to match `mul`, `don't` and `do`,  and a local one focused on the `mul()` to extract the inner values.


## [Day 4](https://adventofcode.com/2024/day/4)
Nice one, the twist on part two was quite clever.

**Input parsing:** 2D matrix of characters.

**Part one solution:** For each position check if the string "XMAS" can be "read" on each of the 8 possible directions.

**Part two solution:** For each position that is an "A", check its 4 corners to see if they contain opposite "S" and "M".


## [Day 5](https://adventofcode.com/2024/day/5)
Still straightforward, as long as one recognizes this as a sort problem. I assumed that the "rules" defined a [total order](https://en.wikipedia.org/wiki/Total_order), which is the case with the input.

**Input parsing:** Map of the rules, which define a total order on the numbers, and a vector of vectors of integers for the "orders". The map contains, for each value the possible successors (or values greater than it).

**Part one solution:** Assuming that the rules define a total order, one just needs to check that each position is less than the following one, which can be done directly on the map.

**Part two solution:** Similarly assuming a total order defined by the input map, this part is just a sort using that order.


## [Day 6](https://adventofcode.com/2024/day/6)
This was one of those days were i started solving the problem with the wrong solution, which took some time to recognize and correct. During part two, i recognized that there was a need to store the visited positions and the corresponding directions, which led me to redo the whole day. For efficiency, to store the visited positions/directions a 1D array of integers is used, where each position is treated as a bitmask of the different directions that went through there.

**Input parsing:** 2D matrix of characters.

**Part one solution:** Simple walk through the map, marking the visited positions and corresponding directions.

**Part two solution:** With the previous "walk through the map", place walls on the path, and repeat the walk, checking if a loop was found.


## [Day 7](https://adventofcode.com/2024/day/7)
Part two of this day is taking too long (about 600ms), so i must revisit this. The problem is that the number of possible combinations for the operations can be large, so i'm not seeing an easy way out

**Input parsing:** Vector of tuples, each with the total and a vector of the other numbers.

**Part one solution:** Place each operation on each of the available places, checking the total. The issue can be seen as looping through all numbers of base n that have p digits (n being the number of operations, p being the possible places for the operations), and for each digit apply a corresponding operation. Alternatively, generate the multi cartesian product of p iterators with n values and do the same as before.

**Part two solution:** Same as part one, with a base of 3.


## [Day 8](https://adventofcode.com/2024/day/8)
Actually, for a Sunday, i was expecting more trouble. This wasn't too hard.

**Input parsing:** Return the map as a 2d array of chars, and a hashmap with the locations for each antenna type.

**Part one solution:** For each antenna type, generate all possible pair combinations. Then, for each pair check if the 2 positions placed on the line defined by the pair are within the grid.

**Part two solution:** Similar to the previous one, except that we generate all positions defined by the pair line, which fit in the grid.
