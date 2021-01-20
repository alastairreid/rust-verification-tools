# Sources of examples

We have several goals in picking examples

- They should resemble "normal" programming problems.
  (To my mind, this rules out a lot of the more mathematical
  exercises like solving Sudoko puzzles, etc. – though this
  is a bit subjective.)

- It is good if there is a little ambiguity in the problem statement.
  We rarely get complete problem statements in real life and
  a lot of real programming involves making decisions about edge cases,
  where to validate inputs, what is an incorrect input, what to do
  when there are multiple 'correct' results, etc.

- Can be implemented in several interestingly different ways.
  If there is only one sensible way of implementing a problem,
  then the specification often looks a lot like the implementation
  and it is not clear whether we are checking anything useful at all.
  It is useful to have some examples like this though because
  it forces us to think harder about how to write a meaningful specification.

- Is not too complicated to solve.
  We want to keep the emphasis on the specification and verification effort,
  not on the implementation.


## Rustlings

[Rustlings](https://github.com/rust-lang/rustlings/blob/main/README.md)
is a collection of small exercises for learning Rust.
However, almost all of the exercises are about learning the syntax,
type system and borrow-checker rather than learning how to write programs
so although these exercises are good for learning Rust, they are
not so useful for specifications and verification.


## [Reddit r/dailyprogrammer](https://www.reddit.com/r/dailyprogrammer/)

### [Progressive taxation](https://www.reddit.com/r/dailyprogrammer/comments/cdieag/20190715_challenge_379_easy_progressive_taxation/)

This is a series of exercises around progressive taxation.

- Implement a hardcoded progressive taxation function that calculates the tax on
  a given income level.
- Read the tax brackets from a file.
- Implement the inverse function: given a tax level (e.g., 32%), find the
  corresponding income level.
  (Hint: progressive taxation has the property that increasing your income never
  decreases the amount of tax you pay or your tax rate.)


## James Baum's beginner exercises

James Baum's [beginner exercises](https://github.com/whostolemyhat/learning-projects)
consists of 29 exercises with short (1-4 sentence descriptions).


## Advent of Code

[Advent of Code](https://adventofcode.com/) consists of 50 exercises [each year
since 2015](https://adventofcode.com/2020/events) that can be solved in any
programming language.

Note that exercises come in pairs with the second exercise only being released when you
solve the first one so we may wish to limit ourselves to only tackling the first
of each pair.

### 2015 Day 1

Propverify problems found

- No support for using Arbitrary::any strategy using "x: i32" syntax. Fixed.
- No support for regex string strategies
- No support for using ? the way that proptest does

Specification thoughts

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


## Programming textbooks

Programming textbooks often contain exercises of increasing complexity.
These may vary in how much the focus on language issues (less interesting) and
how much on solving the problems.

### Kernighan and Ritchie "C Programming Language"



