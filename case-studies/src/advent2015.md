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

## [Day 2](https://adventofcode.com/2015/day/2)

Part 1: Calculate area of wrapping paper needed for a box.

Part 2: Calculate length of ribbon needed for a box.

Specification thoughts for part 1

- The problem statement includes some examples - it is worth checking all of
  those.
- Neither the area nor the length is just the mathematical definition so some
  standard tests don't quite work because standard mathematical laws don't hold.
- There doesn't seem to be any reasonable way to write a specification that is
  not just a repeat of the implementation so we are forced to rely on
  writing properties that do not completely characterize the function.

  - Two zero length sides require zero paper and zero ribbon.
  - Doubling the length of all the sides quadruples the paper needed.
    (The ribbon calculation does not allow an easy check like that.)
  - The order in which the sides are specified does not affect the result.
  - Larger boxes need more paper and more ribbon.

- Do we think that the properties we wrote would catch all likely bugs?

  - Implementations that always return zero would pass the property checks
    (but not the examples).
  - Implementations that use `+` for `*` or `*` for `+` would probably
    not pass the zero-length check and would definitely be caught by
    the examples.
  - Implementations with typos that confuse x, y and z or cut-and-paste
    repetitions would likely be caught by the reordering rules.

  Overall, it's probably not too bad.
