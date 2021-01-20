# [Advent of Code 2015](https://adventofcode.com/2015)

The [Advent of Code](https://adventofcode.com) puzzles all have a little
story associated with them. We will not repeat the stories here but will give
a short summary of the essence of the problem.

## [Day 1](https://adventofcode.com/2015/day/1)

Part 1: Calculate the value of a string where '(' counts as `+1` and ')' counts as `-1`.

Part 2: Find the first location where the value goes negative.

Propverify problems found

- No support for using Arbitrary::any strategy using "x: i32" syntax. Fixed.
- No support for regex string strategies
- No support for using ? the way that proptest does

Specification thoughts for part 1

- Testing tradition might use up, down, none as tests and those might be fairly
  effective at finding the non-corner case bugs.
  Their constrained nature might also make them work well with KLEE - except for
  the unbounded nature of the strings.
- The tests empty, singleton and append completely characterize the behaviour of
  santa and their unconstrained inputs means that they have potential to find
  corner case bugs.
  But, they are also harder for KLEE to run because they are unconstrained.
- The singleton test doesn't give a lot of assurance because the `santa_onechar`
  helper function replicates so much of the structure of `santa` that
  common-mode failure is likely. (The up/down/none tests are better in that
  regard.)
- The filtered check is probably the most satisfying.
  One way to think about it  is as a less efficient
  implementation of `santa`.
  This view is emphasized in the filtered2 variant that creates a separate
  function with (almost) the same signature as `santa`.
- Irritating noise about isize -> usize conversion and use of `unwrap()`
  to handle it in `santa_spec` - slightly worrying to have the reference
  potentially panic.
  (That's from the type system though, not the verification)


Specification thoughts for part 2

- Harder to write an obviously correct but inefficient implementation to use as a specification.
- Relatively easy to write some inexact characterizations about length,
  last character, slice, etc.
- Some trivial but annoying out-by-one errors found because of the way that the
  problem is defined.

