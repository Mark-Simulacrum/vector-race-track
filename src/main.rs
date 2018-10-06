extern crate vector_track;

use std::time::Instant;

use vector_track::Point;
use vector_track::Boundaries;

pub fn main() {
    let overall_start = Instant::now();
    for x in 6..=9 {
        let root = Point { x, y: 30 };
        let start = Instant::now();
        let paths = vector_track::compute_final_paths(root, &Boundaries::new());
        eprintln!("took {:?} for x={}; {} paths, with length={:?}",
            start.elapsed(), x, paths.len(), paths.get(0).map(|p| (p.len(), p)));
    }
    eprintln!("total time: {:?}", overall_start.elapsed());
}
