use std::cmp::Ordering;
use {List, Point, Vector2};

#[derive(Clone, Debug)]
pub struct Step {
    l: usize,
    points: List<Point>,
}

impl Step {
    #[must_use]
    pub fn with_vector(&self, vector: Vector2) -> Self {
        let position = self.position();
        Step {
            l: self.l + 1,
            points: self.points.push(position + vector),
        }
    }

    pub fn from_point(pt: Point) -> Self {
        Step {
            points: List::new().push(pt),
            l: 1,
        }
    }

    pub fn last_vector(&self) -> Option<Vector2> {
        let mut pts = self.points();

        let last_point = pts.next()?;
        let prev_last_point = pts.next()?;
        Some(Vector2 {
            x: last_point.x - prev_last_point.x,
            y: last_point.y - prev_last_point.y,
        })
    }

    pub fn len(&self) -> usize {
        self.l
    }

    pub fn points(&self) -> impl Iterator<Item=Point> + '_ {
        self.points.iter()
    }

    pub fn into_points(self) -> Vec<Point> {
        let mut pts: Vec<_> = self.points.iter().collect();
        // the points are stored in reverse order
        pts.reverse();
        pts
    }

    pub fn position(&self) -> Point {
        self.points.iter().next().unwrap()
    }
}

impl Eq for Step {}
impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.position() == other.position()
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        self.len().cmp(&other.len())
    }
}
