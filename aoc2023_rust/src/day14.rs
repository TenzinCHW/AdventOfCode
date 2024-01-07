// Another initial-final position problem
// For each column, keep track of the row index of the last known obstacle (O, # or wall), start at
// 0
// Iterating over each row
//      set O rocks final pos row index of last known obstacle + 1, increment row index by 1
//      set row index to current row index if it's a #
//  For all Os, sum up their row pos
