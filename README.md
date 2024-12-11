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
My first solution was kind of complex and inefficient, simplifying it led to much better runtimes.

**Input parsing:** Vector of tuples, each with the total and a vector of the other numbers.

**Part one solution:** Loop through each position, iteratively applying the various operations. Keep two stacks: the current one with the totals up until that position, and the next one, generated from applying the different operations to each of the values in the current stack. Maybe a recursive solution would be more readable, but this works too.

**Part two solution:** Same as part one, increasing the number of operations to 3.


## [Day 8](https://adventofcode.com/2024/day/8)
Actually, for a Sunday, i was expecting more trouble. This wasn't too hard.

**Input parsing:** Return the map as a 2d array of chars, and a hashmap with the locations for each antenna type.

**Part one solution:** For each antenna type, generate all possible pair combinations. Then, for each pair check if the 2 positions placed on the line defined by the pair are within the grid.

**Part two solution:** Similar to the previous one, except that we generate all positions defined by the pair line, which fit in the grid.


## [Day 9](https://adventofcode.com/2024/day/9)
A quick and dirty solution, with lots of imaginary pointers to keep track of stuff. I chose to not only use the input representation, but also an explicit memory representation, possible because it's about 100k values.

**Input parsing:** The input as a vector of numbers and the corresponding memory representation, with the free spaces marked as -1.

**Part one solution:** On the memory representation, keep 2 pointers, one from the start and one from the end. Copy from the end to the start when the start is an empty space and the end is an occupied space, until they pass each other.

**Part two solution:** Trickier, as we need to use both representations, and an additional one to keep track of the moves that are made. Loop through the initial input representation from the end, trying to find space for each value on the moved input representation from the start. When enough space is found, update the moved input representation (updating the source and destination and their corresponding neighbors to account for the change in free space) as well as the memory representation.


## [Day 10](https://adventofcode.com/2024/day/10)
Very straightforward, a simple depth first search on the grid. I finally got tired of representing positions and directions as tuples, and created specific structures for them, simplifying the code.

**Input parsing:** The input as a grid of numbers and a vector with the locations of the trailheads start.

**Part one solution:** Do a depth first search from the starting points until a goal is reached. Store the unique goal location for part one and increase a counter of goal paths for part two.

**Part two solution:** Done in part one.


## [Day 11](https://adventofcode.com/2024/day/11)
Took me more time than i care to admit to recognize that it wasn't necessary to "blink" each value individually, that it was possible to keep a count of the different values and just evolve those counts...

**Input parsing:** An hashmap with the count of the different values that appear on the input.

**Part one solution:** Evolve each value according to the rules, creating a new Hashmap with the evolved values and the old counts.

**Part two solution:** Similar to part one.
