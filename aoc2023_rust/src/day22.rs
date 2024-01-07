// Ok a bit more challenging, even if it's only the data input that's hard to understand
// To solve, we have to again create a directed graph
// We need to let the bricks settle first since they're still in the air
// Keep a vec of blocks that have settled
// For each brick, look downwards to see which surface it will land on
// This will be the brick that supports it, create dependency graph

