# Advent of Code 2024 in Rust

My solutions for the [Advent of Code 2024](https://adventofcode.com/2024) in Rust.

As usual, there's very little or no error handling so the solutions panic if the input files aren't exactly as expected and specified in each day's puzzle description.

Code is in `src/`, input files are in `data/`.

## Build and Test

With [Cargo](https://doc.rust-lang.org/cargo/getting-started/index.html)

> `cargo build`
>
> `cargo test`

## Usage

To run, from the root dir:
> `./target/debug/aoc2024 $day [-t] [$file]`
>
> `cargo run -- $day [-t] [$file]`

This will run the 2 parts of the specified `$day`, using `$file` as input. If no file is specified `./data/input{$day}` is used as input. If no file is specified and `-t` is used, `./data/input{$day}Test` is used instead.

## Post-Event Impressions

The following are approximate runtimes of each puzzle on a i5-11400, in ms:

|    Day    |    Part 1    |    Part 2    |
|:---------:|:------------:|:------------:|
|     1     |     0.11     |     0.12     |
|     2     |     0.19     |     0.24     |
|     3     |     0.39     |     0.49     |
|     4     |     0.40     |     0.27     |
|     5     |     0.22     |     0.40     |
|     6     |     0.09     |    27.40     |
|     7     |     1.01     |    21.60     |
|     8     |     0.05     |     0.09     |
|     9     |     0.59     |    16.90     |
|    10     |     0.19     |     0.17     |
|    11     |     0.17     |     6.77     |
|    12     |     0.99     |     0.94     |
| **Total** | **34.39 ms** | **89.56 ms** |

## [Day 1](https://adventofcode.com/2024/day/1)

And we're off. The borrow checker put up some fight, but that's the price of learning. Somehow the solution doesn't look very idiomatic, i'll have to take a look at some others to get the hang of it.

**Input parsing:** Returns the two lists separately as vectors of integers

**Part one solution:** Sort the lists, take the element wise absolute difference and sum it.

**Part two solution:** Build a histogram of the values on the second list, multiply the values on the first list by the value in the histogram.

## [Day 2](https://adventofcode.com/2024/day/2)

Part two was surprisingly twisted, so early in the game.

**Input parsing:** Vector of vector of integers

**Part one solution:** Check that each sequence is increasing or decreasing, respecting the bounds, by taking the difference between adjacent elements.

**Part two solution:** The straightforward and inefficient way to solve it is to iteratively remove each value from a sequence and check if it is valid, but we can do much better, by identifying the index of the first sequence break and just remove the element at that index (and the successor), and check if the sequence is valid.

## [Day 3](https://adventofcode.com/2024/day/3)

Straightforward enough, given that the work is mostly done via regular expressions.

**Part one solution:** With a regular expression capture the relevant values between `mul()`.

**Part two solution:** For this we need a global regular expression to match `mul`, `don't` and `do`, and a local one focused on the `mul()` to extract the inner values.

## [Day 4](https://adventofcode.com/2024/day/4)

Nice one, the twist on part two was quite clever.

**Input parsing:** 2D matrix of characters.

**Part one solution:** For each position check if the string "XMAS" can be "read" on each of the 8 possible directions.

**Part two solution:** For each position that is an "A", check its 4 corners to see if they contain opposite "S" and " M".

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

**Part two solution:** Trickier, as we need to use both representations, and an additional one to keep track of the moves that are made. Loop through the initial input representation from the end, trying to find space for each value on the moved input representation from the start. When enough space is found, update the moved input representation ( updating the source and destination and their corresponding neighbors to account for the change in free space) as well as the memory representation.

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

## [Day 12](https://adventofcode.com/2024/day/12)

Getting part two took right some time, but in the end it's similar to part one, just adjusting the calculation to subtract the common sides for the neighbors.

**Input parsing:** Just separate the chars.

**Part one solution:** For each position of the grid that hasn't been explored do a flood calculating the area and perimeter of the region. The perimeter is calculated by storing, for each position a bitmask where a bit is set for each direction that is "outside" the region.

**Part two solution:** Similar to part one, but to correctly calculate the perimeter, subtract the common sides that are outside for each of the neighbors.

## [Day 13](https://adventofcode.com/2024/day/13)

It's a simple system of linear equations, directly solver through linear algebra. With numpy this would be a lot faster, as is, and because i don't want to use external crates, i implemented the necessary operations to calculate determinant, dot product and inverse 2x2 matrices.

**Input parsing:** For each "machine" returns the prize vector and the corresponding matrix.

**Part one solution:** Solve aX=b => a = dot(b, inv(X)), where b is the prize vector and X the other matrix. Afterwards, only consider solutions that are integers.

**Part two solution:** Same as part one.

## [Day 14](https://adventofcode.com/2024/day/14)

Interesting day. Part two was underspecified, which let me somehow unsure on how to proceed. Manually visualizing the grid evolution to spot a tree sounded like a dead end, but there was no specific conditions to identify the tree. At first i thought that it could be assumed to be a full tree, so all robots would be together, but then i noticed the " most robots" on the description which invalidated that. Nevertheless, to form a tree, most of the robots would have to be close together... positions close together can be efficiently identified by finding the minimum variance on each axis, which sounds like a plan. There still was the question of how many steps to consider to identify the minimum variance but, noticing that this is modular arithmetic after all, there's a repetition period, which can be at most the width/height of the grid. So assuming that the period on each axis is at most its length, one can identify the time step with the minimum variance in that period, then use the chinese remainder theorem to calculate when the two axis will both have minimum variance.

**Input parsing:** For each line, its corresponding position and direction.

**Part one solution:** Straightforward modular arithmetic to calculate the positions of the robots.

**Part two solution:** Explained before. To calculate the minimum chinese theorem, i took the easy way out, doing a systematic search on the possible time steps instead of searching by seaving (which i did in AoC 2020).

## [Day 15](https://adventofcode.com/2024/day/15)

Fun day. Theoretically it's straightforward, but in practice, part two took some time to get all the kinks worked out.

**Input parsing:** Returns the grid and the list of directions.

**Part one solution:** For each direction and from the current position, walk the grid until an empty space is found. If one is found, walk back to the starting position, moving each cell.

**Part two solution:** The strategy is to get all the connected cells, starting from a position and in a given direction. If it's not possible to move (found a wall), return `None`. Given that, iterate through the directions, moving all connected cells. To move, and for performance reasons, use an auxiliary scratchpad, where the contents of the cells to be moved are written, then clear those cells from the grid and copy from the scratchpad to the new positions.

## [Day 16](https://adventofcode.com/2024/day/16)

Yet another Dijkstra search. This one is almost identical to last year's day 17, so i just copied the solution, translating it from C++ to Rust. Couldn't use a bucket queue because the indices would be sparsely populated, so i used the traditional priority queue.

Furthermore, had to use Rc for part two, to keep track of the followed path, which led to some struggle with Rust...

**Input parsing:** The grid.

**Part one solution:** Dijkstra search.

**Part two solution:** Dijkstra search, keeping all the paths that lead to the goal, and returning unique positions.

## [Day 17](https://adventofcode.com/2024/day/17)

I have a love/hate relationship with these kinds of problems. On one hand i like writing interpreters, on the other i hate interpreting low level code, but when i "see" it, it's a good feeling... Anyway one more done, with a lot of wasted time.

**Input parsing:** The register contents and the program.

**Part one solution:** Simple interpreter, not much to say.

**Part two solution:** I initially tried to decode the instructions and rewrite them in Rust, but after a minute or so of runtime, i realized this wasn't the way. Looking at the program, and after a long time, i noticed the following regarding the instructions:

- Each loop outputs a digit;
- It only depends on the A register value;
- At the end of each loop A is divided by 8 (shifted 3 bits to the right), so at the beginning of the loop A can be at most in a range of [end..end+8[;
- After generating the last digit, A must be 0;

Therefore, the solution can be constructed recursively, working backwards from the last digit to the first:

- Test each of the values [A..A+8[, to see which one outputs the pretended digit at that position. Save that possible solution, and recurse to the previous digit;
- There can be more than one solution at each step, and that each one must be kept and carried to try to generate the previous digit. In the end the minimum is calculated and returned.

## [Day 18](https://adventofcode.com/2024/day/18)

Easy enough, a straightforward BFS. Reading the description i got the impression that, in part two, the order of the " falling" bytes would be relevant, so i chose to store in each grid position not a simple occupied/free indicator but the time index that that position becomes occupied. This come indeed handy in part two, particularly for performance reasons, because there's no need to regenerate the grid in each step.

**Input parsing:** The positions.

**Part one solution:** Simple BFS, only considering the positions that are under the time limit.

**Part two solution:** With the grid structure, this is also relatively straightforward: define a time limit and verify if there's a path using the BFS. To find the time limit i used a simple binary search, given that iteratively searching for it was taking about 1s, and with a binary search it takes less than 1ms.

## [Day 19](https://adventofcode.com/2024/day/19)

Nice twist on part two. Part one was relatively straightforward, I chose to just keep track of the reached indexes on the design, and not all the possible pattern combinations to do that, which helped with performance. Looking at part two, i though that just by changing 3 or 4 lines of the solution i had would be enough, which indeed was, but only with the test input... On the real input it didn't seem to want to end, so i had to get back to the drawing board and create a different solution.

**Input parsing:** The patterns present on the first line and the designs on the other lines, both as vector of vector of chars.

**Part one solution:** For each design, keep a list of the reached indexes up until that point, and try to fit each of the patterns from that index on. If the pattern fits and the end of the design is reached, break and increase the solution counter, otherwise, update the list of reached indexes with the pattern length.

**Part two solution:** For each design keep a count of the number of paths that have reached each of the design's indexes. Iterate through the design's length, trying to fit each pattern, and updating the reached indexes count. The solution is the count on the last position.

## [Day 20](https://adventofcode.com/2024/day/20)

Getting to a solution was easy and quick enough. Build a path on the maze, saving the corresponding positions. Then iterate through the combinations of those positions and for each pair check if their manhattan distance is less than the max cheat distance, and if the saved time (which is tha path length between the positions minus their manhattan distance) is bigger than what is intended.

This was straightforward but took tens of milliseconds to run, so i looked for some optimizations. For now, what i come up with is build a second grid with the path marked.

Need revisiting

**Input parsing:** The grid.

**Part one solution:** Described earlier.

**Part two solution:** Described earlier.

## [Day 21](https://adventofcode.com/2024/day/21)

A very interesting and hard day. For part one i built a kind of keypad simulator that, given a map with the keys positions, a starting position and a desired character to output, recursively calculates and returns the moves necessary to move from the start position to the position of the specified character. This was generic and worked for both keypads, solving part one, though it took some time to execute.

For part two that strategy wasn't going to work. The numeric keypad moves could still be obtained by the previous simulator, but the directional keypad moves, at the desired depth level, leads to a combinatorial explosion.

The first insight is that the directional keypad is simple enough to enable hardcoding the possible moves from each key to another, and that map can be used to expand a "complex" move by iterating over each simple move (from one key to another). There remains the issue that there's a kind of 2-dimensional expansion on each depth level transition: one where each simple move is expanded via the map, and another to account for the different possible ways that expansion can be done ( eg. >>^, >^> or ^>>). This leads to a combinatorial explosion that must be dealt with by:

- Doing a Depth-First expansion;
- Only keeping and returning the minimum count of characters that an expansion leads to, not the whole expanded moves list;
- Memoize the results for each pair expansion-level
  A lot of care must be taken to account for the structure of the problem, namely keeping the initial "A"s on the correct place, but i'm satisfied with the result.
  Still had to struggle with the borrow checker, but it made me appreciate it more than anything, given that it pointed out subtle memory leaks that i would certainly let pass if not having its help.

**Input parsing:** The input lines.

**Part one solution:** Described above. Simulate the numeric keypad to get the list of moves that must be performed on the directional keypad, and recursively do a Depth-First expansion of each single move, keeping track of the minimum ones and using a hashmap to memoize intermediary results.

**Part two solution:** Same as part one, with a deeper depth.

## [Day 22](https://adventofcode.com/2024/day/22)

After yesterday, this was a breeze. The description is confusing, but the intent is straightforward. Still tried to optimize the runtime by using bitwise operations instead of arithmetic and tuples, but it's still not enough. Need to take a second look.

**Input parsing:** Input numbers.

**Part one solution:** Some bit manipulations and sum the result.

**Part two solution:** Main issue here is that, for each monkey, we need to store only the first difference that appears, otherwise this would be cleaner.

## [Day 23](https://adventofcode.com/2024/day/23)

Graph theory. Part one is finding all triangle cliques in the graph and part two the maximal clique. For part two i wasn't in the mood to derive an (inefficient) algorithm, so i just looked up on wikipedia and implemented [Bron–Kerbosch](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm).

**Input parsing:** The undirected graph, with both vertexes being keys.

**Part one solution:** To find triangle cliques for a given vertex, for each of its neighbors check if their neighbor's set intersection with the vertex neighbor's set is not empty.

**Part two solution:** [Bron–Kerbosch](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm) with pivoting.

## [Day 24](https://adventofcode.com/2024/day/24)

**Input parsing:**

**Part one solution:**

**Part two solution:**

## [Day 25](https://adventofcode.com/2024/day/25)

And that's a wrap with a simple one, back to the beginning.

**Input parsing:** Separate and parse the locks and keys on the input, returning the frequencies of the relevant character, '#' or '.' for each column

**Part one solution:** Looping through the locks and keys, check if the frequencies of the opposite characters match for each column.

**Part two solution:** N/A
