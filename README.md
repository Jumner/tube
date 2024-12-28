# Test Tube Game Solver

So my mom plays these dumb games and one of them seemed interesting to solve. Essentially you have these test tubes with 4 different coloured liquids in them. Your goal is to match them up. The rules are:

1. You can only pour one colour at a time. In other words, you can only pour off the top liquid.
2. You can only pour a colour into an empty tube, or on top of the same colour, assuming it's not full.
3. You start with n full tubes and m empty tubes. The exact configuration can be specified.

# Why Rust

Because I wanted to learn. Plus I like it :)

# How it works

The algorithm is pretty simple. For each game state you find all the legal moves and their corresponding game states. This creates a tree of sorts.

It essentially does a breadth-first search with some caching so it doesn't take forever or get stuck in loops.

# Testing

Simply clone the package and assuming cargo is setup run:

`cargo test`

This will build the project and run all the tests.

First are unit tests which check that the game follows the rules and that all the individual functions are working correctly. You can find these at the bottom of [lib.rs](https://github.com/Jumner/tube/blob/master/src/lib.rs)
Next are tons of games that are manually entered directly from the game. You can find these tests in [main.rs](https://github.com/Jumner/tube/blob/master/src/main.rs).


