// Need to use i32 for this problem as we may dig in the negative x direction
// Trace a path of vec of coords
// Create grid the size of (max x - min x) by (max y) (since we start at 1 and cannot go further up)
// Fill in grid based on squares starting with max x and min x and max y
// For each non-filled square, it is nside if there are # to the left and right, and abov and below
// Count number of #
