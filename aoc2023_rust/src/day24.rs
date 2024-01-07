struct Hailstone {
    x: f32,
    y: f32,
    z: f32,
    vx: f32,
    vy: f32,
    vz: f32,
}


struct Point2D {
    x: f32,
    y:f32,
}


fn intersection_2d(h1: &Hailstone, h2:&Hailstone) -> Point2D {
    // Check parallel i.e. (h2.vy / h2.vx) = (h1.vy / h1.vx) and h1.x != h2.x and h1.y != h2.y
    // solve for
    // tx * h1.vx + h1.x = tx * h2.vx + h2.x 
    // => tx * (h1.vx - h2.vx) = h2.x - h1.x
    // => tx = (h2.x - h1.x) / (h1.vx - h2.vx)
    // ty * h1.vy + h1.y = ty * h2.vy + h2.y
    // ty = (h2.y - h1.y) / (h1.vy - h2.vy)
    // if ty == tx, they collide, otherwise no
    // if ty == tx but they're both negative, they collided already
}


fn day24_p1(input_file: &str) {
    // TODO Set up combinations of hailstones
}

