extern crate vector_track;

use std::time::Instant;

use vector_track::Point;

pub fn main() {
    let boundaries = [
        (20, 29),
        (16, 23),
        (16, 19),
        (13, 17),
        (10, 19),
        (10, 31),
        (5, 31), // start
        (5, 20),
        (6, 17),
        (8, 14),
        (12, 12),
        (16, 13),
        (19, 15),
        (21, 22),
        (24, 25),
        (29, 26),
        (31, 30),
        (24, 30), // finish (31, 30) -> (24, 30)
    ];
    for x in 6..=9 {
        let root = Point { x, y: 30 };
        let start = Instant::now();
        let paths = vector_track::compute_final_paths(root, &boundaries);
        eprintln!("took {:?} for x={}; {} paths, with length={:?}",
            start.elapsed(), x, paths.len(), paths.get(0).map(|p| (p.len(), p)));
    }
}
