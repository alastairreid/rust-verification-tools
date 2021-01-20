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

## Programming textbooks

Programming textbooks often contain exercises of increasing complexity.
These may vary in how much the focus on language issues (less interesting) and
how much on solving the problems.

### Kernighan and Ritchie "C Programming Language"



