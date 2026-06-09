# Aquarium Engine

For a list of commands that a developer uses to debug this package,
look into `Makefile`.

## Code Structure

Most of the core business logic is contained in the
`./Sources/Engine/` directory.

Firstly, there is the `Game` struct. This is instantiated once per
aquarium game, and it can contain many `Instance`s when backtracking
for a solution.

Hence static and pre-processed data is stored in `Game`, and
referenced in `Instance`. This minimizes cost when cloning `Instance`
during recursion.

A `Point` is a `row`-`col` pair struct, and a `PourPoint` denotes a
key point in the aquarium grid at which pouring gives a unique result.
Every game has its own set of `PourPoint`s. (if two points are side by
side and in the same group, only one of them will be in the game's
list of `PourPoint`s.)

`PourPoint`s tell the programmer where water will flow when poured at
a particular `Point` in the grid (and air too). This list of
`PourPoint`s are pre-processed at the game's initialization and used
by instances when trying different fluids at different points.

Forcing moves are determined by trying to pour a fluid at a `PourPoint`.
If the `Instance` remains valid after the pour, then it's not a
forcing move. But if it is immediately invalid, then we are forced to
pour the other liquid.

the `Instance::fastForward` method is a self-mutating function that
executes all forcing moves until either no forcing moves are left, or
the instance becomes invalid.

## On cloning an `Instance`

Wherever possible, try use the `Instance::undo()` method to undo a
fluid pour instead of cloning and re-assigning an `Instance`.

## Up Next

1. Implement backtracking on the `Instance` struct that is
   instantiated in `Game::solve()`
2. Sort `PourPoint`s based on a heuristic to maximise backtracking
   effectiveness
