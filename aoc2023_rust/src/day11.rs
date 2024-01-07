// Keep a vec of true for each column
// Keep an empty vec of u32 for each expanded row index
// Keep an empty vec of coords of galaxies
// For each line
//     if all chars are ., push the index into row
//     For each column with a #, set the column index in the col vec to false
//     Add the (r, c) indices to the coords vec
// Create new vec of u32 for expanded column indices
// For each column that is true, add the index to this expanded column vec
// Create updated galaxy positions vec
// For each galaxy x and y, add 1 for each column that current x is bigger than column index and
// each row that is bigger than a row index
// Take combinations of all galaxies and compute the Manhattan distance between them
